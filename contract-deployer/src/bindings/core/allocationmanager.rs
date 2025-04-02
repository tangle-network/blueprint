#![allow(clippy::all, clippy::pedantic, clippy::nursery, warnings, unknown_lints, rustdoc::all, elided_lifetimes_in_paths)]
use AllocationManager::*;

///Module containing a contract's types and functions.
/**

```solidity
library IAllocationManagerTypes {
    struct AllocateParams { OperatorSet operatorSet; address[] strategies; uint64[] newMagnitudes; }
    struct Allocation { uint64 currentMagnitude; int128 pendingDiff; uint32 effectBlock; }
    struct CreateSetParams { uint32 operatorSetId; address[] strategies; }
    struct DeregisterParams { address operator; address avs; uint32[] operatorSetIds; }
    struct RegisterParams { address avs; uint32[] operatorSetIds; bytes data; }
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
struct AllocateParams { OperatorSet operatorSet; address[] strategies; uint64[] newMagnitudes; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct AllocateParams {
        #[allow(missing_docs)]
        pub operatorSet: <OperatorSet as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub strategies: alloy::sol_types::private::Vec<
            alloy::sol_types::private::Address,
        >,
        #[allow(missing_docs)]
        pub newMagnitudes: alloy::sol_types::private::Vec<u64>,
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
            OperatorSet,
            alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
            alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<64>>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            <OperatorSet as alloy::sol_types::SolType>::RustType,
            alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
            alloy::sol_types::private::Vec<u64>,
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
        impl ::core::convert::From<AllocateParams> for UnderlyingRustTuple<'_> {
            fn from(value: AllocateParams) -> Self {
                (value.operatorSet, value.strategies, value.newMagnitudes)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for AllocateParams {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    operatorSet: tuple.0,
                    strategies: tuple.1,
                    newMagnitudes: tuple.2,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for AllocateParams {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for AllocateParams {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <OperatorSet as alloy_sol_types::SolType>::tokenize(
                        &self.operatorSet,
                    ),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Address,
                    > as alloy_sol_types::SolType>::tokenize(&self.strategies),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<64>,
                    > as alloy_sol_types::SolType>::tokenize(&self.newMagnitudes),
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
        impl alloy_sol_types::SolType for AllocateParams {
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
        impl alloy_sol_types::SolStruct for AllocateParams {
            const NAME: &'static str = "AllocateParams";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "AllocateParams(OperatorSet operatorSet,address[] strategies,uint64[] newMagnitudes)",
                )
            }
            #[inline]
            fn eip712_components() -> alloy_sol_types::private::Vec<
                alloy_sol_types::private::Cow<'static, str>,
            > {
                let mut components = alloy_sol_types::private::Vec::with_capacity(1);
                components
                    .push(
                        <OperatorSet as alloy_sol_types::SolStruct>::eip712_root_type(),
                    );
                components
                    .extend(
                        <OperatorSet as alloy_sol_types::SolStruct>::eip712_components(),
                    );
                components
            }
            #[inline]
            fn eip712_encode_data(&self) -> alloy_sol_types::private::Vec<u8> {
                [
                    <OperatorSet as alloy_sol_types::SolType>::eip712_data_word(
                            &self.operatorSet,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Address,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.strategies)
                        .0,
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<64>,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.newMagnitudes)
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for AllocateParams {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <OperatorSet as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.operatorSet,
                    )
                    + <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Address,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.strategies,
                    )
                    + <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<64>,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.newMagnitudes,
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
                <OperatorSet as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.operatorSet,
                    out,
                );
                <alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::Address,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.strategies,
                    out,
                );
                <alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::Uint<64>,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.newMagnitudes,
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
struct Allocation { uint64 currentMagnitude; int128 pendingDiff; uint32 effectBlock; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct Allocation {
        #[allow(missing_docs)]
        pub currentMagnitude: u64,
        #[allow(missing_docs)]
        pub pendingDiff: i128,
        #[allow(missing_docs)]
        pub effectBlock: u32,
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
            alloy::sol_types::sol_data::Int<128>,
            alloy::sol_types::sol_data::Uint<32>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (u64, i128, u32);
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<Allocation> for UnderlyingRustTuple<'_> {
            fn from(value: Allocation) -> Self {
                (value.currentMagnitude, value.pendingDiff, value.effectBlock)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for Allocation {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    currentMagnitude: tuple.0,
                    pendingDiff: tuple.1,
                    effectBlock: tuple.2,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for Allocation {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for Allocation {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::tokenize(&self.currentMagnitude),
                    <alloy::sol_types::sol_data::Int<
                        128,
                    > as alloy_sol_types::SolType>::tokenize(&self.pendingDiff),
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.effectBlock),
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
        impl alloy_sol_types::SolType for Allocation {
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
        impl alloy_sol_types::SolStruct for Allocation {
            const NAME: &'static str = "Allocation";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "Allocation(uint64 currentMagnitude,int128 pendingDiff,uint32 effectBlock)",
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
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.currentMagnitude,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Int<
                        128,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.pendingDiff)
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.effectBlock)
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for Allocation {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.currentMagnitude,
                    )
                    + <alloy::sol_types::sol_data::Int<
                        128,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.pendingDiff,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.effectBlock,
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
                    &rust.currentMagnitude,
                    out,
                );
                <alloy::sol_types::sol_data::Int<
                    128,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.pendingDiff,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.effectBlock,
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
struct CreateSetParams { uint32 operatorSetId; address[] strategies; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct CreateSetParams {
        #[allow(missing_docs)]
        pub operatorSetId: u32,
        #[allow(missing_docs)]
        pub strategies: alloy::sol_types::private::Vec<
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
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::Uint<32>,
            alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            u32,
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
        impl ::core::convert::From<CreateSetParams> for UnderlyingRustTuple<'_> {
            fn from(value: CreateSetParams) -> Self {
                (value.operatorSetId, value.strategies)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for CreateSetParams {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    operatorSetId: tuple.0,
                    strategies: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for CreateSetParams {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for CreateSetParams {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.operatorSetId),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Address,
                    > as alloy_sol_types::SolType>::tokenize(&self.strategies),
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
        impl alloy_sol_types::SolType for CreateSetParams {
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
        impl alloy_sol_types::SolStruct for CreateSetParams {
            const NAME: &'static str = "CreateSetParams";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "CreateSetParams(uint32 operatorSetId,address[] strategies)",
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
                        32,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.operatorSetId)
                        .0,
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Address,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.strategies)
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for CreateSetParams {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
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
struct DeregisterParams { address operator; address avs; uint32[] operatorSetIds; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct DeregisterParams {
        #[allow(missing_docs)]
        pub operator: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub avs: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub operatorSetIds: alloy::sol_types::private::Vec<u32>,
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
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<DeregisterParams> for UnderlyingRustTuple<'_> {
            fn from(value: DeregisterParams) -> Self {
                (value.operator, value.avs, value.operatorSetIds)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for DeregisterParams {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    operator: tuple.0,
                    avs: tuple.1,
                    operatorSetIds: tuple.2,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for DeregisterParams {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for DeregisterParams {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
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
        impl alloy_sol_types::SolType for DeregisterParams {
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
        impl alloy_sol_types::SolStruct for DeregisterParams {
            const NAME: &'static str = "DeregisterParams";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "DeregisterParams(address operator,address avs,uint32[] operatorSetIds)",
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
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.avs,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<32>,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.operatorSetIds,
                        )
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for DeregisterParams {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.operator,
                    )
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.avs,
                    )
                    + <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<32>,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.operatorSetIds,
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
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.avs,
                    out,
                );
                <alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::Uint<32>,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.operatorSetIds,
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
struct RegisterParams { address avs; uint32[] operatorSetIds; bytes data; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct RegisterParams {
        #[allow(missing_docs)]
        pub avs: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub operatorSetIds: alloy::sol_types::private::Vec<u32>,
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
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::Address,
            alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<32>>,
            alloy::sol_types::sol_data::Bytes,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::Address,
            alloy::sol_types::private::Vec<u32>,
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
        impl ::core::convert::From<RegisterParams> for UnderlyingRustTuple<'_> {
            fn from(value: RegisterParams) -> Self {
                (value.avs, value.operatorSetIds, value.data)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for RegisterParams {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    avs: tuple.0,
                    operatorSetIds: tuple.1,
                    data: tuple.2,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for RegisterParams {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for RegisterParams {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
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
        impl alloy_sol_types::SolType for RegisterParams {
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
        impl alloy_sol_types::SolStruct for RegisterParams {
            const NAME: &'static str = "RegisterParams";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "RegisterParams(address avs,uint32[] operatorSetIds,bytes data)",
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
                            &self.avs,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<32>,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.operatorSetIds,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::eip712_data_word(
                            &self.data,
                        )
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for RegisterParams {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.avs,
                    )
                    + <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<32>,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.operatorSetIds,
                    )
                    + <alloy::sol_types::sol_data::Bytes as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.data,
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
                    &rust.avs,
                    out,
                );
                <alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::Uint<32>,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.operatorSetIds,
                    out,
                );
                <alloy::sol_types::sol_data::Bytes as alloy_sol_types::EventTopic>::encode_topic_preimage(
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
    struct AllocateParams {
        OperatorSet operatorSet;
        address[] strategies;
        uint64[] newMagnitudes;
    }
    struct Allocation {
        uint64 currentMagnitude;
        int128 pendingDiff;
        uint32 effectBlock;
    }
    struct CreateSetParams {
        uint32 operatorSetId;
        address[] strategies;
    }
    struct DeregisterParams {
        address operator;
        address avs;
        uint32[] operatorSetIds;
    }
    struct RegisterParams {
        address avs;
        uint32[] operatorSetIds;
        bytes data;
    }
    struct SlashingParams {
        address operator;
        uint32 operatorSetId;
        address[] strategies;
        uint256[] wadsToSlash;
        string description;
    }
}

interface AllocationManager {
    struct OperatorSet {
        address avs;
        uint32 id;
    }

    error AlreadyMemberOfSet();
    error CurrentlyPaused();
    error Empty();
    error InputAddressZero();
    error InputArrayLengthMismatch();
    error InsufficientMagnitude();
    error InvalidAVSRegistrar();
    error InvalidCaller();
    error InvalidNewPausedStatus();
    error InvalidOperator();
    error InvalidOperatorSet();
    error InvalidPermissions();
    error InvalidShortString();
    error InvalidSnapshotOrdering();
    error InvalidWadToSlash();
    error ModificationAlreadyPending();
    error NonexistentAVSMetadata();
    error NotMemberOfSet();
    error OnlyPauser();
    error OnlyUnpauser();
    error OperatorNotSlashable();
    error OutOfBounds();
    error SameMagnitude();
    error StrategiesMustBeInAscendingOrder();
    error StrategyAlreadyInOperatorSet();
    error StrategyNotInOperatorSet();
    error StringTooLong(string str);
    error UninitializedAllocationDelay();

    event AVSMetadataURIUpdated(address indexed avs, string metadataURI);
    event AVSRegistrarSet(address avs, address registrar);
    event AllocationDelaySet(address operator, uint32 delay, uint32 effectBlock);
    event AllocationUpdated(address operator, OperatorSet operatorSet, address strategy, uint64 magnitude, uint32 effectBlock);
    event EncumberedMagnitudeUpdated(address operator, address strategy, uint64 encumberedMagnitude);
    event Initialized(uint8 version);
    event MaxMagnitudeUpdated(address operator, address strategy, uint64 maxMagnitude);
    event OperatorAddedToOperatorSet(address indexed operator, OperatorSet operatorSet);
    event OperatorRemovedFromOperatorSet(address indexed operator, OperatorSet operatorSet);
    event OperatorSetCreated(OperatorSet operatorSet);
    event OperatorSlashed(address operator, OperatorSet operatorSet, address[] strategies, uint256[] wadSlashed, string description);
    event OwnershipTransferred(address indexed previousOwner, address indexed newOwner);
    event Paused(address indexed account, uint256 newPausedStatus);
    event StrategyAddedToOperatorSet(OperatorSet operatorSet, address strategy);
    event StrategyRemovedFromOperatorSet(OperatorSet operatorSet, address strategy);
    event Unpaused(address indexed account, uint256 newPausedStatus);

    constructor(address _delegation, address _pauserRegistry, address _permissionController, uint32 _DEALLOCATION_DELAY, uint32 _ALLOCATION_CONFIGURATION_DELAY, string _version);

    function ALLOCATION_CONFIGURATION_DELAY() external view returns (uint32);
    function DEALLOCATION_DELAY() external view returns (uint32);
    function addStrategiesToOperatorSet(address avs, uint32 operatorSetId, address[] memory strategies) external;
    function clearDeallocationQueue(address operator, address[] memory strategies, uint16[] memory numToClear) external;
    function createOperatorSets(address avs, IAllocationManagerTypes.CreateSetParams[] memory params) external;
    function delegation() external view returns (address);
    function deregisterFromOperatorSets(IAllocationManagerTypes.DeregisterParams memory params) external;
    function getAVSRegistrar(address avs) external view returns (address);
    function getAllocatableMagnitude(address operator, address strategy) external view returns (uint64);
    function getAllocatedSets(address operator) external view returns (OperatorSet[] memory);
    function getAllocatedStake(OperatorSet memory operatorSet, address[] memory operators, address[] memory strategies) external view returns (uint256[][] memory);
    function getAllocatedStrategies(address operator, OperatorSet memory operatorSet) external view returns (address[] memory);
    function getAllocation(address operator, OperatorSet memory operatorSet, address strategy) external view returns (IAllocationManagerTypes.Allocation memory);
    function getAllocationDelay(address operator) external view returns (bool, uint32);
    function getAllocations(address[] memory operators, OperatorSet memory operatorSet, address strategy) external view returns (IAllocationManagerTypes.Allocation[] memory);
    function getEncumberedMagnitude(address operator, address strategy) external view returns (uint64);
    function getMaxMagnitude(address operator, address strategy) external view returns (uint64);
    function getMaxMagnitudes(address[] memory operators, address strategy) external view returns (uint64[] memory);
    function getMaxMagnitudes(address operator, address[] memory strategies) external view returns (uint64[] memory);
    function getMaxMagnitudesAtBlock(address operator, address[] memory strategies, uint32 blockNumber) external view returns (uint64[] memory);
    function getMemberCount(OperatorSet memory operatorSet) external view returns (uint256);
    function getMembers(OperatorSet memory operatorSet) external view returns (address[] memory);
    function getMinimumSlashableStake(OperatorSet memory operatorSet, address[] memory operators, address[] memory strategies, uint32 futureBlock) external view returns (uint256[][] memory slashableStake);
    function getOperatorSetCount(address avs) external view returns (uint256);
    function getRegisteredSets(address operator) external view returns (OperatorSet[] memory);
    function getStrategiesInOperatorSet(OperatorSet memory operatorSet) external view returns (address[] memory);
    function getStrategyAllocations(address operator, address strategy) external view returns (OperatorSet[] memory, IAllocationManagerTypes.Allocation[] memory);
    function initialize(address initialOwner, uint256 initialPausedStatus) external;
    function isMemberOfOperatorSet(address operator, OperatorSet memory operatorSet) external view returns (bool);
    function isOperatorSet(OperatorSet memory operatorSet) external view returns (bool);
    function isOperatorSlashable(address operator, OperatorSet memory operatorSet) external view returns (bool);
    function modifyAllocations(address operator, IAllocationManagerTypes.AllocateParams[] memory params) external;
    function owner() external view returns (address);
    function pause(uint256 newPausedStatus) external;
    function pauseAll() external;
    function paused(uint8 index) external view returns (bool);
    function paused() external view returns (uint256);
    function pauserRegistry() external view returns (address);
    function permissionController() external view returns (address);
    function registerForOperatorSets(address operator, IAllocationManagerTypes.RegisterParams memory params) external;
    function removeStrategiesFromOperatorSet(address avs, uint32 operatorSetId, address[] memory strategies) external;
    function renounceOwnership() external;
    function setAVSRegistrar(address avs, address registrar) external;
    function setAllocationDelay(address operator, uint32 delay) external;
    function slashOperator(address avs, IAllocationManagerTypes.SlashingParams memory params) external;
    function transferOwnership(address newOwner) external;
    function unpause(uint256 newPausedStatus) external;
    function updateAVSMetadataURI(address avs, string memory metadataURI) external;
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
        "name": "_delegation",
        "type": "address",
        "internalType": "contract IDelegationManager"
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
        "name": "_DEALLOCATION_DELAY",
        "type": "uint32",
        "internalType": "uint32"
      },
      {
        "name": "_ALLOCATION_CONFIGURATION_DELAY",
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
    "name": "ALLOCATION_CONFIGURATION_DELAY",
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
    "name": "DEALLOCATION_DELAY",
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
    "name": "addStrategiesToOperatorSet",
    "inputs": [
      {
        "name": "avs",
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
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "clearDeallocationQueue",
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
      },
      {
        "name": "numToClear",
        "type": "uint16[]",
        "internalType": "uint16[]"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "createOperatorSets",
    "inputs": [
      {
        "name": "avs",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "params",
        "type": "tuple[]",
        "internalType": "struct IAllocationManagerTypes.CreateSetParams[]",
        "components": [
          {
            "name": "operatorSetId",
            "type": "uint32",
            "internalType": "uint32"
          },
          {
            "name": "strategies",
            "type": "address[]",
            "internalType": "contract IStrategy[]"
          }
        ]
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "delegation",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "contract IDelegationManager"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "deregisterFromOperatorSets",
    "inputs": [
      {
        "name": "params",
        "type": "tuple",
        "internalType": "struct IAllocationManagerTypes.DeregisterParams",
        "components": [
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
        ]
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "getAVSRegistrar",
    "inputs": [
      {
        "name": "avs",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "contract IAVSRegistrar"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getAllocatableMagnitude",
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
        "type": "uint64",
        "internalType": "uint64"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getAllocatedSets",
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
        "type": "tuple[]",
        "internalType": "struct OperatorSet[]",
        "components": [
          {
            "name": "avs",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "id",
            "type": "uint32",
            "internalType": "uint32"
          }
        ]
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getAllocatedStake",
    "inputs": [
      {
        "name": "operatorSet",
        "type": "tuple",
        "internalType": "struct OperatorSet",
        "components": [
          {
            "name": "avs",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "id",
            "type": "uint32",
            "internalType": "uint32"
          }
        ]
      },
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
    "name": "getAllocatedStrategies",
    "inputs": [
      {
        "name": "operator",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "operatorSet",
        "type": "tuple",
        "internalType": "struct OperatorSet",
        "components": [
          {
            "name": "avs",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "id",
            "type": "uint32",
            "internalType": "uint32"
          }
        ]
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "address[]",
        "internalType": "contract IStrategy[]"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getAllocation",
    "inputs": [
      {
        "name": "operator",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "operatorSet",
        "type": "tuple",
        "internalType": "struct OperatorSet",
        "components": [
          {
            "name": "avs",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "id",
            "type": "uint32",
            "internalType": "uint32"
          }
        ]
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
        "type": "tuple",
        "internalType": "struct IAllocationManagerTypes.Allocation",
        "components": [
          {
            "name": "currentMagnitude",
            "type": "uint64",
            "internalType": "uint64"
          },
          {
            "name": "pendingDiff",
            "type": "int128",
            "internalType": "int128"
          },
          {
            "name": "effectBlock",
            "type": "uint32",
            "internalType": "uint32"
          }
        ]
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getAllocationDelay",
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
      },
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
    "name": "getAllocations",
    "inputs": [
      {
        "name": "operators",
        "type": "address[]",
        "internalType": "address[]"
      },
      {
        "name": "operatorSet",
        "type": "tuple",
        "internalType": "struct OperatorSet",
        "components": [
          {
            "name": "avs",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "id",
            "type": "uint32",
            "internalType": "uint32"
          }
        ]
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
        "type": "tuple[]",
        "internalType": "struct IAllocationManagerTypes.Allocation[]",
        "components": [
          {
            "name": "currentMagnitude",
            "type": "uint64",
            "internalType": "uint64"
          },
          {
            "name": "pendingDiff",
            "type": "int128",
            "internalType": "int128"
          },
          {
            "name": "effectBlock",
            "type": "uint32",
            "internalType": "uint32"
          }
        ]
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getEncumberedMagnitude",
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
        "type": "uint64",
        "internalType": "uint64"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getMaxMagnitude",
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
        "type": "uint64",
        "internalType": "uint64"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getMaxMagnitudes",
    "inputs": [
      {
        "name": "operators",
        "type": "address[]",
        "internalType": "address[]"
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
        "type": "uint64[]",
        "internalType": "uint64[]"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getMaxMagnitudes",
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
        "type": "uint64[]",
        "internalType": "uint64[]"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getMaxMagnitudesAtBlock",
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
      },
      {
        "name": "blockNumber",
        "type": "uint32",
        "internalType": "uint32"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "uint64[]",
        "internalType": "uint64[]"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getMemberCount",
    "inputs": [
      {
        "name": "operatorSet",
        "type": "tuple",
        "internalType": "struct OperatorSet",
        "components": [
          {
            "name": "avs",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "id",
            "type": "uint32",
            "internalType": "uint32"
          }
        ]
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
    "name": "getMembers",
    "inputs": [
      {
        "name": "operatorSet",
        "type": "tuple",
        "internalType": "struct OperatorSet",
        "components": [
          {
            "name": "avs",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "id",
            "type": "uint32",
            "internalType": "uint32"
          }
        ]
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
    "name": "getMinimumSlashableStake",
    "inputs": [
      {
        "name": "operatorSet",
        "type": "tuple",
        "internalType": "struct OperatorSet",
        "components": [
          {
            "name": "avs",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "id",
            "type": "uint32",
            "internalType": "uint32"
          }
        ]
      },
      {
        "name": "operators",
        "type": "address[]",
        "internalType": "address[]"
      },
      {
        "name": "strategies",
        "type": "address[]",
        "internalType": "contract IStrategy[]"
      },
      {
        "name": "futureBlock",
        "type": "uint32",
        "internalType": "uint32"
      }
    ],
    "outputs": [
      {
        "name": "slashableStake",
        "type": "uint256[][]",
        "internalType": "uint256[][]"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getOperatorSetCount",
    "inputs": [
      {
        "name": "avs",
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
    "name": "getRegisteredSets",
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
        "type": "tuple[]",
        "internalType": "struct OperatorSet[]",
        "components": [
          {
            "name": "avs",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "id",
            "type": "uint32",
            "internalType": "uint32"
          }
        ]
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getStrategiesInOperatorSet",
    "inputs": [
      {
        "name": "operatorSet",
        "type": "tuple",
        "internalType": "struct OperatorSet",
        "components": [
          {
            "name": "avs",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "id",
            "type": "uint32",
            "internalType": "uint32"
          }
        ]
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "address[]",
        "internalType": "contract IStrategy[]"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getStrategyAllocations",
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
        "type": "tuple[]",
        "internalType": "struct OperatorSet[]",
        "components": [
          {
            "name": "avs",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "id",
            "type": "uint32",
            "internalType": "uint32"
          }
        ]
      },
      {
        "name": "",
        "type": "tuple[]",
        "internalType": "struct IAllocationManagerTypes.Allocation[]",
        "components": [
          {
            "name": "currentMagnitude",
            "type": "uint64",
            "internalType": "uint64"
          },
          {
            "name": "pendingDiff",
            "type": "int128",
            "internalType": "int128"
          },
          {
            "name": "effectBlock",
            "type": "uint32",
            "internalType": "uint32"
          }
        ]
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
    "name": "isMemberOfOperatorSet",
    "inputs": [
      {
        "name": "operator",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "operatorSet",
        "type": "tuple",
        "internalType": "struct OperatorSet",
        "components": [
          {
            "name": "avs",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "id",
            "type": "uint32",
            "internalType": "uint32"
          }
        ]
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
    "name": "isOperatorSet",
    "inputs": [
      {
        "name": "operatorSet",
        "type": "tuple",
        "internalType": "struct OperatorSet",
        "components": [
          {
            "name": "avs",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "id",
            "type": "uint32",
            "internalType": "uint32"
          }
        ]
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
    "name": "isOperatorSlashable",
    "inputs": [
      {
        "name": "operator",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "operatorSet",
        "type": "tuple",
        "internalType": "struct OperatorSet",
        "components": [
          {
            "name": "avs",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "id",
            "type": "uint32",
            "internalType": "uint32"
          }
        ]
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
    "name": "modifyAllocations",
    "inputs": [
      {
        "name": "operator",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "params",
        "type": "tuple[]",
        "internalType": "struct IAllocationManagerTypes.AllocateParams[]",
        "components": [
          {
            "name": "operatorSet",
            "type": "tuple",
            "internalType": "struct OperatorSet",
            "components": [
              {
                "name": "avs",
                "type": "address",
                "internalType": "address"
              },
              {
                "name": "id",
                "type": "uint32",
                "internalType": "uint32"
              }
            ]
          },
          {
            "name": "strategies",
            "type": "address[]",
            "internalType": "contract IStrategy[]"
          },
          {
            "name": "newMagnitudes",
            "type": "uint64[]",
            "internalType": "uint64[]"
          }
        ]
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
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
    "name": "registerForOperatorSets",
    "inputs": [
      {
        "name": "operator",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "params",
        "type": "tuple",
        "internalType": "struct IAllocationManagerTypes.RegisterParams",
        "components": [
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
        ]
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "removeStrategiesFromOperatorSet",
    "inputs": [
      {
        "name": "avs",
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
    "name": "setAVSRegistrar",
    "inputs": [
      {
        "name": "avs",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "registrar",
        "type": "address",
        "internalType": "contract IAVSRegistrar"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "setAllocationDelay",
    "inputs": [
      {
        "name": "operator",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "delay",
        "type": "uint32",
        "internalType": "uint32"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "slashOperator",
    "inputs": [
      {
        "name": "avs",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "params",
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
    "name": "updateAVSMetadataURI",
    "inputs": [
      {
        "name": "avs",
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
    "name": "AVSMetadataURIUpdated",
    "inputs": [
      {
        "name": "avs",
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
    "name": "AVSRegistrarSet",
    "inputs": [
      {
        "name": "avs",
        "type": "address",
        "indexed": false,
        "internalType": "address"
      },
      {
        "name": "registrar",
        "type": "address",
        "indexed": false,
        "internalType": "contract IAVSRegistrar"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "AllocationDelaySet",
    "inputs": [
      {
        "name": "operator",
        "type": "address",
        "indexed": false,
        "internalType": "address"
      },
      {
        "name": "delay",
        "type": "uint32",
        "indexed": false,
        "internalType": "uint32"
      },
      {
        "name": "effectBlock",
        "type": "uint32",
        "indexed": false,
        "internalType": "uint32"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "AllocationUpdated",
    "inputs": [
      {
        "name": "operator",
        "type": "address",
        "indexed": false,
        "internalType": "address"
      },
      {
        "name": "operatorSet",
        "type": "tuple",
        "indexed": false,
        "internalType": "struct OperatorSet",
        "components": [
          {
            "name": "avs",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "id",
            "type": "uint32",
            "internalType": "uint32"
          }
        ]
      },
      {
        "name": "strategy",
        "type": "address",
        "indexed": false,
        "internalType": "contract IStrategy"
      },
      {
        "name": "magnitude",
        "type": "uint64",
        "indexed": false,
        "internalType": "uint64"
      },
      {
        "name": "effectBlock",
        "type": "uint32",
        "indexed": false,
        "internalType": "uint32"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "EncumberedMagnitudeUpdated",
    "inputs": [
      {
        "name": "operator",
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
        "name": "encumberedMagnitude",
        "type": "uint64",
        "indexed": false,
        "internalType": "uint64"
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
    "name": "MaxMagnitudeUpdated",
    "inputs": [
      {
        "name": "operator",
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
        "name": "maxMagnitude",
        "type": "uint64",
        "indexed": false,
        "internalType": "uint64"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "OperatorAddedToOperatorSet",
    "inputs": [
      {
        "name": "operator",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "operatorSet",
        "type": "tuple",
        "indexed": false,
        "internalType": "struct OperatorSet",
        "components": [
          {
            "name": "avs",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "id",
            "type": "uint32",
            "internalType": "uint32"
          }
        ]
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "OperatorRemovedFromOperatorSet",
    "inputs": [
      {
        "name": "operator",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "operatorSet",
        "type": "tuple",
        "indexed": false,
        "internalType": "struct OperatorSet",
        "components": [
          {
            "name": "avs",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "id",
            "type": "uint32",
            "internalType": "uint32"
          }
        ]
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "OperatorSetCreated",
    "inputs": [
      {
        "name": "operatorSet",
        "type": "tuple",
        "indexed": false,
        "internalType": "struct OperatorSet",
        "components": [
          {
            "name": "avs",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "id",
            "type": "uint32",
            "internalType": "uint32"
          }
        ]
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "OperatorSlashed",
    "inputs": [
      {
        "name": "operator",
        "type": "address",
        "indexed": false,
        "internalType": "address"
      },
      {
        "name": "operatorSet",
        "type": "tuple",
        "indexed": false,
        "internalType": "struct OperatorSet",
        "components": [
          {
            "name": "avs",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "id",
            "type": "uint32",
            "internalType": "uint32"
          }
        ]
      },
      {
        "name": "strategies",
        "type": "address[]",
        "indexed": false,
        "internalType": "contract IStrategy[]"
      },
      {
        "name": "wadSlashed",
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
    "name": "StrategyAddedToOperatorSet",
    "inputs": [
      {
        "name": "operatorSet",
        "type": "tuple",
        "indexed": false,
        "internalType": "struct OperatorSet",
        "components": [
          {
            "name": "avs",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "id",
            "type": "uint32",
            "internalType": "uint32"
          }
        ]
      },
      {
        "name": "strategy",
        "type": "address",
        "indexed": false,
        "internalType": "contract IStrategy"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "StrategyRemovedFromOperatorSet",
    "inputs": [
      {
        "name": "operatorSet",
        "type": "tuple",
        "indexed": false,
        "internalType": "struct OperatorSet",
        "components": [
          {
            "name": "avs",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "id",
            "type": "uint32",
            "internalType": "uint32"
          }
        ]
      },
      {
        "name": "strategy",
        "type": "address",
        "indexed": false,
        "internalType": "contract IStrategy"
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
    "name": "AlreadyMemberOfSet",
    "inputs": []
  },
  {
    "type": "error",
    "name": "CurrentlyPaused",
    "inputs": []
  },
  {
    "type": "error",
    "name": "Empty",
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
    "name": "InsufficientMagnitude",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidAVSRegistrar",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidCaller",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidNewPausedStatus",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidOperator",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidOperatorSet",
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
    "name": "InvalidSnapshotOrdering",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidWadToSlash",
    "inputs": []
  },
  {
    "type": "error",
    "name": "ModificationAlreadyPending",
    "inputs": []
  },
  {
    "type": "error",
    "name": "NonexistentAVSMetadata",
    "inputs": []
  },
  {
    "type": "error",
    "name": "NotMemberOfSet",
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
    "name": "OperatorNotSlashable",
    "inputs": []
  },
  {
    "type": "error",
    "name": "OutOfBounds",
    "inputs": []
  },
  {
    "type": "error",
    "name": "SameMagnitude",
    "inputs": []
  },
  {
    "type": "error",
    "name": "StrategiesMustBeInAscendingOrder",
    "inputs": []
  },
  {
    "type": "error",
    "name": "StrategyAlreadyInOperatorSet",
    "inputs": []
  },
  {
    "type": "error",
    "name": "StrategyNotInOperatorSet",
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
    "name": "UninitializedAllocationDelay",
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
pub mod AllocationManager {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /// The creation / init bytecode of the contract.
    ///
    /// ```text
    ///0x610140604052348015610010575f5ffd5b50604051615f8f380380615f8f83398101604081905261002f91610208565b8084878585896001600160a01b03811661005c576040516339b190bb60e11b815260040160405180910390fd5b6001600160a01b0390811660805292831660a05263ffffffff91821660c0521660e052166101005261008d816100a5565b610120525061009a6100eb565b505050505050610368565b5f5f829050601f815111156100d8578260405163305a27a960e01b81526004016100cf9190610310565b60405180910390fd5b80516100e382610342565b179392505050565b5f54610100900460ff16156101525760405162461bcd60e51b815260206004820152602760248201527f496e697469616c697a61626c653a20636f6e747261637420697320696e697469604482015266616c697a696e6760c81b60648201526084016100cf565b5f5460ff908116146101a1575f805460ff191660ff9081179091556040519081527f7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb38474024989060200160405180910390a15b565b6001600160a01b03811681146101b7575f5ffd5b50565b805163ffffffff811681146101cd575f5ffd5b919050565b634e487b7160e01b5f52604160045260245ffd5b5f5b838110156102005781810151838201526020016101e8565b50505f910152565b5f5f5f5f5f5f60c0878903121561021d575f5ffd5b8651610228816101a3565b6020880151909650610239816101a3565b604088015190955061024a816101a3565b9350610258606088016101ba565b9250610266608088016101ba565b60a08801519092506001600160401b03811115610281575f5ffd5b8701601f81018913610291575f5ffd5b80516001600160401b038111156102aa576102aa6101d2565b604051601f8201601f19908116603f011681016001600160401b03811182821017156102d8576102d86101d2565b6040528181528282016020018b10156102ef575f5ffd5b6103008260208301602086016101e6565b8093505050509295509295509295565b602081525f825180602084015261032e8160408501602087016101e6565b601f01601f19169190910160400192915050565b80516020808301519190811015610362575f198160200360031b1b821691505b50919050565b60805160a05160c05160e0516101005161012051615b8c6104035f395f611ad901525f818161043501526134eb01525f81816105990152613d5501525f818161035201528181611f78015261266a01525f81816107180152818161148701528181611b0d01528181611b6f0152818161294d01526135ca01525f81816105c00152818161082d01528181611c21015261316e0152615b8c5ff3fe608060405234801561000f575f5ffd5b50600436106102a2575f3560e01c80636cfb44811161016c578063a9821821116100d5578063c221d8ae1161008f578063c221d8ae146106da578063cd6dc687146106ed578063d3d96ff414610700578063df5cf72314610713578063f2fde38b1461073a578063f605ce081461074d578063fabc1cbc14610760575f5ffd5b8063a98218211461064c578063adc2e3d91461065f578063b2447af714610672578063b66bd98914610685578063b9fbaed114610698578063ba1a84e5146106c7575f5ffd5b8063886f119511610126578063886f1195146105bb5780638ce64854146105e25780638da5cb5b1461060257806394d7d00c14610613578063952899ee14610626578063a9333ec814610639575f5ffd5b80636cfb4481146105285780636e3492b5146105535780636e875dba14610566578063715018a61461057957806379ae50cd146105815780637bc1ef6114610594575f5ffd5b80634177a87c1161020e57806354fd4d50116101c857806354fd4d50146104b057806356c483e6146104c5578063595c6a67146104d85780635ac86ab7146104e05780635c975abb14610503578063670d3ba214610515575f5ffd5b80634177a87c146104105780634657e26a146104305780634a10ffe5146104575780634b5046ef1461047757806350feea201461048a578063547afb871461049d575f5ffd5b80632981eb771161025f5780632981eb771461034d5780632b453a9a146103895780632bab2c4a146103a9578063304c10cd146103bc57806336352057146103dc57806340120dab146103ef575f5ffd5b806310e1b9b8146102a65780631352c3e6146102cf578063136439dd146102f257806315fe502814610307578063260dc75814610327578063261f84e01461033a575b5f5ffd5b6102b96102b43660046149b7565b610773565b6040516102c69190614a29565b60405180910390f35b6102e26102dd366004614a37565b61079b565b60405190151581526020016102c6565b610305610300366004614a6b565b610816565b005b61031a610315366004614a82565b6108f0565b6040516102c69190614b00565b6102e2610335366004614b12565b610a07565b610305610348366004614b6c565b610a38565b6103747f000000000000000000000000000000000000000000000000000000000000000081565b60405163ffffffff90911681526020016102c6565b61039c610397366004614c51565b610d13565b6040516102c69190614cf4565b61039c6103b7366004614d57565b610d29565b6103cf6103ca366004614a82565b610dc8565b6040516102c69190614ddb565b6103056103ea366004614def565b610df7565b6104026103fd366004614e41565b6115db565b6040516102c6929190614ea8565b61042361041e366004614b12565b611746565b6040516102c69190614f05565b6103cf7f000000000000000000000000000000000000000000000000000000000000000081565b61046a610465366004614f17565b61176a565b6040516102c69190614f5a565b610305610485366004614fa5565b611812565b610305610498366004615025565b6118cc565b61046a6104ab366004615083565b611a2a565b6104b8611ad2565b6040516102c691906150c5565b6103056104d3366004615110565b611b02565b610305611c0a565b6102e26104ee36600461513a565b606654600160ff9092169190911b9081161490565b6066545b6040519081526020016102c6565b6102e2610523366004614a37565b611cbe565b61053b610536366004614e41565b611ccf565b6040516001600160401b0390911681526020016102c6565b610305610561366004615170565b611ce4565b610423610574366004614b12565b6120c5565b6103056120d6565b61031a61058f366004614a82565b6120e7565b6103747f000000000000000000000000000000000000000000000000000000000000000081565b6103cf7f000000000000000000000000000000000000000000000000000000000000000081565b6105f56105f03660046151a1565b6121c1565b6040516102c691906151e4565b6033546001600160a01b03166103cf565b61046a6106213660046151f6565b61226d565b610305610634366004615251565b612359565b61053b610647366004614e41565b612805565b61030561065a3660046153fa565b612834565b61030561066d366004615478565b6128e6565b610507610680366004614b12565b612c46565b610305610693366004615025565b612c68565b6106ab6106a6366004614a82565b612dc2565b60408051921515835263ffffffff9091166020830152016102c6565b6105076106d5366004614a82565b612e5a565b6104236106e8366004614a37565b612e7a565b6103056106fb3660046154ba565b612ea3565b61030561070e366004614e41565b612fc0565b6103cf7f000000000000000000000000000000000000000000000000000000000000000081565b610305610748366004614a82565b6130e7565b61053b61075b366004614e41565b613160565b61030561076e366004614a6b565b61316c565b61077b6148a8565b5f61078f8561078986613282565b856132e5565b925050505b9392505050565b6001600160a01b0382165f908152609e602052604081208190816107be85613282565b815260208082019290925260409081015f2081518083019092525460ff8116151580835261010090910463ffffffff169282019290925291508061080c5750806020015163ffffffff164311155b9150505b92915050565b60405163237dfb4760e11b81526001600160a01b037f000000000000000000000000000000000000000000000000000000000000000016906346fbf68e90610862903390600401614ddb565b602060405180830381865afa15801561087d573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906108a191906154e4565b6108be57604051631d77d47760e21b815260040160405180910390fd5b60665481811681146108e35760405163c61dca5d60e01b815260040160405180910390fd5b6108ec82613445565b5050565b6001600160a01b0381165f908152609d602052604081206060919061091490613482565b90505f816001600160401b0381111561092f5761092f6148db565b60405190808252806020026020018201604052801561097357816020015b604080518082019091525f808252602082015281526020019060019003908161094d5790505b5090505f5b828110156109ff576001600160a01b0385165f908152609d602052604090206109da906109a5908361348b565b604080518082019091525f80825260208201525060408051808201909152606082901c815263ffffffff909116602082015290565b8282815181106109ec576109ec615503565b6020908102919091010152600101610978565b509392505050565b60208082015182516001600160a01b03165f9081526098909252604082206108109163ffffffff9081169061349616565b82610a42816134ad565b610a5f5760405163932d94f760e01b815260040160405180910390fd5b6001600160a01b0384165f90815260a4602052604090205460ff16610a97576040516348f7dbb960e01b815260040160405180910390fd5b5f5b82811015610d0c575f6040518060400160405280876001600160a01b03168152602001868685818110610ace57610ace615503565b9050602002810190610ae09190615517565b610aee906020810190615535565b63ffffffff168152509050610b38816020015163ffffffff1660985f896001600160a01b03166001600160a01b031681526020019081526020015f2061355790919063ffffffff16565b610b5557604051631fb1705560e21b815260040160405180910390fd5b7f31629285ead2335ae0933f86ed2ae63321f7af77b4e6eaabc42c057880977e6c6040518060400160405280886001600160a01b03168152602001836020015163ffffffff16815250604051610bab919061554e565b60405180910390a15f610bbd82613282565b90505f5b868685818110610bd357610bd3615503565b9050602002810190610be59190615517565b610bf390602081019061555c565b9050811015610d0157610c69878786818110610c1157610c11615503565b9050602002810190610c239190615517565b610c3190602081019061555c565b83818110610c4157610c41615503565b9050602002016020810190610c569190614a82565b5f84815260996020526040902090613562565b507f7ab260fe0af193db5f4986770d831bda4ea46099dc817e8b6716dcae8af8e88b83888887818110610c9e57610c9e615503565b9050602002810190610cb09190615517565b610cbe90602081019061555c565b84818110610cce57610cce615503565b9050602002016020810190610ce39190614a82565b604051610cf19291906155a1565b60405180910390a1600101610bc1565b505050600101610a99565b5050505050565b6060610d2184848443613576565b949350505050565b6060610d3785858585613576565b90505f5b8451811015610dbf57610d67858281518110610d5957610d59615503565b60200260200101518761079b565b610db7575f5b8451811015610db5575f838381518110610d8957610d89615503565b60200260200101518281518110610da257610da2615503565b6020908102919091010152600101610d6d565b505b600101610d3b565b50949350505050565b6001600160a01b038082165f908152609760205260408120549091168015610df05780610794565b5090919050565b606654600190600290811603610e205760405163840a48d560e01b815260040160405180910390fd5b82610e2a816134ad565b610e475760405163932d94f760e01b815260040160405180910390fd5b5f6040518060400160405280866001600160a01b03168152602001856020016020810190610e759190615535565b63ffffffff1690529050610e8c606085018561555c565b9050610e9b604086018661555c565b905014610ebb576040516343714afd60e01b815260040160405180910390fd5b60208082015182516001600160a01b03165f90815260989092526040909120610eed9163ffffffff9081169061349616565b610f0a57604051631fb1705560e21b815260040160405180910390fd5b610f20610f1a6020860186614a82565b8261079b565b610f3d5760405163ebbff49760e01b815260040160405180910390fd5b5f610f4b604086018661555c565b90506001600160401b03811115610f6457610f646148db565b604051908082528060200260200182016040528015610f8d578160200160208202803683370190505b5090505f5b610f9f604087018761555c565b905081101561156d578015806110325750610fbd604087018761555c565b610fc86001846155db565b818110610fd757610fd7615503565b9050602002016020810190610fec9190614a82565b6001600160a01b0316611002604088018861555c565b8381811061101257611012615503565b90506020020160208101906110279190614a82565b6001600160a01b0316115b61104f57604051639f1c805360e01b815260040160405180910390fd5b61105c606087018761555c565b8281811061106c5761106c615503565b905060200201355f1080156110ac5750670de0b6b3a7640000611092606088018861555c565b838181106110a2576110a2615503565b9050602002013511155b6110c957604051631353603160e01b815260040160405180910390fd5b6111256110d9604088018861555c565b838181106110e9576110e9615503565b90506020020160208101906110fe9190614a82565b60995f61110a87613282565b81526020019081526020015f2061386390919063ffffffff16565b611142576040516331bc342760e11b815260040160405180910390fd5b5f8061119461115460208a018a614a82565b61115d87613282565b61116a60408c018c61555c565b8781811061117a5761117a615503565b905060200201602081019061118f9190614a82565b6132e5565b805191935091506001600160401b03165f036111b1575050611565565b5f6111ec6111c260608b018b61555c565b868181106111d2576111d2615503565b85516001600160401b031692602090910201359050613884565b83519091506112076001600160401b0380841690831661389a565b86868151811061121957611219615503565b60200260200101818152505081835f0181815161123691906155ee565b6001600160401b03169052508351829085906112539083906155ee565b6001600160401b03169052506020840180518391906112739083906155ee565b6001600160401b031690525060208301515f600f9190910b1215611378575f6112d66112a260608d018d61555c565b888181106112b2576112b2615503565b9050602002013585602001516112c79061560d565b6001600160801b031690613884565b9050806001600160401b0316846020018181516112f3919061562e565b600f0b9052505f516020615b375f395f51905f5261131460208d018d614a82565b8961132260408f018f61555c565b8a81811061133257611332615503565b90506020020160208101906113479190614a82565b611358885f015189602001516138ae565b886040015160405161136e95949392919061565b565b60405180910390a1505b6113ca61138860208c018c614a82565b61139189613282565b61139e60408e018e61555c565b898181106113ae576113ae615503565b90506020020160208101906113c39190614a82565b87876138cd565b5f516020615b375f395f51905f526113e560208c018c614a82565b886113f360408e018e61555c565b8981811061140357611403615503565b90506020020160208101906114189190614a82565b865160405161142c9493929190439061565b565b60405180910390a161147d61144460208c018c614a82565b61145160408d018d61555c565b8881811061146157611461615503565b90506020020160208101906114769190614a82565b8651613b01565b6001600160a01b037f00000000000000000000000000000000000000000000000000000000000000001663601bb36f6114b960208d018d614a82565b6114c660408e018e61555c565b898181106114d6576114d6615503565b90506020020160208101906114eb9190614a82565b875160405160e085901b6001600160e01b03191681526001600160a01b0393841660048201529290911660248301526001600160401b0380861660448401521660648201526084015f604051808303815f87803b15801561154a575f5ffd5b505af115801561155c573d5f5f3e3d5ffd5b50505050505050505b600101610f92565b507f80969ad29428d6797ee7aad084f9e4a42a82fc506dcd2ca3b6fb431f85ccebe561159c6020870187614a82565b836115aa604089018961555c565b856115b860808c018c6156ac565b6040516115cb9796959493929190615716565b60405180910390a1505050505050565b6001600160a01b0382165f908152609d60205260408120606091829161160090613482565b90505f816001600160401b0381111561161b5761161b6148db565b60405190808252806020026020018201604052801561165f57816020015b604080518082019091525f80825260208201528152602001906001900390816116395790505b5090505f826001600160401b0381111561167b5761167b6148db565b6040519080825280602002602001820160405280156116b457816020015b6116a16148a8565b8152602001906001900390816116995790505b5090505f5b83811015611737576001600160a01b0388165f908152609d602052604081206116e6906109a5908461348b565b9050808483815181106116fb576116fb615503565b602002602001018190525061171189828a610773565b83838151811061172357611723615503565b6020908102919091010152506001016116b9565b509093509150505b9250929050565b60605f61079460995f61175886613282565b81526020019081526020015f20613b63565b60605f83516001600160401b03811115611786576117866148db565b6040519080825280602002602001820160405280156117af578160200160208202803683370190505b5090505f5b84518110156109ff576117e08582815181106117d2576117d2615503565b602002602001015185612805565b8282815181106117f2576117f2615503565b6001600160401b03909216602092830291909101909101526001016117b4565b6066545f9060019081160361183a5760405163840a48d560e01b815260040160405180910390fd5b83821461185a576040516343714afd60e01b815260040160405180910390fd5b5f5b848110156118c3576118bb8787878481811061187a5761187a615503565b905060200201602081019061188f9190614a82565b8686858181106118a1576118a1615503565b90506020020160208101906118b691906157ac565b613b6f565b60010161185c565b50505050505050565b836118d6816134ad565b6118f35760405163932d94f760e01b815260040160405180910390fd5b604080518082019091526001600160a01b038616815263ffffffff851660208201525f61191f82613282565b9050611960826020015163ffffffff1660985f8a6001600160a01b03166001600160a01b031681526020019081526020015f2061349690919063ffffffff16565b61197d57604051631fb1705560e21b815260040160405180910390fd5b5f5b84811015611a205761199c868683818110610c4157610c41615503565b6119b95760405163585cfb2f60e01b815260040160405180910390fd5b7f7ab260fe0af193db5f4986770d831bda4ea46099dc817e8b6716dcae8af8e88b838787848181106119ed576119ed615503565b9050602002016020810190611a029190614a82565b604051611a109291906155a1565b60405180910390a160010161197f565b5050505050505050565b60605f82516001600160401b03811115611a4657611a466148db565b604051908082528060200260200182016040528015611a6f578160200160208202803683370190505b5090505f5b83518110156109ff57611aa085858381518110611a9357611a93615503565b6020026020010151612805565b828281518110611ab257611ab2615503565b6001600160401b0390921660209283029190910190910152600101611a74565b6060611afd7f0000000000000000000000000000000000000000000000000000000000000000613c73565b905090565b336001600160a01b037f00000000000000000000000000000000000000000000000000000000000000001614611c0057611b3b826134ad565b611b58576040516348f5c3ed60e01b815260040160405180910390fd5b6040516336b87bd760e11b81526001600160a01b037f00000000000000000000000000000000000000000000000000000000000000001690636d70f7ae90611ba4908590600401614ddb565b602060405180830381865afa158015611bbf573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611be391906154e4565b611c005760405163ccea9e6f60e01b815260040160405180910390fd5b6108ec8282613cb0565b60405163237dfb4760e11b81526001600160a01b037f000000000000000000000000000000000000000000000000000000000000000016906346fbf68e90611c56903390600401614ddb565b602060405180830381865afa158015611c71573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611c9591906154e4565b611cb257604051631d77d47760e21b815260040160405180910390fd5b611cbc5f19613445565b565b5f61079483609a5f61110a86613282565b5f5f611cdb8484613e52565b95945050505050565b606654600290600490811603611d0d5760405163840a48d560e01b815260040160405180910390fd5b611d22611d1d6020840184614a82565b6134ad565b80611d3b5750611d3b611d1d6040840160208501614a82565b611d58576040516348f5c3ed60e01b815260040160405180910390fd5b5f5b611d67604084018461555c565b9050811015612029575f6040518060400160405280856020016020810190611d8f9190614a82565b6001600160a01b03168152602001611daa604087018761555c565b85818110611dba57611dba615503565b9050602002016020810190611dcf9190615535565b63ffffffff168152509050611e1c816020015163ffffffff1660985f876020016020810190611dfe9190614a82565b6001600160a01b0316815260208101919091526040015f2090613496565b611e3957604051631fb1705560e21b815260040160405180910390fd5b609e5f611e496020870187614a82565b6001600160a01b03166001600160a01b031681526020019081526020015f205f611e7283613282565b815260208101919091526040015f205460ff16611ea2576040516325131d4f60e01b815260040160405180910390fd5b611edc611eae82613282565b609c5f611ebe6020890189614a82565b6001600160a01b0316815260208101919091526040015f2090613fc1565b50611f14611eed6020860186614a82565b609a5f611ef985613282565b81526020019081526020015f20613fcc90919063ffffffff16565b50611f226020850185614a82565b6001600160a01b03167fad34c3070be1dffbcaa499d000ba2b8d9848aefcac3059df245dd95c4ece14fe82604051611f5a919061554e565b60405180910390a2604080518082019091525f815260208101611f9d7f0000000000000000000000000000000000000000000000000000000000000000436157cd565b63ffffffff169052609e5f611fb56020880188614a82565b6001600160a01b03166001600160a01b031681526020019081526020015f205f611fde84613282565b81526020808201929092526040015f2082518154939092015163ffffffff166101000264ffffffff00199215159290921664ffffffffff199093169290921717905550600101611d5a565b5061203d6103ca6040840160208501614a82565b6001600160a01b031663303ca9566120586020850185614a82565b6120686040860160208701614a82565b612075604087018761555c565b6040518563ffffffff1660e01b81526004016120949493929190615822565b5f604051808303815f87803b1580156120ab575f5ffd5b505af11580156120bd573d5f5f3e3d5ffd5b505050505050565b6060610810609a5f61175885613282565b6120de613fe0565b611cbc5f61403a565b6001600160a01b0381165f908152609c602052604081206060919061210b90613482565b90505f816001600160401b03811115612126576121266148db565b60405190808252806020026020018201604052801561216a57816020015b604080518082019091525f80825260208201528152602001906001900390816121445790505b5090505f5b828110156109ff576001600160a01b0385165f908152609c6020526040902061219c906109a5908361348b565b8282815181106121ae576121ae615503565b602090810291909101015260010161216f565b60605f84516001600160401b038111156121dd576121dd6148db565b60405190808252806020026020018201604052801561221657816020015b6122036148a8565b8152602001906001900390816121fb5790505b5090505f5b8551811015610dbf5761224886828151811061223957612239615503565b60200260200101518686610773565b82828151811061225a5761225a615503565b602090810291909101015260010161221b565b60605f83516001600160401b03811115612289576122896148db565b6040519080825280602002602001820160405280156122b2578160200160208202803683370190505b5090505f5b8451811015610dbf576001600160a01b0386165f90815260a1602052604081208651612327928792918990869081106122f2576122f2615503565b60200260200101516001600160a01b03166001600160a01b031681526020019081526020015f2061408b90919063ffffffff16565b82828151811061233957612339615503565b6001600160401b03909216602092830291909101909101526001016122b7565b6066545f906001908116036123815760405163840a48d560e01b815260040160405180910390fd5b61238a836134ad565b6123a7576040516348f5c3ed60e01b815260040160405180910390fd5b5f5f5f6123b386612dc2565b91509150816123d55760405163fa55fc8160e01b815260040160405180910390fd5b91505f90505b8351811015610d0c578381815181106123f6576123f6615503565b6020026020010151604001515184828151811061241557612415615503565b6020026020010151602001515114612440576040516343714afd60e01b815260040160405180910390fd5b5f84828151811061245357612453615503565b602090810291909101810151518082015181516001600160a01b03165f908152609890935260409092209092506124939163ffffffff9081169061349616565b6124b057604051631fb1705560e21b815260040160405180910390fd5b5f6124bb878361079b565b90505f5b8684815181106124d1576124d1615503565b602002602001015160200151518110156127fa575f8785815181106124f8576124f8615503565b602002602001015160200151828151811061251557612515615503565b6020026020010151905061252c898261ffff613b6f565b5f5f61253b8b61078988613282565b91509150806040015163ffffffff165f1461256957604051630d8fcbe360e41b815260040160405180910390fd5b5f6125768785848961409f565b90506125bb825f01518c8a8151811061259157612591615503565b60200260200101516040015187815181106125ae576125ae615503565b60200260200101516140d5565b600f0b602083018190525f036125e457604051634606179360e11b815260040160405180910390fd5b5f8260200151600f0b12156127285780156126aa5761266561260588613282565b6001600160a01b03808f165f90815260a360209081526040808320938a16835292905220908154600160801b90819004600f0b5f818152600180860160205260409091209390935583546001600160801b03908116939091011602179055565b61268f7f0000000000000000000000000000000000000000000000000000000000000000436157cd565b61269a9060016157cd565b63ffffffff166040830152612795565b6126bc836020015183602001516138ae565b6001600160401b031660208401528a518b90899081106126de576126de615503565b60200260200101516040015185815181106126fb576126fb615503565b6020908102919091018101516001600160401b031683525f9083015263ffffffff43166040830152612795565b5f8260200151600f0b131561279557612749836020015183602001516138ae565b6001600160401b03908116602085018190528451909116101561277f57604051636c9be0bf60e01b815260040160405180910390fd5b61278989436157cd565b63ffffffff1660408301525b6127aa8c6127a289613282565b8686866138cd565b5f516020615b375f395f51905f528c88866127cc865f015187602001516138ae565b86604001516040516127e295949392919061565b565b60405180910390a15050600190920191506124bf9050565b5050506001016123db565b6001600160a01b038083165f90815260a1602090815260408083209385168352929052908120610794906140ec565b8261283e816134ad565b61285b5760405163932d94f760e01b815260040160405180910390fd5b6001600160a01b0384165f90815260a4602052604090205460ff1661289d576001600160a01b0384165f90815260a460205260409020805460ff191660011790555b836001600160a01b03167fa89c1dc243d8908a96dd84944bcc97d6bc6ac00dd78e20621576be6a3c94371384846040516128d892919061584e565b60405180910390a250505050565b60665460029060049081160361290f5760405163840a48d560e01b815260040160405180910390fd5b82612919816134ad565b6129365760405163932d94f760e01b815260040160405180910390fd5b6040516336b87bd760e11b81526001600160a01b037f00000000000000000000000000000000000000000000000000000000000000001690636d70f7ae90612982908790600401614ddb565b602060405180830381865afa15801561299d573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906129c191906154e4565b6129de5760405163ccea9e6f60e01b815260040160405180910390fd5b5f5b6129ed602085018561555c565b9050811015612bb557604080518082019091525f9080612a106020880188614a82565b6001600160a01b03168152602001868060200190612a2e919061555c565b85818110612a3e57612a3e615503565b9050602002016020810190612a539190615535565b63ffffffff90811690915260208083015183516001600160a01b03165f90815260989092526040909120929350612a8f92919081169061349616565b612aac57604051631fb1705560e21b815260040160405180910390fd5b612ab6868261079b565b15612ad457604051636c6c6e2760e11b815260040160405180910390fd5b612afd612ae082613282565b6001600160a01b0388165f908152609c6020526040902090613557565b50612b2986609a5f612b0e85613282565b81526020019081526020015f2061356290919063ffffffff16565b50856001600160a01b03167f43232edf9071753d2321e5fa7e018363ee248e5f2142e6c08edd3265bfb4895e82604051612b63919061554e565b60405180910390a26001600160a01b0386165f908152609e60205260408120600191612b8e84613282565b815260208101919091526040015f20805460ff1916911515919091179055506001016129e0565b50612bc66103ca6020850185614a82565b6001600160a01b031663c63fd50285612be26020870187614a82565b612bef602088018861555c565b612bfc60408a018a6156ac565b6040518763ffffffff1660e01b8152600401612c1d96959493929190615861565b5f604051808303815f87803b158015612c34575f5ffd5b505af1158015611a20573d5f5f3e3d5ffd5b5f610810609a5f612c5685613282565b81526020019081526020015f20613482565b83612c72816134ad565b612c8f5760405163932d94f760e01b815260040160405180910390fd5b6040805180820182526001600160a01b03871680825263ffffffff80881660208085018290525f93845260989052939091209192612cce929161349616565b612ceb57604051631fb1705560e21b815260040160405180910390fd5b5f612cf582613282565b90505f5b84811015611a2057612d3e868683818110612d1657612d16615503565b9050602002016020810190612d2b9190614a82565b5f84815260996020526040902090613fcc565b612d5b576040516331bc342760e11b815260040160405180910390fd5b7f7b4b073d80dcac55a11177d8459ad9f664ceeb91f71f27167bb14f8152a7eeee83878784818110612d8f57612d8f615503565b9050602002016020810190612da49190614a82565b604051612db29291906155a1565b60405180910390a1600101612cf9565b6001600160a01b0381165f908152609b602090815260408083208151608081018352905463ffffffff80821680845260ff600160201b8404161515958401869052600160281b8304821694840194909452600160481b909104166060820181905284939192919015801590612e415750826060015163ffffffff164310155b15612e50575050604081015160015b9590945092505050565b6001600160a01b0381165f90815260986020526040812061081090613482565b6001600160a01b0382165f908152609f602052604081206060919061080c908261175886613282565b5f54610100900460ff1615808015612ec157505f54600160ff909116105b80612eda5750303b158015612eda57505f5460ff166001145b612f425760405162461bcd60e51b815260206004820152602e60248201527f496e697469616c697a61626c653a20636f6e747261637420697320616c72656160448201526d191e481a5b9a5d1a585b1a5e995960921b60648201526084015b60405180910390fd5b5f805460ff191660011790558015612f63575f805461ff0019166101001790555b612f6c82613445565b612f758361403a565b8015612fbb575f805461ff0019169055604051600181527f7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb3847402498906020015b60405180910390a15b505050565b81612fca816134ad565b612fe75760405163932d94f760e01b815260040160405180910390fd5b60405163b526578760e01b81526001600160a01b0383169063b526578790613013908690600401614ddb565b602060405180830381865afa15801561302e573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061305291906154e4565b61306f57604051631d0b13c160e31b815260040160405180910390fd5b6001600160a01b038381165f90815260976020526040902080546001600160a01b0319169184169190911790557f2ae945c40c44dc0ec263f95609c3fdc6952e0aefa22d6374e44f2c997acedf85836130c781610dc8565b604080516001600160a01b03938416815292909116602083015201612fb2565b6130ef613fe0565b6001600160a01b0381166131545760405162461bcd60e51b815260206004820152602660248201527f4f776e61626c653a206e6577206f776e657220697320746865207a65726f206160448201526564647265737360d01b6064820152608401612f39565b61315d8161403a565b50565b5f5f610dbf8484613e52565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031663eab66d7a6040518163ffffffff1660e01b8152600401602060405180830381865afa1580156131c8573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906131ec91906158ad565b6001600160a01b0316336001600160a01b03161461321d5760405163794821ff60e01b815260040160405180910390fd5b606654801982198116146132445760405163c61dca5d60e01b815260040160405180910390fd5b606682905560405182815233907f3582d1828e26bf56bd801502bc021ac0bc8afb57c826e4986b45593c8fad389c9060200160405180910390a25050565b5f815f0151826020015163ffffffff166040516020016132cd92919060609290921b6bffffffffffffffffffffffff1916825260a01b6001600160a01b031916601482015260200190565b604051602081830303815290604052610810906158c8565b604080518082019091525f80825260208201526133006148a8565b6040805180820182526001600160a01b038088165f90815260a160209081528482209288168252919091529182208190613339906140ec565b6001600160401b0390811682526001600160a01b038981165f81815260a260209081526040808320948c168084529482528083205486169682019690965291815260a082528481208b8252825284812092815291815290839020835160608101855290549283168152600160401b8304600f0b91810191909152600160c01b90910463ffffffff169181018290529192504310156133db57909250905061343d565b6133ec815f015182602001516138ae565b6001600160401b0316815260208101515f600f9190910b121561342a5761341b826020015182602001516138ae565b6001600160401b031660208301525b5f60408201819052602082015290925090505b935093915050565b606681905560405181815233907fab40a374bc51de372200a8bc981af8c9ecdc08dfdaef0bb6e09f88f3c616ef3d9060200160405180910390a250565b5f610810825490565b5f61079483836140ff565b5f8181526001830160205260408120541515610794565b604051631beb2b9760e31b81526001600160a01b0382811660048301523360248301523060448301525f80356001600160e01b0319166064840152917f00000000000000000000000000000000000000000000000000000000000000009091169063df595cb8906084016020604051808303815f875af1158015613533573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061081091906154e4565b5f6107948383614125565b5f610794836001600160a01b038416614125565b606083516001600160401b03811115613591576135916148db565b6040519080825280602002602001820160405280156135c457816020015b60608152602001906001900390816135af5790505b5090505f7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031663f0e0e67686866040518363ffffffff1660e01b81526004016136169291906158eb565b5f60405180830381865afa158015613630573d5f5f3e3d5ffd5b505050506040513d5f823e601f3d908101601f19168201604052613657919081019061590f565b90505f5b8551811015613859575f86828151811061367757613677615503565b6020026020010151905085516001600160401b0381111561369a5761369a6148db565b6040519080825280602002602001820160405280156136c3578160200160208202803683370190505b508483815181106136d6576136d6615503565b60209081029190910101525f5b865181101561384f575f8782815181106136ff576136ff615503565b6020908102919091018101516001600160a01b038086165f90815260a184526040808220928416825291909352822090925061373a906140ec565b9050806001600160401b03165f03613753575050613847565b5f61375f858d85610773565b90508863ffffffff16816040015163ffffffff161115801561378757505f8160200151600f0b125b156137a95761379d815f015182602001516138ae565b6001600160401b031681525b80515f906137c4906001600160401b0390811690851661389a565b905061380b818989815181106137dc576137dc615503565b602002602001015187815181106137f5576137f5615503565b602002602001015161417190919063ffffffff16565b89888151811061381d5761381d615503565b6020026020010151868151811061383657613836615503565b602002602001018181525050505050505b6001016136e3565b505060010161365b565b5050949350505050565b6001600160a01b0381165f9081526001830160205260408120541515610794565b5f6107948383670de0b6b3a76400006001614185565b5f61079483670de0b6b3a7640000846141de565b5f6107946138c5836001600160401b03861661562e565b600f0b6142c3565b6020808301516001600160a01b038088165f90815260a284526040808220928816825291909352909120546001600160401b0390811691161461398f57602082810180516001600160a01b038881165f90815260a28552604080822092891682529190945292839020805467ffffffffffffffff19166001600160401b039092169190911790555190517facf9095feb3a370c9cf692421c69ef320d4db5c66e6a7d29c7694eb02364fc55916139869188918791615a1b565b60405180910390a15b6001600160a01b038086165f90815260a060209081526040808320888452825280832093871683529281529082902083518154928501519385015163ffffffff16600160c01b0263ffffffff60c01b196001600160801b038616600160401b026001600160c01b03199095166001600160401b03909316929092179390931716919091179055600f0b15613a71576001600160a01b0385165f908152609f602090815260408083208784529091529020613a499084613562565b506001600160a01b0385165f908152609d60205260409020613a6b9085613557565b50610d0c565b80516001600160401b03165f03610d0c576001600160a01b0385165f908152609f602090815260408083208784529091529020613aae9084613fcc565b506001600160a01b0385165f908152609f602090815260408083208784529091529020613ada90613482565b5f03610d0c576001600160a01b0385165f908152609d602052604090206120bd9085613fc1565b6001600160a01b038084165f90815260a160209081526040808320938616835292905220613b3090438361432e565b7f1c6458079a41077d003c11faf9bf097e693bd67979e4e6500bac7b29db779b5c838383604051612fb293929190615a1b565b60605f61079483614342565b6001600160a01b038381165f90815260a360209081526040808320938616835292905290812054600f81810b600160801b909204900b035b5f81118015613bb957508261ffff1682105b15610d0c576001600160a01b038086165f90815260a3602090815260408083209388168352929052908120613bed9061439b565b90505f5f613bfc8884896132e5565b91509150806040015163ffffffff16431015613c1a57505050610d0c565b613c2788848985856138cd565b6001600160a01b038089165f90815260a360209081526040808320938b16835292905220613c54906143ed565b50613c5e85615a47565b9450613c6984615a5f565b9350505050613ba7565b60605f613c7f83614461565b6040805160208082528183019092529192505f91906020820181803683375050509182525060208101929092525090565b6001600160a01b0382165f908152609b60209081526040918290208251608081018452905463ffffffff808216835260ff600160201b830416151593830193909352600160281b8104831693820193909352600160481b909204166060820181905215801590613d2a5750806060015163ffffffff164310155b15613d4457604081015163ffffffff168152600160208201525b63ffffffff82166040820152613d7a7f0000000000000000000000000000000000000000000000000000000000000000436157cd565b613d859060016157cd565b63ffffffff90811660608381019182526001600160a01b0386165f818152609b602090815260409182902087518154838a0151858b01519851928a1664ffffffffff1990921691909117600160201b91151591909102176cffffffffffffffff00000000001916600160281b9789169790970263ffffffff60481b191696909617600160481b968816968702179055815192835294871694820194909452928301919091527f4e85751d6331506c6c62335f207eb31f12a61e570f34f5c17640308785c6d4db9101612fb2565b6001600160a01b038281165f81815260a2602090815260408083209486168084529482528083205493835260a38252808320948352939052918220546001600160401b039091169190600f81810b600160801b909204900b03815b81811015613f7d576001600160a01b038087165f90815260a3602090815260408083209389168352929052908120613ee59083614488565b6001600160a01b038881165f90815260a0602090815260408083208584528252808320938b16835292815290829020825160608101845290546001600160401b0381168252600160401b8104600f0b92820192909252600160c01b90910463ffffffff16918101829052919250431015613f60575050613f7d565b613f6e8682602001516138ae565b95505050806001019050613ead565b506001600160a01b038086165f90815260a1602090815260408083209388168352929052208390613fad906140ec565b613fb791906155ee565b9150509250929050565b5f61079483836144f7565b5f610794836001600160a01b0384166144f7565b6033546001600160a01b03163314611cbc5760405162461bcd60e51b815260206004820181905260248201527f4f776e61626c653a2063616c6c6572206973206e6f7420746865206f776e65726044820152606401612f39565b603380546001600160a01b038381166001600160a01b0319831681179093556040519116919082907f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e0905f90a35050565b5f6107948383670de0b6b3a76400006145da565b5f6140b08460995f61110a89613282565b80156140b95750815b8015611cdb57505090516001600160401b031615159392505050565b5f6107946001600160401b03808516908416615a74565b5f61081082670de0b6b3a764000061462f565b5f825f01828154811061411457614114615503565b905f5260205f200154905092915050565b5f81815260018301602052604081205461416a57508154600181810184555f848152602080822090930184905584548482528286019093526040902091909155610810565b505f610810565b5f6107948383670de0b6b3a76400006141de565b5f5f6141928686866141de565b905060018360028111156141a8576141a8615aa1565b1480156141c457505f84806141bf576141bf615ab5565b868809115b15611cdb576141d4600182615ac9565b9695505050505050565b5f80805f19858709858702925082811083820303915050805f036142155783828161420b5761420b615ab5565b0492505050610794565b80841161425c5760405162461bcd60e51b81526020600482015260156024820152744d6174683a206d756c446976206f766572666c6f7760581b6044820152606401612f39565b5f8486880960026001871981018816978890046003810283188082028403028082028403028082028403028082028403028082028403029081029092039091025f889003889004909101858311909403939093029303949094049190911702949350505050565b5f6001600160401b0382111561432a5760405162461bcd60e51b815260206004820152602660248201527f53616665436173743a2076616c756520646f65736e27742066697420696e203660448201526534206269747360d01b6064820152608401612f39565b5090565b612fbb83836001600160401b038416614666565b6060815f0180548060200260200160405190810160405280929190818152602001828054801561438f57602002820191905f5260205f20905b81548152602001906001019080831161437b575b50505050509050919050565b5f6143b58254600f81810b600160801b909204900b131590565b156143d357604051631ed9509560e11b815260040160405180910390fd5b508054600f0b5f9081526001909101602052604090205490565b5f6144078254600f81810b600160801b909204900b131590565b1561442557604051631ed9509560e11b815260040160405180910390fd5b508054600f0b5f818152600180840160205260408220805492905583546001600160801b03191692016001600160801b03169190911790915590565b5f60ff8216601f81111561081057604051632cd44ac360e21b815260040160405180910390fd5b5f5f6144aa61449684614769565b85546144a59190600f0b615adc565b6147d2565b8454909150600160801b9004600f90810b9082900b126144dd57604051632d0483c560e21b815260040160405180910390fd5b600f0b5f9081526001939093016020525050604090205490565b5f81815260018301602052604081205480156145d1575f6145196001836155db565b85549091505f9061452c906001906155db565b905081811461458b575f865f01828154811061454a5761454a615503565b905f5260205f200154905080875f01848154811061456a5761456a615503565b5f918252602080832090910192909255918252600188019052604090208390555b855486908061459c5761459c615b03565b600190038181905f5260205f20015f90559055856001015f8681526020019081526020015f205f905560019350505050610810565b5f915050610810565b82545f90816145eb8686838561483b565b905080156146255761460f866146026001846155db565b5f91825260209091200190565b54600160201b90046001600160e01b031661078f565b5091949350505050565b81545f90801561465e57614648846146026001846155db565b54600160201b90046001600160e01b031661080c565b509092915050565b8254801561471c575f61467e856146026001856155db565b60408051808201909152905463ffffffff808216808452600160201b9092046001600160e01b0316602084015291925090851610156146d05760405163151b8e3f60e11b815260040160405180910390fd5b805163ffffffff80861691160361471a57826146f1866146026001866155db565b80546001600160e01b0392909216600160201b0263ffffffff9092169190911790555050505050565b505b506040805180820190915263ffffffff92831681526001600160e01b03918216602080830191825285546001810187555f968752952091519051909216600160201b029190921617910155565b5f6001600160ff1b0382111561432a5760405162461bcd60e51b815260206004820152602860248201527f53616665436173743a2076616c756520646f65736e27742066697420696e2061604482015267371034b73a191a9b60c11b6064820152608401612f39565b80600f81900b81146148365760405162461bcd60e51b815260206004820152602760248201527f53616665436173743a2076616c756520646f65736e27742066697420696e20316044820152663238206269747360c81b6064820152608401612f39565b919050565b5f5b818310156109ff575f614850848461488e565b5f8781526020902090915063ffffffff86169082015463ffffffff16111561487a57809250614888565b614885816001615ac9565b93505b5061483d565b5f61489c6002848418615b17565b61079490848416615ac9565b604080516060810182525f808252602082018190529181019190915290565b6001600160a01b038116811461315d575f5ffd5b634e487b7160e01b5f52604160045260245ffd5b604051606081016001600160401b0381118282101715614911576149116148db565b60405290565b604051601f8201601f191681016001600160401b038111828210171561493f5761493f6148db565b604052919050565b803563ffffffff81168114614836575f5ffd5b5f6040828403121561496a575f5ffd5b604080519081016001600160401b038111828210171561498c5761498c6148db565b604052905080823561499d816148c7565b81526149ab60208401614947565b60208201525092915050565b5f5f5f608084860312156149c9575f5ffd5b83356149d4816148c7565b92506149e3856020860161495a565b915060608401356149f3816148c7565b809150509250925092565b80516001600160401b03168252602080820151600f0b9083015260409081015163ffffffff16910152565b6060810161081082846149fe565b5f5f60608385031215614a48575f5ffd5b8235614a53816148c7565b9150614a62846020850161495a565b90509250929050565b5f60208284031215614a7b575f5ffd5b5035919050565b5f60208284031215614a92575f5ffd5b8135610794816148c7565b80516001600160a01b0316825260209081015163ffffffff16910152565b5f8151808452602084019350602083015f5b82811015614af657614ae0868351614a9d565b6040959095019460209190910190600101614acd565b5093949350505050565b602081525f6107946020830184614abb565b5f60408284031215614b22575f5ffd5b610794838361495a565b5f5f83601f840112614b3c575f5ffd5b5081356001600160401b03811115614b52575f5ffd5b6020830191508360208260051b850101111561173f575f5ffd5b5f5f5f60408486031215614b7e575f5ffd5b8335614b89816148c7565b925060208401356001600160401b03811115614ba3575f5ffd5b614baf86828701614b2c565b9497909650939450505050565b5f6001600160401b03821115614bd457614bd46148db565b5060051b60200190565b5f82601f830112614bed575f5ffd5b8135614c00614bfb82614bbc565b614917565b8082825260208201915060208360051b860101925085831115614c21575f5ffd5b602085015b83811015614c47578035614c39816148c7565b835260209283019201614c26565b5095945050505050565b5f5f5f60808486031215614c63575f5ffd5b614c6d858561495a565b925060408401356001600160401b03811115614c87575f5ffd5b614c9386828701614bde565b92505060608401356001600160401b03811115614cae575f5ffd5b614cba86828701614bde565b9150509250925092565b5f8151808452602084019350602083015f5b82811015614af6578151865260209586019590910190600101614cd6565b5f602082016020835280845180835260408501915060408160051b8601019250602086015f5b82811015614d4b57603f19878603018452614d36858351614cc4565b94506020938401939190910190600101614d1a565b50929695505050505050565b5f5f5f5f60a08587031215614d6a575f5ffd5b614d74868661495a565b935060408501356001600160401b03811115614d8e575f5ffd5b614d9a87828801614bde565b93505060608501356001600160401b03811115614db5575f5ffd5b614dc187828801614bde565b925050614dd060808601614947565b905092959194509250565b6001600160a01b0391909116815260200190565b5f5f60408385031215614e00575f5ffd5b8235614e0b816148c7565b915060208301356001600160401b03811115614e25575f5ffd5b830160a08186031215614e36575f5ffd5b809150509250929050565b5f5f60408385031215614e52575f5ffd5b8235614e5d816148c7565b91506020830135614e36816148c7565b5f8151808452602084019350602083015f5b82811015614af657614e928683516149fe565b6060959095019460209190910190600101614e7f565b604081525f614eba6040830185614abb565b8281036020840152611cdb8185614e6d565b5f8151808452602084019350602083015f5b82811015614af65781516001600160a01b0316865260209586019590910190600101614ede565b602081525f6107946020830184614ecc565b5f5f60408385031215614f28575f5ffd5b82356001600160401b03811115614f3d575f5ffd5b614f4985828601614bde565b9250506020830135614e36816148c7565b602080825282518282018190525f918401906040840190835b81811015614f9a5783516001600160401b0316835260209384019390920191600101614f73565b509095945050505050565b5f5f5f5f5f60608688031215614fb9575f5ffd5b8535614fc4816148c7565b945060208601356001600160401b03811115614fde575f5ffd5b614fea88828901614b2c565b90955093505060408601356001600160401b03811115615008575f5ffd5b61501488828901614b2c565b969995985093965092949392505050565b5f5f5f5f60608587031215615038575f5ffd5b8435615043816148c7565b935061505160208601614947565b925060408501356001600160401b0381111561506b575f5ffd5b61507787828801614b2c565b95989497509550505050565b5f5f60408385031215615094575f5ffd5b823561509f816148c7565b915060208301356001600160401b038111156150b9575f5ffd5b613fb785828601614bde565b602081525f82518060208401525f5b818110156150f157602081860181015160408684010152016150d4565b505f604082850101526040601f19601f83011684010191505092915050565b5f5f60408385031215615121575f5ffd5b823561512c816148c7565b9150614a6260208401614947565b5f6020828403121561514a575f5ffd5b813560ff81168114610794575f5ffd5b5f6060828403121561516a575f5ffd5b50919050565b5f60208284031215615180575f5ffd5b81356001600160401b03811115615195575f5ffd5b61080c8482850161515a565b5f5f5f608084860312156151b3575f5ffd5b83356001600160401b038111156151c8575f5ffd5b6151d486828701614bde565b9350506149e3856020860161495a565b602081525f6107946020830184614e6d565b5f5f5f60608486031215615208575f5ffd5b8335615213816148c7565b925060208401356001600160401b0381111561522d575f5ffd5b61523986828701614bde565b92505061524860408501614947565b90509250925092565b5f5f60408385031215615262575f5ffd5b823561526d816148c7565b915060208301356001600160401b03811115615287575f5ffd5b8301601f81018513615297575f5ffd5b80356152a5614bfb82614bbc565b8082825260208201915060208360051b8501019250878311156152c6575f5ffd5b602084015b838110156153eb5780356001600160401b038111156152e8575f5ffd5b85016080818b03601f190112156152fd575f5ffd5b6153056148ef565b6153128b6020840161495a565b815260608201356001600160401b0381111561532c575f5ffd5b61533b8c602083860101614bde565b60208301525060808201356001600160401b03811115615359575f5ffd5b6020818401019250508a601f830112615370575f5ffd5b813561537e614bfb82614bbc565b8082825260208201915060208360051b86010192508d83111561539f575f5ffd5b6020850194505b828510156153d55784356001600160401b03811681146153c4575f5ffd5b8252602094850194909101906153a6565b60408401525050845250602092830192016152cb565b50809450505050509250929050565b5f5f5f6040848603121561540c575f5ffd5b8335615417816148c7565b925060208401356001600160401b03811115615431575f5ffd5b8401601f81018613615441575f5ffd5b80356001600160401b03811115615456575f5ffd5b866020828401011115615467575f5ffd5b939660209190910195509293505050565b5f5f60408385031215615489575f5ffd5b8235615494816148c7565b915060208301356001600160401b038111156154ae575f5ffd5b613fb78582860161515a565b5f5f604083850312156154cb575f5ffd5b82356154d6816148c7565b946020939093013593505050565b5f602082840312156154f4575f5ffd5b81518015158114610794575f5ffd5b634e487b7160e01b5f52603260045260245ffd5b5f8235603e1983360301811261552b575f5ffd5b9190910192915050565b5f60208284031215615545575f5ffd5b61079482614947565b604081016108108284614a9d565b5f5f8335601e19843603018112615571575f5ffd5b8301803591506001600160401b0382111561558a575f5ffd5b6020019150600581901b360382131561173f575f5ffd5b606081016155af8285614a9d565b6001600160a01b039290921660409190910152919050565b634e487b7160e01b5f52601160045260245ffd5b81810381811115610810576108106155c7565b6001600160401b038281168282160390811115610810576108106155c7565b5f600f82900b6001607f1b8101615626576156266155c7565b5f0392915050565b600f81810b9083900b0160016001607f1b03811360016001607f1b031982121715610810576108106155c7565b6001600160a01b038616815260c081016156786020830187614a9d565b6001600160a01b039490941660608201526001600160401b0392909216608083015263ffffffff1660a09091015292915050565b5f5f8335601e198436030181126156c1575f5ffd5b8301803591506001600160401b038211156156da575f5ffd5b60200191503681900382131561173f575f5ffd5b81835281816020850137505f828201602090810191909152601f909101601f19169091010190565b6001600160a01b03881681525f60c08201615734602084018a614a9d565b60c060608401528690528660e083015f5b88811015615775578235615758816148c7565b6001600160a01b0316825260209283019290910190600101615745565b5083810360808501526157888188614cc4565b91505082810360a084015261579e8185876156ee565b9a9950505050505050505050565b5f602082840312156157bc575f5ffd5b813561ffff81168114610794575f5ffd5b63ffffffff8181168382160190811115610810576108106155c7565b8183526020830192505f815f5b84811015614af65763ffffffff61580c83614947565b16865260209586019591909101906001016157f6565b6001600160a01b038581168252841660208201526060604082018190525f906141d490830184866157e9565b602081525f610d216020830184866156ee565b6001600160a01b038781168252861660208201526080604082018190525f9061588d90830186886157e9565b82810360608401526158a08185876156ee565b9998505050505050505050565b5f602082840312156158bd575f5ffd5b8151610794816148c7565b8051602080830151919081101561516a575f1960209190910360031b1b16919050565b604081525f6158fd6040830185614ecc565b8281036020840152611cdb8185614ecc565b5f6020828403121561591f575f5ffd5b81516001600160401b03811115615934575f5ffd5b8201601f81018413615944575f5ffd5b8051615952614bfb82614bbc565b8082825260208201915060208360051b850101925086831115615973575f5ffd5b602084015b83811015615a105780516001600160401b03811115615995575f5ffd5b8501603f810189136159a5575f5ffd5b60208101516159b6614bfb82614bbc565b808282526020820191506020808460051b8601010192508b8311156159d9575f5ffd5b6040840193505b828410156159fb5783518252602093840193909101906159e0565b86525050602093840193919091019050615978565b509695505050505050565b6001600160a01b0393841681529190921660208201526001600160401b03909116604082015260600190565b5f60018201615a5857615a586155c7565b5060010190565b5f81615a6d57615a6d6155c7565b505f190190565b600f82810b9082900b0360016001607f1b0319811260016001607f1b0382131715610810576108106155c7565b634e487b7160e01b5f52602160045260245ffd5b634e487b7160e01b5f52601260045260245ffd5b80820180821115610810576108106155c7565b8082018281125f831280158216821582161715615afb57615afb6155c7565b505092915050565b634e487b7160e01b5f52603160045260245ffd5b5f82615b3157634e487b7160e01b5f52601260045260245ffd5b50049056fe1487af5418c47ee5ea45ef4a93398668120890774a9e13487e61e9dc3baf76dda264697066735822122064a2cb6e41f7f8bd29297c0eef600bae9afeb2ec41d7d1f5544aca9fe17d469964736f6c634300081b0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"a\x01@`@R4\x80\x15a\0\x10W__\xFD[P`@Qa_\x8F8\x03\x80a_\x8F\x839\x81\x01`@\x81\x90Ra\0/\x91a\x02\x08V[\x80\x84\x87\x85\x85\x89`\x01`\x01`\xA0\x1B\x03\x81\x16a\0\\W`@Qc9\xB1\x90\xBB`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x90\x81\x16`\x80R\x92\x83\x16`\xA0Rc\xFF\xFF\xFF\xFF\x91\x82\x16`\xC0R\x16`\xE0R\x16a\x01\0Ra\0\x8D\x81a\0\xA5V[a\x01 RPa\0\x9Aa\0\xEBV[PPPPPPa\x03hV[__\x82\x90P`\x1F\x81Q\x11\x15a\0\xD8W\x82`@Qc0Z'\xA9`\xE0\x1B\x81R`\x04\x01a\0\xCF\x91\x90a\x03\x10V[`@Q\x80\x91\x03\x90\xFD[\x80Qa\0\xE3\x82a\x03BV[\x17\x93\x92PPPV[_Ta\x01\0\x90\x04`\xFF\x16\x15a\x01RW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`'`$\x82\x01R\x7FInitializable: contract is initi`D\x82\x01Rfalizing`\xC8\x1B`d\x82\x01R`\x84\x01a\0\xCFV[_T`\xFF\x90\x81\x16\x14a\x01\xA1W_\x80T`\xFF\x19\x16`\xFF\x90\x81\x17\x90\x91U`@Q\x90\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1[V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x01\xB7W__\xFD[PV[\x80Qc\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x01\xCDW__\xFD[\x91\x90PV[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[_[\x83\x81\x10\x15a\x02\0W\x81\x81\x01Q\x83\x82\x01R` \x01a\x01\xE8V[PP_\x91\x01RV[______`\xC0\x87\x89\x03\x12\x15a\x02\x1DW__\xFD[\x86Qa\x02(\x81a\x01\xA3V[` \x88\x01Q\x90\x96Pa\x029\x81a\x01\xA3V[`@\x88\x01Q\x90\x95Pa\x02J\x81a\x01\xA3V[\x93Pa\x02X``\x88\x01a\x01\xBAV[\x92Pa\x02f`\x80\x88\x01a\x01\xBAV[`\xA0\x88\x01Q\x90\x92P`\x01`\x01`@\x1B\x03\x81\x11\x15a\x02\x81W__\xFD[\x87\x01`\x1F\x81\x01\x89\x13a\x02\x91W__\xFD[\x80Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x02\xAAWa\x02\xAAa\x01\xD2V[`@Q`\x1F\x82\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x02\xD8Wa\x02\xD8a\x01\xD2V[`@R\x81\x81R\x82\x82\x01` \x01\x8B\x10\x15a\x02\xEFW__\xFD[a\x03\0\x82` \x83\x01` \x86\x01a\x01\xE6V[\x80\x93PPPP\x92\x95P\x92\x95P\x92\x95V[` \x81R_\x82Q\x80` \x84\x01Ra\x03.\x81`@\x85\x01` \x87\x01a\x01\xE6V[`\x1F\x01`\x1F\x19\x16\x91\x90\x91\x01`@\x01\x92\x91PPV[\x80Q` \x80\x83\x01Q\x91\x90\x81\x10\x15a\x03bW_\x19\x81` \x03`\x03\x1B\x1B\x82\x16\x91P[P\x91\x90PV[`\x80Q`\xA0Q`\xC0Q`\xE0Qa\x01\0Qa\x01 Qa[\x8Ca\x04\x03_9_a\x1A\xD9\x01R_\x81\x81a\x045\x01Ra4\xEB\x01R_\x81\x81a\x05\x99\x01Ra=U\x01R_\x81\x81a\x03R\x01R\x81\x81a\x1Fx\x01Ra&j\x01R_\x81\x81a\x07\x18\x01R\x81\x81a\x14\x87\x01R\x81\x81a\x1B\r\x01R\x81\x81a\x1Bo\x01R\x81\x81a)M\x01Ra5\xCA\x01R_\x81\x81a\x05\xC0\x01R\x81\x81a\x08-\x01R\x81\x81a\x1C!\x01Ra1n\x01Ra[\x8C_\xF3\xFE`\x80`@R4\x80\x15a\0\x0FW__\xFD[P`\x046\x10a\x02\xA2W_5`\xE0\x1C\x80cl\xFBD\x81\x11a\x01lW\x80c\xA9\x82\x18!\x11a\0\xD5W\x80c\xC2!\xD8\xAE\x11a\0\x8FW\x80c\xC2!\xD8\xAE\x14a\x06\xDAW\x80c\xCDm\xC6\x87\x14a\x06\xEDW\x80c\xD3\xD9o\xF4\x14a\x07\0W\x80c\xDF\\\xF7#\x14a\x07\x13W\x80c\xF2\xFD\xE3\x8B\x14a\x07:W\x80c\xF6\x05\xCE\x08\x14a\x07MW\x80c\xFA\xBC\x1C\xBC\x14a\x07`W__\xFD[\x80c\xA9\x82\x18!\x14a\x06LW\x80c\xAD\xC2\xE3\xD9\x14a\x06_W\x80c\xB2Dz\xF7\x14a\x06rW\x80c\xB6k\xD9\x89\x14a\x06\x85W\x80c\xB9\xFB\xAE\xD1\x14a\x06\x98W\x80c\xBA\x1A\x84\xE5\x14a\x06\xC7W__\xFD[\x80c\x88o\x11\x95\x11a\x01&W\x80c\x88o\x11\x95\x14a\x05\xBBW\x80c\x8C\xE6HT\x14a\x05\xE2W\x80c\x8D\xA5\xCB[\x14a\x06\x02W\x80c\x94\xD7\xD0\x0C\x14a\x06\x13W\x80c\x95(\x99\xEE\x14a\x06&W\x80c\xA93>\xC8\x14a\x069W__\xFD[\x80cl\xFBD\x81\x14a\x05(W\x80cn4\x92\xB5\x14a\x05SW\x80cn\x87]\xBA\x14a\x05fW\x80cqP\x18\xA6\x14a\x05yW\x80cy\xAEP\xCD\x14a\x05\x81W\x80c{\xC1\xEFa\x14a\x05\x94W__\xFD[\x80cAw\xA8|\x11a\x02\x0EW\x80cT\xFDMP\x11a\x01\xC8W\x80cT\xFDMP\x14a\x04\xB0W\x80cV\xC4\x83\xE6\x14a\x04\xC5W\x80cY\\jg\x14a\x04\xD8W\x80cZ\xC8j\xB7\x14a\x04\xE0W\x80c\\\x97Z\xBB\x14a\x05\x03W\x80cg\r;\xA2\x14a\x05\x15W__\xFD[\x80cAw\xA8|\x14a\x04\x10W\x80cFW\xE2j\x14a\x040W\x80cJ\x10\xFF\xE5\x14a\x04WW\x80cKPF\xEF\x14a\x04wW\x80cP\xFE\xEA \x14a\x04\x8AW\x80cTz\xFB\x87\x14a\x04\x9DW__\xFD[\x80c)\x81\xEBw\x11a\x02_W\x80c)\x81\xEBw\x14a\x03MW\x80c+E:\x9A\x14a\x03\x89W\x80c+\xAB,J\x14a\x03\xA9W\x80c0L\x10\xCD\x14a\x03\xBCW\x80c65 W\x14a\x03\xDCW\x80c@\x12\r\xAB\x14a\x03\xEFW__\xFD[\x80c\x10\xE1\xB9\xB8\x14a\x02\xA6W\x80c\x13R\xC3\xE6\x14a\x02\xCFW\x80c\x13d9\xDD\x14a\x02\xF2W\x80c\x15\xFEP(\x14a\x03\x07W\x80c&\r\xC7X\x14a\x03'W\x80c&\x1F\x84\xE0\x14a\x03:W[__\xFD[a\x02\xB9a\x02\xB46`\x04aI\xB7V[a\x07sV[`@Qa\x02\xC6\x91\x90aJ)V[`@Q\x80\x91\x03\x90\xF3[a\x02\xE2a\x02\xDD6`\x04aJ7V[a\x07\x9BV[`@Q\x90\x15\x15\x81R` \x01a\x02\xC6V[a\x03\x05a\x03\x006`\x04aJkV[a\x08\x16V[\0[a\x03\x1Aa\x03\x156`\x04aJ\x82V[a\x08\xF0V[`@Qa\x02\xC6\x91\x90aK\0V[a\x02\xE2a\x0356`\x04aK\x12V[a\n\x07V[a\x03\x05a\x03H6`\x04aKlV[a\n8V[a\x03t\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Qc\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x02\xC6V[a\x03\x9Ca\x03\x976`\x04aLQV[a\r\x13V[`@Qa\x02\xC6\x91\x90aL\xF4V[a\x03\x9Ca\x03\xB76`\x04aMWV[a\r)V[a\x03\xCFa\x03\xCA6`\x04aJ\x82V[a\r\xC8V[`@Qa\x02\xC6\x91\x90aM\xDBV[a\x03\x05a\x03\xEA6`\x04aM\xEFV[a\r\xF7V[a\x04\x02a\x03\xFD6`\x04aNAV[a\x15\xDBV[`@Qa\x02\xC6\x92\x91\x90aN\xA8V[a\x04#a\x04\x1E6`\x04aK\x12V[a\x17FV[`@Qa\x02\xC6\x91\x90aO\x05V[a\x03\xCF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x04ja\x04e6`\x04aO\x17V[a\x17jV[`@Qa\x02\xC6\x91\x90aOZV[a\x03\x05a\x04\x856`\x04aO\xA5V[a\x18\x12V[a\x03\x05a\x04\x986`\x04aP%V[a\x18\xCCV[a\x04ja\x04\xAB6`\x04aP\x83V[a\x1A*V[a\x04\xB8a\x1A\xD2V[`@Qa\x02\xC6\x91\x90aP\xC5V[a\x03\x05a\x04\xD36`\x04aQ\x10V[a\x1B\x02V[a\x03\x05a\x1C\nV[a\x02\xE2a\x04\xEE6`\x04aQ:V[`fT`\x01`\xFF\x90\x92\x16\x91\x90\x91\x1B\x90\x81\x16\x14\x90V[`fT[`@Q\x90\x81R` \x01a\x02\xC6V[a\x02\xE2a\x05#6`\x04aJ7V[a\x1C\xBEV[a\x05;a\x0566`\x04aNAV[a\x1C\xCFV[`@Q`\x01`\x01`@\x1B\x03\x90\x91\x16\x81R` \x01a\x02\xC6V[a\x03\x05a\x05a6`\x04aQpV[a\x1C\xE4V[a\x04#a\x05t6`\x04aK\x12V[a \xC5V[a\x03\x05a \xD6V[a\x03\x1Aa\x05\x8F6`\x04aJ\x82V[a \xE7V[a\x03t\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x03\xCF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x05\xF5a\x05\xF06`\x04aQ\xA1V[a!\xC1V[`@Qa\x02\xC6\x91\x90aQ\xE4V[`3T`\x01`\x01`\xA0\x1B\x03\x16a\x03\xCFV[a\x04ja\x06!6`\x04aQ\xF6V[a\"mV[a\x03\x05a\x0646`\x04aRQV[a#YV[a\x05;a\x06G6`\x04aNAV[a(\x05V[a\x03\x05a\x06Z6`\x04aS\xFAV[a(4V[a\x03\x05a\x06m6`\x04aTxV[a(\xE6V[a\x05\x07a\x06\x806`\x04aK\x12V[a,FV[a\x03\x05a\x06\x936`\x04aP%V[a,hV[a\x06\xABa\x06\xA66`\x04aJ\x82V[a-\xC2V[`@\x80Q\x92\x15\x15\x83Rc\xFF\xFF\xFF\xFF\x90\x91\x16` \x83\x01R\x01a\x02\xC6V[a\x05\x07a\x06\xD56`\x04aJ\x82V[a.ZV[a\x04#a\x06\xE86`\x04aJ7V[a.zV[a\x03\x05a\x06\xFB6`\x04aT\xBAV[a.\xA3V[a\x03\x05a\x07\x0E6`\x04aNAV[a/\xC0V[a\x03\xCF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x03\x05a\x07H6`\x04aJ\x82V[a0\xE7V[a\x05;a\x07[6`\x04aNAV[a1`V[a\x03\x05a\x07n6`\x04aJkV[a1lV[a\x07{aH\xA8V[_a\x07\x8F\x85a\x07\x89\x86a2\x82V[\x85a2\xE5V[\x92PPP[\x93\x92PPPV[`\x01`\x01`\xA0\x1B\x03\x82\x16_\x90\x81R`\x9E` R`@\x81 \x81\x90\x81a\x07\xBE\x85a2\x82V[\x81R` \x80\x82\x01\x92\x90\x92R`@\x90\x81\x01_ \x81Q\x80\x83\x01\x90\x92RT`\xFF\x81\x16\x15\x15\x80\x83Ra\x01\0\x90\x91\x04c\xFF\xFF\xFF\xFF\x16\x92\x82\x01\x92\x90\x92R\x91P\x80a\x08\x0CWP\x80` \x01Qc\xFF\xFF\xFF\xFF\x16C\x11\x15[\x91PP[\x92\x91PPV[`@Qc#}\xFBG`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90cF\xFB\xF6\x8E\x90a\x08b\x903\x90`\x04\x01aM\xDBV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x08}W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x08\xA1\x91\x90aT\xE4V[a\x08\xBEW`@Qc\x1Dw\xD4w`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`fT\x81\x81\x16\x81\x14a\x08\xE3W`@Qc\xC6\x1D\xCA]`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x08\xEC\x82a4EV[PPV[`\x01`\x01`\xA0\x1B\x03\x81\x16_\x90\x81R`\x9D` R`@\x81 ``\x91\x90a\t\x14\x90a4\x82V[\x90P_\x81`\x01`\x01`@\x1B\x03\x81\x11\x15a\t/Wa\t/aH\xDBV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\tsW\x81` \x01[`@\x80Q\x80\x82\x01\x90\x91R_\x80\x82R` \x82\x01R\x81R` \x01\x90`\x01\x90\x03\x90\x81a\tMW\x90P[P\x90P_[\x82\x81\x10\x15a\t\xFFW`\x01`\x01`\xA0\x1B\x03\x85\x16_\x90\x81R`\x9D` R`@\x90 a\t\xDA\x90a\t\xA5\x90\x83a4\x8BV[`@\x80Q\x80\x82\x01\x90\x91R_\x80\x82R` \x82\x01RP`@\x80Q\x80\x82\x01\x90\x91R``\x82\x90\x1C\x81Rc\xFF\xFF\xFF\xFF\x90\x91\x16` \x82\x01R\x90V[\x82\x82\x81Q\x81\x10a\t\xECWa\t\xECaU\x03V[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a\txV[P\x93\x92PPPV[` \x80\x82\x01Q\x82Q`\x01`\x01`\xA0\x1B\x03\x16_\x90\x81R`\x98\x90\x92R`@\x82 a\x08\x10\x91c\xFF\xFF\xFF\xFF\x90\x81\x16\x90a4\x96\x16V[\x82a\nB\x81a4\xADV[a\n_W`@Qc\x93-\x94\xF7`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x84\x16_\x90\x81R`\xA4` R`@\x90 T`\xFF\x16a\n\x97W`@QcH\xF7\xDB\xB9`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_[\x82\x81\x10\x15a\r\x0CW_`@Q\x80`@\x01`@R\x80\x87`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x86\x86\x85\x81\x81\x10a\n\xCEWa\n\xCEaU\x03V[\x90P` \x02\x81\x01\x90a\n\xE0\x91\x90aU\x17V[a\n\xEE\x90` \x81\x01\x90aU5V[c\xFF\xFF\xFF\xFF\x16\x81RP\x90Pa\x0B8\x81` \x01Qc\xFF\xFF\xFF\xFF\x16`\x98_\x89`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x90\x81R` \x01_ a5W\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[a\x0BUW`@Qc\x1F\xB1pU`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x7F1b\x92\x85\xEA\xD23Z\xE0\x93?\x86\xED*\xE63!\xF7\xAFw\xB4\xE6\xEA\xAB\xC4,\x05x\x80\x97~l`@Q\x80`@\x01`@R\x80\x88`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x83` \x01Qc\xFF\xFF\xFF\xFF\x16\x81RP`@Qa\x0B\xAB\x91\x90aUNV[`@Q\x80\x91\x03\x90\xA1_a\x0B\xBD\x82a2\x82V[\x90P_[\x86\x86\x85\x81\x81\x10a\x0B\xD3Wa\x0B\xD3aU\x03V[\x90P` \x02\x81\x01\x90a\x0B\xE5\x91\x90aU\x17V[a\x0B\xF3\x90` \x81\x01\x90aU\\V[\x90P\x81\x10\x15a\r\x01Wa\x0Ci\x87\x87\x86\x81\x81\x10a\x0C\x11Wa\x0C\x11aU\x03V[\x90P` \x02\x81\x01\x90a\x0C#\x91\x90aU\x17V[a\x0C1\x90` \x81\x01\x90aU\\V[\x83\x81\x81\x10a\x0CAWa\x0CAaU\x03V[\x90P` \x02\x01` \x81\x01\x90a\x0CV\x91\x90aJ\x82V[_\x84\x81R`\x99` R`@\x90 \x90a5bV[P\x7Fz\xB2`\xFE\n\xF1\x93\xDB_I\x86w\r\x83\x1B\xDAN\xA4`\x99\xDC\x81~\x8Bg\x16\xDC\xAE\x8A\xF8\xE8\x8B\x83\x88\x88\x87\x81\x81\x10a\x0C\x9EWa\x0C\x9EaU\x03V[\x90P` \x02\x81\x01\x90a\x0C\xB0\x91\x90aU\x17V[a\x0C\xBE\x90` \x81\x01\x90aU\\V[\x84\x81\x81\x10a\x0C\xCEWa\x0C\xCEaU\x03V[\x90P` \x02\x01` \x81\x01\x90a\x0C\xE3\x91\x90aJ\x82V[`@Qa\x0C\xF1\x92\x91\x90aU\xA1V[`@Q\x80\x91\x03\x90\xA1`\x01\x01a\x0B\xC1V[PPP`\x01\x01a\n\x99V[PPPPPV[``a\r!\x84\x84\x84Ca5vV[\x94\x93PPPPV[``a\r7\x85\x85\x85\x85a5vV[\x90P_[\x84Q\x81\x10\x15a\r\xBFWa\rg\x85\x82\x81Q\x81\x10a\rYWa\rYaU\x03V[` \x02` \x01\x01Q\x87a\x07\x9BV[a\r\xB7W_[\x84Q\x81\x10\x15a\r\xB5W_\x83\x83\x81Q\x81\x10a\r\x89Wa\r\x89aU\x03V[` \x02` \x01\x01Q\x82\x81Q\x81\x10a\r\xA2Wa\r\xA2aU\x03V[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a\rmV[P[`\x01\x01a\r;V[P\x94\x93PPPPV[`\x01`\x01`\xA0\x1B\x03\x80\x82\x16_\x90\x81R`\x97` R`@\x81 T\x90\x91\x16\x80\x15a\r\xF0W\x80a\x07\x94V[P\x90\x91\x90PV[`fT`\x01\x90`\x02\x90\x81\x16\x03a\x0E W`@Qc\x84\nH\xD5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x82a\x0E*\x81a4\xADV[a\x0EGW`@Qc\x93-\x94\xF7`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_`@Q\x80`@\x01`@R\x80\x86`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x85` \x01` \x81\x01\x90a\x0Eu\x91\x90aU5V[c\xFF\xFF\xFF\xFF\x16\x90R\x90Pa\x0E\x8C``\x85\x01\x85aU\\V[\x90Pa\x0E\x9B`@\x86\x01\x86aU\\V[\x90P\x14a\x0E\xBBW`@QcCqJ\xFD`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[` \x80\x82\x01Q\x82Q`\x01`\x01`\xA0\x1B\x03\x16_\x90\x81R`\x98\x90\x92R`@\x90\x91 a\x0E\xED\x91c\xFF\xFF\xFF\xFF\x90\x81\x16\x90a4\x96\x16V[a\x0F\nW`@Qc\x1F\xB1pU`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x0F a\x0F\x1A` \x86\x01\x86aJ\x82V[\x82a\x07\x9BV[a\x0F=W`@Qc\xEB\xBF\xF4\x97`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a\x0FK`@\x86\x01\x86aU\\V[\x90P`\x01`\x01`@\x1B\x03\x81\x11\x15a\x0FdWa\x0FdaH\xDBV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x0F\x8DW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P_[a\x0F\x9F`@\x87\x01\x87aU\\V[\x90P\x81\x10\x15a\x15mW\x80\x15\x80a\x102WPa\x0F\xBD`@\x87\x01\x87aU\\V[a\x0F\xC8`\x01\x84aU\xDBV[\x81\x81\x10a\x0F\xD7Wa\x0F\xD7aU\x03V[\x90P` \x02\x01` \x81\x01\x90a\x0F\xEC\x91\x90aJ\x82V[`\x01`\x01`\xA0\x1B\x03\x16a\x10\x02`@\x88\x01\x88aU\\V[\x83\x81\x81\x10a\x10\x12Wa\x10\x12aU\x03V[\x90P` \x02\x01` \x81\x01\x90a\x10'\x91\x90aJ\x82V[`\x01`\x01`\xA0\x1B\x03\x16\x11[a\x10OW`@Qc\x9F\x1C\x80S`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x10\\``\x87\x01\x87aU\\V[\x82\x81\x81\x10a\x10lWa\x10laU\x03V[\x90P` \x02\x015_\x10\x80\x15a\x10\xACWPg\r\xE0\xB6\xB3\xA7d\0\0a\x10\x92``\x88\x01\x88aU\\V[\x83\x81\x81\x10a\x10\xA2Wa\x10\xA2aU\x03V[\x90P` \x02\x015\x11\x15[a\x10\xC9W`@Qc\x13S`1`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x11%a\x10\xD9`@\x88\x01\x88aU\\V[\x83\x81\x81\x10a\x10\xE9Wa\x10\xE9aU\x03V[\x90P` \x02\x01` \x81\x01\x90a\x10\xFE\x91\x90aJ\x82V[`\x99_a\x11\n\x87a2\x82V[\x81R` \x01\x90\x81R` \x01_ a8c\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[a\x11BW`@Qc1\xBC4'`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x80a\x11\x94a\x11T` \x8A\x01\x8AaJ\x82V[a\x11]\x87a2\x82V[a\x11j`@\x8C\x01\x8CaU\\V[\x87\x81\x81\x10a\x11zWa\x11zaU\x03V[\x90P` \x02\x01` \x81\x01\x90a\x11\x8F\x91\x90aJ\x82V[a2\xE5V[\x80Q\x91\x93P\x91P`\x01`\x01`@\x1B\x03\x16_\x03a\x11\xB1WPPa\x15eV[_a\x11\xECa\x11\xC2``\x8B\x01\x8BaU\\V[\x86\x81\x81\x10a\x11\xD2Wa\x11\xD2aU\x03V[\x85Q`\x01`\x01`@\x1B\x03\x16\x92` \x90\x91\x02\x015\x90Pa8\x84V[\x83Q\x90\x91Pa\x12\x07`\x01`\x01`@\x1B\x03\x80\x84\x16\x90\x83\x16a8\x9AV[\x86\x86\x81Q\x81\x10a\x12\x19Wa\x12\x19aU\x03V[` \x02` \x01\x01\x81\x81RPP\x81\x83_\x01\x81\x81Qa\x126\x91\x90aU\xEEV[`\x01`\x01`@\x1B\x03\x16\x90RP\x83Q\x82\x90\x85\x90a\x12S\x90\x83\x90aU\xEEV[`\x01`\x01`@\x1B\x03\x16\x90RP` \x84\x01\x80Q\x83\x91\x90a\x12s\x90\x83\x90aU\xEEV[`\x01`\x01`@\x1B\x03\x16\x90RP` \x83\x01Q_`\x0F\x91\x90\x91\x0B\x12\x15a\x13xW_a\x12\xD6a\x12\xA2``\x8D\x01\x8DaU\\V[\x88\x81\x81\x10a\x12\xB2Wa\x12\xB2aU\x03V[\x90P` \x02\x015\x85` \x01Qa\x12\xC7\x90aV\rV[`\x01`\x01`\x80\x1B\x03\x16\x90a8\x84V[\x90P\x80`\x01`\x01`@\x1B\x03\x16\x84` \x01\x81\x81Qa\x12\xF3\x91\x90aV.V[`\x0F\x0B\x90RP_Q` a[7_9_Q\x90_Ra\x13\x14` \x8D\x01\x8DaJ\x82V[\x89a\x13\"`@\x8F\x01\x8FaU\\V[\x8A\x81\x81\x10a\x132Wa\x132aU\x03V[\x90P` \x02\x01` \x81\x01\x90a\x13G\x91\x90aJ\x82V[a\x13X\x88_\x01Q\x89` \x01Qa8\xAEV[\x88`@\x01Q`@Qa\x13n\x95\x94\x93\x92\x91\x90aV[V[`@Q\x80\x91\x03\x90\xA1P[a\x13\xCAa\x13\x88` \x8C\x01\x8CaJ\x82V[a\x13\x91\x89a2\x82V[a\x13\x9E`@\x8E\x01\x8EaU\\V[\x89\x81\x81\x10a\x13\xAEWa\x13\xAEaU\x03V[\x90P` \x02\x01` \x81\x01\x90a\x13\xC3\x91\x90aJ\x82V[\x87\x87a8\xCDV[_Q` a[7_9_Q\x90_Ra\x13\xE5` \x8C\x01\x8CaJ\x82V[\x88a\x13\xF3`@\x8E\x01\x8EaU\\V[\x89\x81\x81\x10a\x14\x03Wa\x14\x03aU\x03V[\x90P` \x02\x01` \x81\x01\x90a\x14\x18\x91\x90aJ\x82V[\x86Q`@Qa\x14,\x94\x93\x92\x91\x90C\x90aV[V[`@Q\x80\x91\x03\x90\xA1a\x14}a\x14D` \x8C\x01\x8CaJ\x82V[a\x14Q`@\x8D\x01\x8DaU\\V[\x88\x81\x81\x10a\x14aWa\x14aaU\x03V[\x90P` \x02\x01` \x81\x01\x90a\x14v\x91\x90aJ\x82V[\x86Qa;\x01V[`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16c`\x1B\xB3oa\x14\xB9` \x8D\x01\x8DaJ\x82V[a\x14\xC6`@\x8E\x01\x8EaU\\V[\x89\x81\x81\x10a\x14\xD6Wa\x14\xD6aU\x03V[\x90P` \x02\x01` \x81\x01\x90a\x14\xEB\x91\x90aJ\x82V[\x87Q`@Q`\xE0\x85\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x81R`\x01`\x01`\xA0\x1B\x03\x93\x84\x16`\x04\x82\x01R\x92\x90\x91\x16`$\x83\x01R`\x01`\x01`@\x1B\x03\x80\x86\x16`D\x84\x01R\x16`d\x82\x01R`\x84\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x15JW__\xFD[PZ\xF1\x15\x80\x15a\x15\\W=__>=_\xFD[PPPPPPPP[`\x01\x01a\x0F\x92V[P\x7F\x80\x96\x9A\xD2\x94(\xD6y~\xE7\xAA\xD0\x84\xF9\xE4\xA4*\x82\xFCPm\xCD,\xA3\xB6\xFBC\x1F\x85\xCC\xEB\xE5a\x15\x9C` \x87\x01\x87aJ\x82V[\x83a\x15\xAA`@\x89\x01\x89aU\\V[\x85a\x15\xB8`\x80\x8C\x01\x8CaV\xACV[`@Qa\x15\xCB\x97\x96\x95\x94\x93\x92\x91\x90aW\x16V[`@Q\x80\x91\x03\x90\xA1PPPPPPV[`\x01`\x01`\xA0\x1B\x03\x82\x16_\x90\x81R`\x9D` R`@\x81 ``\x91\x82\x91a\x16\0\x90a4\x82V[\x90P_\x81`\x01`\x01`@\x1B\x03\x81\x11\x15a\x16\x1BWa\x16\x1BaH\xDBV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x16_W\x81` \x01[`@\x80Q\x80\x82\x01\x90\x91R_\x80\x82R` \x82\x01R\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x169W\x90P[P\x90P_\x82`\x01`\x01`@\x1B\x03\x81\x11\x15a\x16{Wa\x16{aH\xDBV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x16\xB4W\x81` \x01[a\x16\xA1aH\xA8V[\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x16\x99W\x90P[P\x90P_[\x83\x81\x10\x15a\x177W`\x01`\x01`\xA0\x1B\x03\x88\x16_\x90\x81R`\x9D` R`@\x81 a\x16\xE6\x90a\t\xA5\x90\x84a4\x8BV[\x90P\x80\x84\x83\x81Q\x81\x10a\x16\xFBWa\x16\xFBaU\x03V[` \x02` \x01\x01\x81\x90RPa\x17\x11\x89\x82\x8Aa\x07sV[\x83\x83\x81Q\x81\x10a\x17#Wa\x17#aU\x03V[` \x90\x81\x02\x91\x90\x91\x01\x01RP`\x01\x01a\x16\xB9V[P\x90\x93P\x91PP[\x92P\x92\x90PV[``_a\x07\x94`\x99_a\x17X\x86a2\x82V[\x81R` \x01\x90\x81R` \x01_ a;cV[``_\x83Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x17\x86Wa\x17\x86aH\xDBV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x17\xAFW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P_[\x84Q\x81\x10\x15a\t\xFFWa\x17\xE0\x85\x82\x81Q\x81\x10a\x17\xD2Wa\x17\xD2aU\x03V[` \x02` \x01\x01Q\x85a(\x05V[\x82\x82\x81Q\x81\x10a\x17\xF2Wa\x17\xF2aU\x03V[`\x01`\x01`@\x1B\x03\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01R`\x01\x01a\x17\xB4V[`fT_\x90`\x01\x90\x81\x16\x03a\x18:W`@Qc\x84\nH\xD5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x83\x82\x14a\x18ZW`@QcCqJ\xFD`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_[\x84\x81\x10\x15a\x18\xC3Wa\x18\xBB\x87\x87\x87\x84\x81\x81\x10a\x18zWa\x18zaU\x03V[\x90P` \x02\x01` \x81\x01\x90a\x18\x8F\x91\x90aJ\x82V[\x86\x86\x85\x81\x81\x10a\x18\xA1Wa\x18\xA1aU\x03V[\x90P` \x02\x01` \x81\x01\x90a\x18\xB6\x91\x90aW\xACV[a;oV[`\x01\x01a\x18\\V[PPPPPPPV[\x83a\x18\xD6\x81a4\xADV[a\x18\xF3W`@Qc\x93-\x94\xF7`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@\x80Q\x80\x82\x01\x90\x91R`\x01`\x01`\xA0\x1B\x03\x86\x16\x81Rc\xFF\xFF\xFF\xFF\x85\x16` \x82\x01R_a\x19\x1F\x82a2\x82V[\x90Pa\x19`\x82` \x01Qc\xFF\xFF\xFF\xFF\x16`\x98_\x8A`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x90\x81R` \x01_ a4\x96\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[a\x19}W`@Qc\x1F\xB1pU`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_[\x84\x81\x10\x15a\x1A Wa\x19\x9C\x86\x86\x83\x81\x81\x10a\x0CAWa\x0CAaU\x03V[a\x19\xB9W`@QcX\\\xFB/`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x7Fz\xB2`\xFE\n\xF1\x93\xDB_I\x86w\r\x83\x1B\xDAN\xA4`\x99\xDC\x81~\x8Bg\x16\xDC\xAE\x8A\xF8\xE8\x8B\x83\x87\x87\x84\x81\x81\x10a\x19\xEDWa\x19\xEDaU\x03V[\x90P` \x02\x01` \x81\x01\x90a\x1A\x02\x91\x90aJ\x82V[`@Qa\x1A\x10\x92\x91\x90aU\xA1V[`@Q\x80\x91\x03\x90\xA1`\x01\x01a\x19\x7FV[PPPPPPPPV[``_\x82Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1AFWa\x1AFaH\xDBV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x1AoW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P_[\x83Q\x81\x10\x15a\t\xFFWa\x1A\xA0\x85\x85\x83\x81Q\x81\x10a\x1A\x93Wa\x1A\x93aU\x03V[` \x02` \x01\x01Qa(\x05V[\x82\x82\x81Q\x81\x10a\x1A\xB2Wa\x1A\xB2aU\x03V[`\x01`\x01`@\x1B\x03\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01R`\x01\x01a\x1AtV[``a\x1A\xFD\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a<sV[\x90P\x90V[3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x1C\0Wa\x1B;\x82a4\xADV[a\x1BXW`@QcH\xF5\xC3\xED`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@Qc6\xB8{\xD7`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90cmp\xF7\xAE\x90a\x1B\xA4\x90\x85\x90`\x04\x01aM\xDBV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1B\xBFW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1B\xE3\x91\x90aT\xE4V[a\x1C\0W`@Qc\xCC\xEA\x9Eo`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x08\xEC\x82\x82a<\xB0V[`@Qc#}\xFBG`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90cF\xFB\xF6\x8E\x90a\x1CV\x903\x90`\x04\x01aM\xDBV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1CqW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1C\x95\x91\x90aT\xE4V[a\x1C\xB2W`@Qc\x1Dw\xD4w`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x1C\xBC_\x19a4EV[V[_a\x07\x94\x83`\x9A_a\x11\n\x86a2\x82V[__a\x1C\xDB\x84\x84a>RV[\x95\x94PPPPPV[`fT`\x02\x90`\x04\x90\x81\x16\x03a\x1D\rW`@Qc\x84\nH\xD5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x1D\"a\x1D\x1D` \x84\x01\x84aJ\x82V[a4\xADV[\x80a\x1D;WPa\x1D;a\x1D\x1D`@\x84\x01` \x85\x01aJ\x82V[a\x1DXW`@QcH\xF5\xC3\xED`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_[a\x1Dg`@\x84\x01\x84aU\\V[\x90P\x81\x10\x15a )W_`@Q\x80`@\x01`@R\x80\x85` \x01` \x81\x01\x90a\x1D\x8F\x91\x90aJ\x82V[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01a\x1D\xAA`@\x87\x01\x87aU\\V[\x85\x81\x81\x10a\x1D\xBAWa\x1D\xBAaU\x03V[\x90P` \x02\x01` \x81\x01\x90a\x1D\xCF\x91\x90aU5V[c\xFF\xFF\xFF\xFF\x16\x81RP\x90Pa\x1E\x1C\x81` \x01Qc\xFF\xFF\xFF\xFF\x16`\x98_\x87` \x01` \x81\x01\x90a\x1D\xFE\x91\x90aJ\x82V[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x81\x01\x91\x90\x91R`@\x01_ \x90a4\x96V[a\x1E9W`@Qc\x1F\xB1pU`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x9E_a\x1EI` \x87\x01\x87aJ\x82V[`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x90\x81R` \x01_ _a\x1Er\x83a2\x82V[\x81R` \x81\x01\x91\x90\x91R`@\x01_ T`\xFF\x16a\x1E\xA2W`@Qc%\x13\x1DO`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x1E\xDCa\x1E\xAE\x82a2\x82V[`\x9C_a\x1E\xBE` \x89\x01\x89aJ\x82V[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x81\x01\x91\x90\x91R`@\x01_ \x90a?\xC1V[Pa\x1F\x14a\x1E\xED` \x86\x01\x86aJ\x82V[`\x9A_a\x1E\xF9\x85a2\x82V[\x81R` \x01\x90\x81R` \x01_ a?\xCC\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[Pa\x1F\"` \x85\x01\x85aJ\x82V[`\x01`\x01`\xA0\x1B\x03\x16\x7F\xAD4\xC3\x07\x0B\xE1\xDF\xFB\xCA\xA4\x99\xD0\0\xBA+\x8D\x98H\xAE\xFC\xAC0Y\xDF$]\xD9\\N\xCE\x14\xFE\x82`@Qa\x1FZ\x91\x90aUNV[`@Q\x80\x91\x03\x90\xA2`@\x80Q\x80\x82\x01\x90\x91R_\x81R` \x81\x01a\x1F\x9D\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0CaW\xCDV[c\xFF\xFF\xFF\xFF\x16\x90R`\x9E_a\x1F\xB5` \x88\x01\x88aJ\x82V[`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x90\x81R` \x01_ _a\x1F\xDE\x84a2\x82V[\x81R` \x80\x82\x01\x92\x90\x92R`@\x01_ \x82Q\x81T\x93\x90\x92\x01Qc\xFF\xFF\xFF\xFF\x16a\x01\0\x02d\xFF\xFF\xFF\xFF\0\x19\x92\x15\x15\x92\x90\x92\x16d\xFF\xFF\xFF\xFF\xFF\x19\x90\x93\x16\x92\x90\x92\x17\x17\x90UP`\x01\x01a\x1DZV[Pa =a\x03\xCA`@\x84\x01` \x85\x01aJ\x82V[`\x01`\x01`\xA0\x1B\x03\x16c0<\xA9Va X` \x85\x01\x85aJ\x82V[a h`@\x86\x01` \x87\x01aJ\x82V[a u`@\x87\x01\x87aU\\V[`@Q\x85c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a \x94\x94\x93\x92\x91\x90aX\"V[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a \xABW__\xFD[PZ\xF1\x15\x80\x15a \xBDW=__>=_\xFD[PPPPPPV[``a\x08\x10`\x9A_a\x17X\x85a2\x82V[a \xDEa?\xE0V[a\x1C\xBC_a@:V[`\x01`\x01`\xA0\x1B\x03\x81\x16_\x90\x81R`\x9C` R`@\x81 ``\x91\x90a!\x0B\x90a4\x82V[\x90P_\x81`\x01`\x01`@\x1B\x03\x81\x11\x15a!&Wa!&aH\xDBV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a!jW\x81` \x01[`@\x80Q\x80\x82\x01\x90\x91R_\x80\x82R` \x82\x01R\x81R` \x01\x90`\x01\x90\x03\x90\x81a!DW\x90P[P\x90P_[\x82\x81\x10\x15a\t\xFFW`\x01`\x01`\xA0\x1B\x03\x85\x16_\x90\x81R`\x9C` R`@\x90 a!\x9C\x90a\t\xA5\x90\x83a4\x8BV[\x82\x82\x81Q\x81\x10a!\xAEWa!\xAEaU\x03V[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a!oV[``_\x84Q`\x01`\x01`@\x1B\x03\x81\x11\x15a!\xDDWa!\xDDaH\xDBV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\"\x16W\x81` \x01[a\"\x03aH\xA8V[\x81R` \x01\x90`\x01\x90\x03\x90\x81a!\xFBW\x90P[P\x90P_[\x85Q\x81\x10\x15a\r\xBFWa\"H\x86\x82\x81Q\x81\x10a\"9Wa\"9aU\x03V[` \x02` \x01\x01Q\x86\x86a\x07sV[\x82\x82\x81Q\x81\x10a\"ZWa\"ZaU\x03V[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a\"\x1BV[``_\x83Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\"\x89Wa\"\x89aH\xDBV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\"\xB2W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P_[\x84Q\x81\x10\x15a\r\xBFW`\x01`\x01`\xA0\x1B\x03\x86\x16_\x90\x81R`\xA1` R`@\x81 \x86Qa#'\x92\x87\x92\x91\x89\x90\x86\x90\x81\x10a\"\xF2Wa\"\xF2aU\x03V[` \x02` \x01\x01Q`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x90\x81R` \x01_ a@\x8B\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x82\x82\x81Q\x81\x10a#9Wa#9aU\x03V[`\x01`\x01`@\x1B\x03\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01R`\x01\x01a\"\xB7V[`fT_\x90`\x01\x90\x81\x16\x03a#\x81W`@Qc\x84\nH\xD5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a#\x8A\x83a4\xADV[a#\xA7W`@QcH\xF5\xC3\xED`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[___a#\xB3\x86a-\xC2V[\x91P\x91P\x81a#\xD5W`@Qc\xFAU\xFC\x81`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x91P_\x90P[\x83Q\x81\x10\x15a\r\x0CW\x83\x81\x81Q\x81\x10a#\xF6Wa#\xF6aU\x03V[` \x02` \x01\x01Q`@\x01QQ\x84\x82\x81Q\x81\x10a$\x15Wa$\x15aU\x03V[` \x02` \x01\x01Q` \x01QQ\x14a$@W`@QcCqJ\xFD`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x84\x82\x81Q\x81\x10a$SWa$SaU\x03V[` \x90\x81\x02\x91\x90\x91\x01\x81\x01QQ\x80\x82\x01Q\x81Q`\x01`\x01`\xA0\x1B\x03\x16_\x90\x81R`\x98\x90\x93R`@\x90\x92 \x90\x92Pa$\x93\x91c\xFF\xFF\xFF\xFF\x90\x81\x16\x90a4\x96\x16V[a$\xB0W`@Qc\x1F\xB1pU`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a$\xBB\x87\x83a\x07\x9BV[\x90P_[\x86\x84\x81Q\x81\x10a$\xD1Wa$\xD1aU\x03V[` \x02` \x01\x01Q` \x01QQ\x81\x10\x15a'\xFAW_\x87\x85\x81Q\x81\x10a$\xF8Wa$\xF8aU\x03V[` \x02` \x01\x01Q` \x01Q\x82\x81Q\x81\x10a%\x15Wa%\x15aU\x03V[` \x02` \x01\x01Q\x90Pa%,\x89\x82a\xFF\xFFa;oV[__a%;\x8Ba\x07\x89\x88a2\x82V[\x91P\x91P\x80`@\x01Qc\xFF\xFF\xFF\xFF\x16_\x14a%iW`@Qc\r\x8F\xCB\xE3`\xE4\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a%v\x87\x85\x84\x89a@\x9FV[\x90Pa%\xBB\x82_\x01Q\x8C\x8A\x81Q\x81\x10a%\x91Wa%\x91aU\x03V[` \x02` \x01\x01Q`@\x01Q\x87\x81Q\x81\x10a%\xAEWa%\xAEaU\x03V[` \x02` \x01\x01Qa@\xD5V[`\x0F\x0B` \x83\x01\x81\x90R_\x03a%\xE4W`@QcF\x06\x17\x93`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x82` \x01Q`\x0F\x0B\x12\x15a'(W\x80\x15a&\xAAWa&ea&\x05\x88a2\x82V[`\x01`\x01`\xA0\x1B\x03\x80\x8F\x16_\x90\x81R`\xA3` \x90\x81R`@\x80\x83 \x93\x8A\x16\x83R\x92\x90R \x90\x81T`\x01`\x80\x1B\x90\x81\x90\x04`\x0F\x0B_\x81\x81R`\x01\x80\x86\x01` R`@\x90\x91 \x93\x90\x93U\x83T`\x01`\x01`\x80\x1B\x03\x90\x81\x16\x93\x90\x91\x01\x16\x02\x17\x90UV[a&\x8F\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0CaW\xCDV[a&\x9A\x90`\x01aW\xCDV[c\xFF\xFF\xFF\xFF\x16`@\x83\x01Ra'\x95V[a&\xBC\x83` \x01Q\x83` \x01Qa8\xAEV[`\x01`\x01`@\x1B\x03\x16` \x84\x01R\x8AQ\x8B\x90\x89\x90\x81\x10a&\xDEWa&\xDEaU\x03V[` \x02` \x01\x01Q`@\x01Q\x85\x81Q\x81\x10a&\xFBWa&\xFBaU\x03V[` \x90\x81\x02\x91\x90\x91\x01\x81\x01Q`\x01`\x01`@\x1B\x03\x16\x83R_\x90\x83\x01Rc\xFF\xFF\xFF\xFFC\x16`@\x83\x01Ra'\x95V[_\x82` \x01Q`\x0F\x0B\x13\x15a'\x95Wa'I\x83` \x01Q\x83` \x01Qa8\xAEV[`\x01`\x01`@\x1B\x03\x90\x81\x16` \x85\x01\x81\x90R\x84Q\x90\x91\x16\x10\x15a'\x7FW`@Qcl\x9B\xE0\xBF`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a'\x89\x89CaW\xCDV[c\xFF\xFF\xFF\xFF\x16`@\x83\x01R[a'\xAA\x8Ca'\xA2\x89a2\x82V[\x86\x86\x86a8\xCDV[_Q` a[7_9_Q\x90_R\x8C\x88\x86a'\xCC\x86_\x01Q\x87` \x01Qa8\xAEV[\x86`@\x01Q`@Qa'\xE2\x95\x94\x93\x92\x91\x90aV[V[`@Q\x80\x91\x03\x90\xA1PP`\x01\x90\x92\x01\x91Pa$\xBF\x90PV[PPP`\x01\x01a#\xDBV[`\x01`\x01`\xA0\x1B\x03\x80\x83\x16_\x90\x81R`\xA1` \x90\x81R`@\x80\x83 \x93\x85\x16\x83R\x92\x90R\x90\x81 a\x07\x94\x90a@\xECV[\x82a(>\x81a4\xADV[a([W`@Qc\x93-\x94\xF7`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x84\x16_\x90\x81R`\xA4` R`@\x90 T`\xFF\x16a(\x9DW`\x01`\x01`\xA0\x1B\x03\x84\x16_\x90\x81R`\xA4` R`@\x90 \x80T`\xFF\x19\x16`\x01\x17\x90U[\x83`\x01`\x01`\xA0\x1B\x03\x16\x7F\xA8\x9C\x1D\xC2C\xD8\x90\x8A\x96\xDD\x84\x94K\xCC\x97\xD6\xBCj\xC0\r\xD7\x8E b\x15v\xBEj<\x947\x13\x84\x84`@Qa(\xD8\x92\x91\x90aXNV[`@Q\x80\x91\x03\x90\xA2PPPPV[`fT`\x02\x90`\x04\x90\x81\x16\x03a)\x0FW`@Qc\x84\nH\xD5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x82a)\x19\x81a4\xADV[a)6W`@Qc\x93-\x94\xF7`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@Qc6\xB8{\xD7`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90cmp\xF7\xAE\x90a)\x82\x90\x87\x90`\x04\x01aM\xDBV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a)\x9DW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a)\xC1\x91\x90aT\xE4V[a)\xDEW`@Qc\xCC\xEA\x9Eo`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_[a)\xED` \x85\x01\x85aU\\V[\x90P\x81\x10\x15a+\xB5W`@\x80Q\x80\x82\x01\x90\x91R_\x90\x80a*\x10` \x88\x01\x88aJ\x82V[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x86\x80` \x01\x90a*.\x91\x90aU\\V[\x85\x81\x81\x10a*>Wa*>aU\x03V[\x90P` \x02\x01` \x81\x01\x90a*S\x91\x90aU5V[c\xFF\xFF\xFF\xFF\x90\x81\x16\x90\x91R` \x80\x83\x01Q\x83Q`\x01`\x01`\xA0\x1B\x03\x16_\x90\x81R`\x98\x90\x92R`@\x90\x91 \x92\x93Pa*\x8F\x92\x91\x90\x81\x16\x90a4\x96\x16V[a*\xACW`@Qc\x1F\xB1pU`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a*\xB6\x86\x82a\x07\x9BV[\x15a*\xD4W`@Qclln'`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a*\xFDa*\xE0\x82a2\x82V[`\x01`\x01`\xA0\x1B\x03\x88\x16_\x90\x81R`\x9C` R`@\x90 \x90a5WV[Pa+)\x86`\x9A_a+\x0E\x85a2\x82V[\x81R` \x01\x90\x81R` \x01_ a5b\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[P\x85`\x01`\x01`\xA0\x1B\x03\x16\x7FC#.\xDF\x90qu=#!\xE5\xFA~\x01\x83c\xEE$\x8E_!B\xE6\xC0\x8E\xDD2e\xBF\xB4\x89^\x82`@Qa+c\x91\x90aUNV[`@Q\x80\x91\x03\x90\xA2`\x01`\x01`\xA0\x1B\x03\x86\x16_\x90\x81R`\x9E` R`@\x81 `\x01\x91a+\x8E\x84a2\x82V[\x81R` \x81\x01\x91\x90\x91R`@\x01_ \x80T`\xFF\x19\x16\x91\x15\x15\x91\x90\x91\x17\x90UP`\x01\x01a)\xE0V[Pa+\xC6a\x03\xCA` \x85\x01\x85aJ\x82V[`\x01`\x01`\xA0\x1B\x03\x16c\xC6?\xD5\x02\x85a+\xE2` \x87\x01\x87aJ\x82V[a+\xEF` \x88\x01\x88aU\\V[a+\xFC`@\x8A\x01\x8AaV\xACV[`@Q\x87c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a,\x1D\x96\x95\x94\x93\x92\x91\x90aXaV[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a,4W__\xFD[PZ\xF1\x15\x80\x15a\x1A W=__>=_\xFD[_a\x08\x10`\x9A_a,V\x85a2\x82V[\x81R` \x01\x90\x81R` \x01_ a4\x82V[\x83a,r\x81a4\xADV[a,\x8FW`@Qc\x93-\x94\xF7`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@\x80Q\x80\x82\x01\x82R`\x01`\x01`\xA0\x1B\x03\x87\x16\x80\x82Rc\xFF\xFF\xFF\xFF\x80\x88\x16` \x80\x85\x01\x82\x90R_\x93\x84R`\x98\x90R\x93\x90\x91 \x91\x92a,\xCE\x92\x91a4\x96\x16V[a,\xEBW`@Qc\x1F\xB1pU`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a,\xF5\x82a2\x82V[\x90P_[\x84\x81\x10\x15a\x1A Wa->\x86\x86\x83\x81\x81\x10a-\x16Wa-\x16aU\x03V[\x90P` \x02\x01` \x81\x01\x90a-+\x91\x90aJ\x82V[_\x84\x81R`\x99` R`@\x90 \x90a?\xCCV[a-[W`@Qc1\xBC4'`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x7F{K\x07=\x80\xDC\xACU\xA1\x11w\xD8E\x9A\xD9\xF6d\xCE\xEB\x91\xF7\x1F'\x16{\xB1O\x81R\xA7\xEE\xEE\x83\x87\x87\x84\x81\x81\x10a-\x8FWa-\x8FaU\x03V[\x90P` \x02\x01` \x81\x01\x90a-\xA4\x91\x90aJ\x82V[`@Qa-\xB2\x92\x91\x90aU\xA1V[`@Q\x80\x91\x03\x90\xA1`\x01\x01a,\xF9V[`\x01`\x01`\xA0\x1B\x03\x81\x16_\x90\x81R`\x9B` \x90\x81R`@\x80\x83 \x81Q`\x80\x81\x01\x83R\x90Tc\xFF\xFF\xFF\xFF\x80\x82\x16\x80\x84R`\xFF`\x01` \x1B\x84\x04\x16\x15\x15\x95\x84\x01\x86\x90R`\x01`(\x1B\x83\x04\x82\x16\x94\x84\x01\x94\x90\x94R`\x01`H\x1B\x90\x91\x04\x16``\x82\x01\x81\x90R\x84\x93\x91\x92\x91\x90\x15\x80\x15\x90a.AWP\x82``\x01Qc\xFF\xFF\xFF\xFF\x16C\x10\x15[\x15a.PWPP`@\x81\x01Q`\x01[\x95\x90\x94P\x92PPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16_\x90\x81R`\x98` R`@\x81 a\x08\x10\x90a4\x82V[`\x01`\x01`\xA0\x1B\x03\x82\x16_\x90\x81R`\x9F` R`@\x81 ``\x91\x90a\x08\x0C\x90\x82a\x17X\x86a2\x82V[_Ta\x01\0\x90\x04`\xFF\x16\x15\x80\x80\x15a.\xC1WP_T`\x01`\xFF\x90\x91\x16\x10[\x80a.\xDAWP0;\x15\x80\x15a.\xDAWP_T`\xFF\x16`\x01\x14[a/BW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01Rm\x19\x1EH\x1A[\x9A]\x1AX[\x1A^\x99Y`\x92\x1B`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[_\x80T`\xFF\x19\x16`\x01\x17\x90U\x80\x15a/cW_\x80Ta\xFF\0\x19\x16a\x01\0\x17\x90U[a/l\x82a4EV[a/u\x83a@:V[\x80\x15a/\xBBW_\x80Ta\xFF\0\x19\x16\x90U`@Q`\x01\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01[`@Q\x80\x91\x03\x90\xA1[PPPV[\x81a/\xCA\x81a4\xADV[a/\xE7W`@Qc\x93-\x94\xF7`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@Qc\xB5&W\x87`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x83\x16\x90c\xB5&W\x87\x90a0\x13\x90\x86\x90`\x04\x01aM\xDBV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a0.W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a0R\x91\x90aT\xE4V[a0oW`@Qc\x1D\x0B\x13\xC1`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x83\x81\x16_\x90\x81R`\x97` R`@\x90 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x91\x84\x16\x91\x90\x91\x17\x90U\x7F*\xE9E\xC4\x0CD\xDC\x0E\xC2c\xF9V\t\xC3\xFD\xC6\x95.\n\xEF\xA2-ct\xE4O,\x99z\xCE\xDF\x85\x83a0\xC7\x81a\r\xC8V[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x93\x84\x16\x81R\x92\x90\x91\x16` \x83\x01R\x01a/\xB2V[a0\xEFa?\xE0V[`\x01`\x01`\xA0\x1B\x03\x81\x16a1TW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01Reddress`\xD0\x1B`d\x82\x01R`\x84\x01a/9V[a1]\x81a@:V[PV[__a\r\xBF\x84\x84a>RV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\xEA\xB6mz`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a1\xC8W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a1\xEC\x91\x90aX\xADV[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a2\x1DW`@QcyH!\xFF`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`fT\x80\x19\x82\x19\x81\x16\x14a2DW`@Qc\xC6\x1D\xCA]`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`f\x82\x90U`@Q\x82\x81R3\x90\x7F5\x82\xD1\x82\x8E&\xBFV\xBD\x80\x15\x02\xBC\x02\x1A\xC0\xBC\x8A\xFBW\xC8&\xE4\x98kEY<\x8F\xAD8\x9C\x90` \x01`@Q\x80\x91\x03\x90\xA2PPV[_\x81_\x01Q\x82` \x01Qc\xFF\xFF\xFF\xFF\x16`@Q` \x01a2\xCD\x92\x91\x90``\x92\x90\x92\x1Bk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x82R`\xA0\x1B`\x01`\x01`\xA0\x1B\x03\x19\x16`\x14\x82\x01R` \x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@Ra\x08\x10\x90aX\xC8V[`@\x80Q\x80\x82\x01\x90\x91R_\x80\x82R` \x82\x01Ra3\0aH\xA8V[`@\x80Q\x80\x82\x01\x82R`\x01`\x01`\xA0\x1B\x03\x80\x88\x16_\x90\x81R`\xA1` \x90\x81R\x84\x82 \x92\x88\x16\x82R\x91\x90\x91R\x91\x82 \x81\x90a39\x90a@\xECV[`\x01`\x01`@\x1B\x03\x90\x81\x16\x82R`\x01`\x01`\xA0\x1B\x03\x89\x81\x16_\x81\x81R`\xA2` \x90\x81R`@\x80\x83 \x94\x8C\x16\x80\x84R\x94\x82R\x80\x83 T\x86\x16\x96\x82\x01\x96\x90\x96R\x91\x81R`\xA0\x82R\x84\x81 \x8B\x82R\x82R\x84\x81 \x92\x81R\x91\x81R\x90\x83\x90 \x83Q``\x81\x01\x85R\x90T\x92\x83\x16\x81R`\x01`@\x1B\x83\x04`\x0F\x0B\x91\x81\x01\x91\x90\x91R`\x01`\xC0\x1B\x90\x91\x04c\xFF\xFF\xFF\xFF\x16\x91\x81\x01\x82\x90R\x91\x92PC\x10\x15a3\xDBW\x90\x92P\x90Pa4=V[a3\xEC\x81_\x01Q\x82` \x01Qa8\xAEV[`\x01`\x01`@\x1B\x03\x16\x81R` \x81\x01Q_`\x0F\x91\x90\x91\x0B\x12\x15a4*Wa4\x1B\x82` \x01Q\x82` \x01Qa8\xAEV[`\x01`\x01`@\x1B\x03\x16` \x83\x01R[_`@\x82\x01\x81\x90R` \x82\x01R\x90\x92P\x90P[\x93P\x93\x91PPV[`f\x81\x90U`@Q\x81\x81R3\x90\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=\x90` \x01`@Q\x80\x91\x03\x90\xA2PV[_a\x08\x10\x82T\x90V[_a\x07\x94\x83\x83a@\xFFV[_\x81\x81R`\x01\x83\x01` R`@\x81 T\x15\x15a\x07\x94V[`@Qc\x1B\xEB+\x97`\xE3\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x82\x81\x16`\x04\x83\x01R3`$\x83\x01R0`D\x83\x01R_\x805`\x01`\x01`\xE0\x1B\x03\x19\x16`d\x84\x01R\x91\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x16\x90c\xDFY\\\xB8\x90`\x84\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a53W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x08\x10\x91\x90aT\xE4V[_a\x07\x94\x83\x83aA%V[_a\x07\x94\x83`\x01`\x01`\xA0\x1B\x03\x84\x16aA%V[``\x83Q`\x01`\x01`@\x1B\x03\x81\x11\x15a5\x91Wa5\x91aH\xDBV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a5\xC4W\x81` \x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a5\xAFW\x90P[P\x90P_\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\xF0\xE0\xE6v\x86\x86`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a6\x16\x92\x91\x90aX\xEBV[_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a60W=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra6W\x91\x90\x81\x01\x90aY\x0FV[\x90P_[\x85Q\x81\x10\x15a8YW_\x86\x82\x81Q\x81\x10a6wWa6waU\x03V[` \x02` \x01\x01Q\x90P\x85Q`\x01`\x01`@\x1B\x03\x81\x11\x15a6\x9AWa6\x9AaH\xDBV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a6\xC3W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x84\x83\x81Q\x81\x10a6\xD6Wa6\xD6aU\x03V[` \x90\x81\x02\x91\x90\x91\x01\x01R_[\x86Q\x81\x10\x15a8OW_\x87\x82\x81Q\x81\x10a6\xFFWa6\xFFaU\x03V[` \x90\x81\x02\x91\x90\x91\x01\x81\x01Q`\x01`\x01`\xA0\x1B\x03\x80\x86\x16_\x90\x81R`\xA1\x84R`@\x80\x82 \x92\x84\x16\x82R\x91\x90\x93R\x82 \x90\x92Pa7:\x90a@\xECV[\x90P\x80`\x01`\x01`@\x1B\x03\x16_\x03a7SWPPa8GV[_a7_\x85\x8D\x85a\x07sV[\x90P\x88c\xFF\xFF\xFF\xFF\x16\x81`@\x01Qc\xFF\xFF\xFF\xFF\x16\x11\x15\x80\x15a7\x87WP_\x81` \x01Q`\x0F\x0B\x12[\x15a7\xA9Wa7\x9D\x81_\x01Q\x82` \x01Qa8\xAEV[`\x01`\x01`@\x1B\x03\x16\x81R[\x80Q_\x90a7\xC4\x90`\x01`\x01`@\x1B\x03\x90\x81\x16\x90\x85\x16a8\x9AV[\x90Pa8\x0B\x81\x89\x89\x81Q\x81\x10a7\xDCWa7\xDCaU\x03V[` \x02` \x01\x01Q\x87\x81Q\x81\x10a7\xF5Wa7\xF5aU\x03V[` \x02` \x01\x01QaAq\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x89\x88\x81Q\x81\x10a8\x1DWa8\x1DaU\x03V[` \x02` \x01\x01Q\x86\x81Q\x81\x10a86Wa86aU\x03V[` \x02` \x01\x01\x81\x81RPPPPPP[`\x01\x01a6\xE3V[PP`\x01\x01a6[V[PP\x94\x93PPPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16_\x90\x81R`\x01\x83\x01` R`@\x81 T\x15\x15a\x07\x94V[_a\x07\x94\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0`\x01aA\x85V[_a\x07\x94\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84aA\xDEV[_a\x07\x94a8\xC5\x83`\x01`\x01`@\x1B\x03\x86\x16aV.V[`\x0F\x0BaB\xC3V[` \x80\x83\x01Q`\x01`\x01`\xA0\x1B\x03\x80\x88\x16_\x90\x81R`\xA2\x84R`@\x80\x82 \x92\x88\x16\x82R\x91\x90\x93R\x90\x91 T`\x01`\x01`@\x1B\x03\x90\x81\x16\x91\x16\x14a9\x8FW` \x82\x81\x01\x80Q`\x01`\x01`\xA0\x1B\x03\x88\x81\x16_\x90\x81R`\xA2\x85R`@\x80\x82 \x92\x89\x16\x82R\x91\x90\x94R\x92\x83\x90 \x80Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\x01`\x01`@\x1B\x03\x90\x92\x16\x91\x90\x91\x17\x90UQ\x90Q\x7F\xAC\xF9\t_\xEB:7\x0C\x9C\xF6\x92B\x1Ci\xEF2\rM\xB5\xC6nj})\xC7iN\xB0#d\xFCU\x91a9\x86\x91\x88\x91\x87\x91aZ\x1BV[`@Q\x80\x91\x03\x90\xA1[`\x01`\x01`\xA0\x1B\x03\x80\x86\x16_\x90\x81R`\xA0` \x90\x81R`@\x80\x83 \x88\x84R\x82R\x80\x83 \x93\x87\x16\x83R\x92\x81R\x90\x82\x90 \x83Q\x81T\x92\x85\x01Q\x93\x85\x01Qc\xFF\xFF\xFF\xFF\x16`\x01`\xC0\x1B\x02c\xFF\xFF\xFF\xFF`\xC0\x1B\x19`\x01`\x01`\x80\x1B\x03\x86\x16`\x01`@\x1B\x02`\x01`\x01`\xC0\x1B\x03\x19\x90\x95\x16`\x01`\x01`@\x1B\x03\x90\x93\x16\x92\x90\x92\x17\x93\x90\x93\x17\x16\x91\x90\x91\x17\x90U`\x0F\x0B\x15a:qW`\x01`\x01`\xA0\x1B\x03\x85\x16_\x90\x81R`\x9F` \x90\x81R`@\x80\x83 \x87\x84R\x90\x91R\x90 a:I\x90\x84a5bV[P`\x01`\x01`\xA0\x1B\x03\x85\x16_\x90\x81R`\x9D` R`@\x90 a:k\x90\x85a5WV[Pa\r\x0CV[\x80Q`\x01`\x01`@\x1B\x03\x16_\x03a\r\x0CW`\x01`\x01`\xA0\x1B\x03\x85\x16_\x90\x81R`\x9F` \x90\x81R`@\x80\x83 \x87\x84R\x90\x91R\x90 a:\xAE\x90\x84a?\xCCV[P`\x01`\x01`\xA0\x1B\x03\x85\x16_\x90\x81R`\x9F` \x90\x81R`@\x80\x83 \x87\x84R\x90\x91R\x90 a:\xDA\x90a4\x82V[_\x03a\r\x0CW`\x01`\x01`\xA0\x1B\x03\x85\x16_\x90\x81R`\x9D` R`@\x90 a \xBD\x90\x85a?\xC1V[`\x01`\x01`\xA0\x1B\x03\x80\x84\x16_\x90\x81R`\xA1` \x90\x81R`@\x80\x83 \x93\x86\x16\x83R\x92\x90R a;0\x90C\x83aC.V[\x7F\x1CdX\x07\x9AA\x07}\0<\x11\xFA\xF9\xBF\t~i;\xD6yy\xE4\xE6P\x0B\xAC{)\xDBw\x9B\\\x83\x83\x83`@Qa/\xB2\x93\x92\x91\x90aZ\x1BV[``_a\x07\x94\x83aCBV[`\x01`\x01`\xA0\x1B\x03\x83\x81\x16_\x90\x81R`\xA3` \x90\x81R`@\x80\x83 \x93\x86\x16\x83R\x92\x90R\x90\x81 T`\x0F\x81\x81\x0B`\x01`\x80\x1B\x90\x92\x04\x90\x0B\x03[_\x81\x11\x80\x15a;\xB9WP\x82a\xFF\xFF\x16\x82\x10[\x15a\r\x0CW`\x01`\x01`\xA0\x1B\x03\x80\x86\x16_\x90\x81R`\xA3` \x90\x81R`@\x80\x83 \x93\x88\x16\x83R\x92\x90R\x90\x81 a;\xED\x90aC\x9BV[\x90P__a;\xFC\x88\x84\x89a2\xE5V[\x91P\x91P\x80`@\x01Qc\xFF\xFF\xFF\xFF\x16C\x10\x15a<\x1AWPPPa\r\x0CV[a<'\x88\x84\x89\x85\x85a8\xCDV[`\x01`\x01`\xA0\x1B\x03\x80\x89\x16_\x90\x81R`\xA3` \x90\x81R`@\x80\x83 \x93\x8B\x16\x83R\x92\x90R a<T\x90aC\xEDV[Pa<^\x85aZGV[\x94Pa<i\x84aZ_V[\x93PPPPa;\xA7V[``_a<\x7F\x83aDaV[`@\x80Q` \x80\x82R\x81\x83\x01\x90\x92R\x91\x92P_\x91\x90` \x82\x01\x81\x806\x837PPP\x91\x82RP` \x81\x01\x92\x90\x92RP\x90V[`\x01`\x01`\xA0\x1B\x03\x82\x16_\x90\x81R`\x9B` \x90\x81R`@\x91\x82\x90 \x82Q`\x80\x81\x01\x84R\x90Tc\xFF\xFF\xFF\xFF\x80\x82\x16\x83R`\xFF`\x01` \x1B\x83\x04\x16\x15\x15\x93\x83\x01\x93\x90\x93R`\x01`(\x1B\x81\x04\x83\x16\x93\x82\x01\x93\x90\x93R`\x01`H\x1B\x90\x92\x04\x16``\x82\x01\x81\x90R\x15\x80\x15\x90a=*WP\x80``\x01Qc\xFF\xFF\xFF\xFF\x16C\x10\x15[\x15a=DW`@\x81\x01Qc\xFF\xFF\xFF\xFF\x16\x81R`\x01` \x82\x01R[c\xFF\xFF\xFF\xFF\x82\x16`@\x82\x01Ra=z\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0CaW\xCDV[a=\x85\x90`\x01aW\xCDV[c\xFF\xFF\xFF\xFF\x90\x81\x16``\x83\x81\x01\x91\x82R`\x01`\x01`\xA0\x1B\x03\x86\x16_\x81\x81R`\x9B` \x90\x81R`@\x91\x82\x90 \x87Q\x81T\x83\x8A\x01Q\x85\x8B\x01Q\x98Q\x92\x8A\x16d\xFF\xFF\xFF\xFF\xFF\x19\x90\x92\x16\x91\x90\x91\x17`\x01` \x1B\x91\x15\x15\x91\x90\x91\x02\x17l\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\x19\x16`\x01`(\x1B\x97\x89\x16\x97\x90\x97\x02c\xFF\xFF\xFF\xFF`H\x1B\x19\x16\x96\x90\x96\x17`\x01`H\x1B\x96\x88\x16\x96\x87\x02\x17\x90U\x81Q\x92\x83R\x94\x87\x16\x94\x82\x01\x94\x90\x94R\x92\x83\x01\x91\x90\x91R\x7FN\x85u\x1Dc1Pllb3_ ~\xB3\x1F\x12\xA6\x1EW\x0F4\xF5\xC1v@0\x87\x85\xC6\xD4\xDB\x91\x01a/\xB2V[`\x01`\x01`\xA0\x1B\x03\x82\x81\x16_\x81\x81R`\xA2` \x90\x81R`@\x80\x83 \x94\x86\x16\x80\x84R\x94\x82R\x80\x83 T\x93\x83R`\xA3\x82R\x80\x83 \x94\x83R\x93\x90R\x91\x82 T`\x01`\x01`@\x1B\x03\x90\x91\x16\x91\x90`\x0F\x81\x81\x0B`\x01`\x80\x1B\x90\x92\x04\x90\x0B\x03\x81[\x81\x81\x10\x15a?}W`\x01`\x01`\xA0\x1B\x03\x80\x87\x16_\x90\x81R`\xA3` \x90\x81R`@\x80\x83 \x93\x89\x16\x83R\x92\x90R\x90\x81 a>\xE5\x90\x83aD\x88V[`\x01`\x01`\xA0\x1B\x03\x88\x81\x16_\x90\x81R`\xA0` \x90\x81R`@\x80\x83 \x85\x84R\x82R\x80\x83 \x93\x8B\x16\x83R\x92\x81R\x90\x82\x90 \x82Q``\x81\x01\x84R\x90T`\x01`\x01`@\x1B\x03\x81\x16\x82R`\x01`@\x1B\x81\x04`\x0F\x0B\x92\x82\x01\x92\x90\x92R`\x01`\xC0\x1B\x90\x91\x04c\xFF\xFF\xFF\xFF\x16\x91\x81\x01\x82\x90R\x91\x92PC\x10\x15a?`WPPa?}V[a?n\x86\x82` \x01Qa8\xAEV[\x95PPP\x80`\x01\x01\x90Pa>\xADV[P`\x01`\x01`\xA0\x1B\x03\x80\x86\x16_\x90\x81R`\xA1` \x90\x81R`@\x80\x83 \x93\x88\x16\x83R\x92\x90R \x83\x90a?\xAD\x90a@\xECV[a?\xB7\x91\x90aU\xEEV[\x91PP\x92P\x92\x90PV[_a\x07\x94\x83\x83aD\xF7V[_a\x07\x94\x83`\x01`\x01`\xA0\x1B\x03\x84\x16aD\xF7V[`3T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x1C\xBCW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a/9V[`3\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x90\x93U`@Q\x91\x16\x91\x90\x82\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90_\x90\xA3PPV[_a\x07\x94\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0aE\xDAV[_a@\xB0\x84`\x99_a\x11\n\x89a2\x82V[\x80\x15a@\xB9WP\x81[\x80\x15a\x1C\xDBWPP\x90Q`\x01`\x01`@\x1B\x03\x16\x15\x15\x93\x92PPPV[_a\x07\x94`\x01`\x01`@\x1B\x03\x80\x85\x16\x90\x84\x16aZtV[_a\x08\x10\x82g\r\xE0\xB6\xB3\xA7d\0\0aF/V[_\x82_\x01\x82\x81T\x81\x10aA\x14WaA\x14aU\x03V[\x90_R` _ \x01T\x90P\x92\x91PPV[_\x81\x81R`\x01\x83\x01` R`@\x81 TaAjWP\x81T`\x01\x81\x81\x01\x84U_\x84\x81R` \x80\x82 \x90\x93\x01\x84\x90U\x84T\x84\x82R\x82\x86\x01\x90\x93R`@\x90 \x91\x90\x91Ua\x08\x10V[P_a\x08\x10V[_a\x07\x94\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0aA\xDEV[__aA\x92\x86\x86\x86aA\xDEV[\x90P`\x01\x83`\x02\x81\x11\x15aA\xA8WaA\xA8aZ\xA1V[\x14\x80\x15aA\xC4WP_\x84\x80aA\xBFWaA\xBFaZ\xB5V[\x86\x88\t\x11[\x15a\x1C\xDBWaA\xD4`\x01\x82aZ\xC9V[\x96\x95PPPPPPV[_\x80\x80_\x19\x85\x87\t\x85\x87\x02\x92P\x82\x81\x10\x83\x82\x03\x03\x91PP\x80_\x03aB\x15W\x83\x82\x81aB\x0BWaB\x0BaZ\xB5V[\x04\x92PPPa\x07\x94V[\x80\x84\x11aB\\W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x15`$\x82\x01RtMath: mulDiv overflow`X\x1B`D\x82\x01R`d\x01a/9V[_\x84\x86\x88\t`\x02`\x01\x87\x19\x81\x01\x88\x16\x97\x88\x90\x04`\x03\x81\x02\x83\x18\x80\x82\x02\x84\x03\x02\x80\x82\x02\x84\x03\x02\x80\x82\x02\x84\x03\x02\x80\x82\x02\x84\x03\x02\x80\x82\x02\x84\x03\x02\x90\x81\x02\x90\x92\x03\x90\x91\x02_\x88\x90\x03\x88\x90\x04\x90\x91\x01\x85\x83\x11\x90\x94\x03\x93\x90\x93\x02\x93\x03\x94\x90\x94\x04\x91\x90\x91\x17\x02\x94\x93PPPPV[_`\x01`\x01`@\x1B\x03\x82\x11\x15aC*W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FSafeCast: value doesn't fit in 6`D\x82\x01Re4 bits`\xD0\x1B`d\x82\x01R`\x84\x01a/9V[P\x90V[a/\xBB\x83\x83`\x01`\x01`@\x1B\x03\x84\x16aFfV[``\x81_\x01\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15aC\x8FW` \x02\x82\x01\x91\x90_R` _ \x90[\x81T\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11aC{W[PPPPP\x90P\x91\x90PV[_aC\xB5\x82T`\x0F\x81\x81\x0B`\x01`\x80\x1B\x90\x92\x04\x90\x0B\x13\x15\x90V[\x15aC\xD3W`@Qc\x1E\xD9P\x95`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[P\x80T`\x0F\x0B_\x90\x81R`\x01\x90\x91\x01` R`@\x90 T\x90V[_aD\x07\x82T`\x0F\x81\x81\x0B`\x01`\x80\x1B\x90\x92\x04\x90\x0B\x13\x15\x90V[\x15aD%W`@Qc\x1E\xD9P\x95`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[P\x80T`\x0F\x0B_\x81\x81R`\x01\x80\x84\x01` R`@\x82 \x80T\x92\x90U\x83T`\x01`\x01`\x80\x1B\x03\x19\x16\x92\x01`\x01`\x01`\x80\x1B\x03\x16\x91\x90\x91\x17\x90\x91U\x90V[_`\xFF\x82\x16`\x1F\x81\x11\x15a\x08\x10W`@Qc,\xD4J\xC3`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[__aD\xAAaD\x96\x84aGiV[\x85TaD\xA5\x91\x90`\x0F\x0BaZ\xDCV[aG\xD2V[\x84T\x90\x91P`\x01`\x80\x1B\x90\x04`\x0F\x90\x81\x0B\x90\x82\x90\x0B\x12aD\xDDW`@Qc-\x04\x83\xC5`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x0F\x0B_\x90\x81R`\x01\x93\x90\x93\x01` RPP`@\x90 T\x90V[_\x81\x81R`\x01\x83\x01` R`@\x81 T\x80\x15aE\xD1W_aE\x19`\x01\x83aU\xDBV[\x85T\x90\x91P_\x90aE,\x90`\x01\x90aU\xDBV[\x90P\x81\x81\x14aE\x8BW_\x86_\x01\x82\x81T\x81\x10aEJWaEJaU\x03V[\x90_R` _ \x01T\x90P\x80\x87_\x01\x84\x81T\x81\x10aEjWaEjaU\x03V[_\x91\x82R` \x80\x83 \x90\x91\x01\x92\x90\x92U\x91\x82R`\x01\x88\x01\x90R`@\x90 \x83\x90U[\x85T\x86\x90\x80aE\x9CWaE\x9Ca[\x03V[`\x01\x90\x03\x81\x81\x90_R` _ \x01_\x90U\x90U\x85`\x01\x01_\x86\x81R` \x01\x90\x81R` \x01_ _\x90U`\x01\x93PPPPa\x08\x10V[_\x91PPa\x08\x10V[\x82T_\x90\x81aE\xEB\x86\x86\x83\x85aH;V[\x90P\x80\x15aF%WaF\x0F\x86aF\x02`\x01\x84aU\xDBV[_\x91\x82R` \x90\x91 \x01\x90V[T`\x01` \x1B\x90\x04`\x01`\x01`\xE0\x1B\x03\x16a\x07\x8FV[P\x91\x94\x93PPPPV[\x81T_\x90\x80\x15aF^WaFH\x84aF\x02`\x01\x84aU\xDBV[T`\x01` \x1B\x90\x04`\x01`\x01`\xE0\x1B\x03\x16a\x08\x0CV[P\x90\x92\x91PPV[\x82T\x80\x15aG\x1CW_aF~\x85aF\x02`\x01\x85aU\xDBV[`@\x80Q\x80\x82\x01\x90\x91R\x90Tc\xFF\xFF\xFF\xFF\x80\x82\x16\x80\x84R`\x01` \x1B\x90\x92\x04`\x01`\x01`\xE0\x1B\x03\x16` \x84\x01R\x91\x92P\x90\x85\x16\x10\x15aF\xD0W`@Qc\x15\x1B\x8E?`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80Qc\xFF\xFF\xFF\xFF\x80\x86\x16\x91\x16\x03aG\x1AW\x82aF\xF1\x86aF\x02`\x01\x86aU\xDBV[\x80T`\x01`\x01`\xE0\x1B\x03\x92\x90\x92\x16`\x01` \x1B\x02c\xFF\xFF\xFF\xFF\x90\x92\x16\x91\x90\x91\x17\x90UPPPPPV[P[P`@\x80Q\x80\x82\x01\x90\x91Rc\xFF\xFF\xFF\xFF\x92\x83\x16\x81R`\x01`\x01`\xE0\x1B\x03\x91\x82\x16` \x80\x83\x01\x91\x82R\x85T`\x01\x81\x01\x87U_\x96\x87R\x95 \x91Q\x90Q\x90\x92\x16`\x01` \x1B\x02\x91\x90\x92\x16\x17\x91\x01UV[_`\x01`\x01`\xFF\x1B\x03\x82\x11\x15aC*W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`(`$\x82\x01R\x7FSafeCast: value doesn't fit in a`D\x82\x01Rg7\x104\xB7:\x19\x1A\x9B`\xC1\x1B`d\x82\x01R`\x84\x01a/9V[\x80`\x0F\x81\x90\x0B\x81\x14aH6W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`'`$\x82\x01R\x7FSafeCast: value doesn't fit in 1`D\x82\x01Rf28 bits`\xC8\x1B`d\x82\x01R`\x84\x01a/9V[\x91\x90PV[_[\x81\x83\x10\x15a\t\xFFW_aHP\x84\x84aH\x8EV[_\x87\x81R` \x90 \x90\x91Pc\xFF\xFF\xFF\xFF\x86\x16\x90\x82\x01Tc\xFF\xFF\xFF\xFF\x16\x11\x15aHzW\x80\x92PaH\x88V[aH\x85\x81`\x01aZ\xC9V[\x93P[PaH=V[_aH\x9C`\x02\x84\x84\x18a[\x17V[a\x07\x94\x90\x84\x84\x16aZ\xC9V[`@\x80Q``\x81\x01\x82R_\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x91\x90\x91R\x90V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a1]W__\xFD[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[`@Q``\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15aI\x11WaI\x11aH\xDBV[`@R\x90V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15aI?WaI?aH\xDBV[`@R\x91\x90PV[\x805c\xFF\xFF\xFF\xFF\x81\x16\x81\x14aH6W__\xFD[_`@\x82\x84\x03\x12\x15aIjW__\xFD[`@\x80Q\x90\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15aI\x8CWaI\x8CaH\xDBV[`@R\x90P\x80\x825aI\x9D\x81aH\xC7V[\x81RaI\xAB` \x84\x01aIGV[` \x82\x01RP\x92\x91PPV[___`\x80\x84\x86\x03\x12\x15aI\xC9W__\xFD[\x835aI\xD4\x81aH\xC7V[\x92PaI\xE3\x85` \x86\x01aIZV[\x91P``\x84\x015aI\xF3\x81aH\xC7V[\x80\x91PP\x92P\x92P\x92V[\x80Q`\x01`\x01`@\x1B\x03\x16\x82R` \x80\x82\x01Q`\x0F\x0B\x90\x83\x01R`@\x90\x81\x01Qc\xFF\xFF\xFF\xFF\x16\x91\x01RV[``\x81\x01a\x08\x10\x82\x84aI\xFEV[__``\x83\x85\x03\x12\x15aJHW__\xFD[\x825aJS\x81aH\xC7V[\x91PaJb\x84` \x85\x01aIZV[\x90P\x92P\x92\x90PV[_` \x82\x84\x03\x12\x15aJ{W__\xFD[P5\x91\x90PV[_` \x82\x84\x03\x12\x15aJ\x92W__\xFD[\x815a\x07\x94\x81aH\xC7V[\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x82R` \x90\x81\x01Qc\xFF\xFF\xFF\xFF\x16\x91\x01RV[_\x81Q\x80\x84R` \x84\x01\x93P` \x83\x01_[\x82\x81\x10\x15aJ\xF6WaJ\xE0\x86\x83QaJ\x9DV[`@\x95\x90\x95\x01\x94` \x91\x90\x91\x01\x90`\x01\x01aJ\xCDV[P\x93\x94\x93PPPPV[` \x81R_a\x07\x94` \x83\x01\x84aJ\xBBV[_`@\x82\x84\x03\x12\x15aK\"W__\xFD[a\x07\x94\x83\x83aIZV[__\x83`\x1F\x84\x01\x12aK<W__\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15aKRW__\xFD[` \x83\x01\x91P\x83` \x82`\x05\x1B\x85\x01\x01\x11\x15a\x17?W__\xFD[___`@\x84\x86\x03\x12\x15aK~W__\xFD[\x835aK\x89\x81aH\xC7V[\x92P` \x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aK\xA3W__\xFD[aK\xAF\x86\x82\x87\x01aK,V[\x94\x97\x90\x96P\x93\x94PPPPV[_`\x01`\x01`@\x1B\x03\x82\x11\x15aK\xD4WaK\xD4aH\xDBV[P`\x05\x1B` \x01\x90V[_\x82`\x1F\x83\x01\x12aK\xEDW__\xFD[\x815aL\0aK\xFB\x82aK\xBCV[aI\x17V[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x86\x01\x01\x92P\x85\x83\x11\x15aL!W__\xFD[` \x85\x01[\x83\x81\x10\x15aLGW\x805aL9\x81aH\xC7V[\x83R` \x92\x83\x01\x92\x01aL&V[P\x95\x94PPPPPV[___`\x80\x84\x86\x03\x12\x15aLcW__\xFD[aLm\x85\x85aIZV[\x92P`@\x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aL\x87W__\xFD[aL\x93\x86\x82\x87\x01aK\xDEV[\x92PP``\x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aL\xAEW__\xFD[aL\xBA\x86\x82\x87\x01aK\xDEV[\x91PP\x92P\x92P\x92V[_\x81Q\x80\x84R` \x84\x01\x93P` \x83\x01_[\x82\x81\x10\x15aJ\xF6W\x81Q\x86R` \x95\x86\x01\x95\x90\x91\x01\x90`\x01\x01aL\xD6V[_` \x82\x01` \x83R\x80\x84Q\x80\x83R`@\x85\x01\x91P`@\x81`\x05\x1B\x86\x01\x01\x92P` \x86\x01_[\x82\x81\x10\x15aMKW`?\x19\x87\x86\x03\x01\x84RaM6\x85\x83QaL\xC4V[\x94P` \x93\x84\x01\x93\x91\x90\x91\x01\x90`\x01\x01aM\x1AV[P\x92\x96\x95PPPPPPV[____`\xA0\x85\x87\x03\x12\x15aMjW__\xFD[aMt\x86\x86aIZV[\x93P`@\x85\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aM\x8EW__\xFD[aM\x9A\x87\x82\x88\x01aK\xDEV[\x93PP``\x85\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aM\xB5W__\xFD[aM\xC1\x87\x82\x88\x01aK\xDEV[\x92PPaM\xD0`\x80\x86\x01aIGV[\x90P\x92\x95\x91\x94P\x92PV[`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x81R` \x01\x90V[__`@\x83\x85\x03\x12\x15aN\0W__\xFD[\x825aN\x0B\x81aH\xC7V[\x91P` \x83\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aN%W__\xFD[\x83\x01`\xA0\x81\x86\x03\x12\x15aN6W__\xFD[\x80\x91PP\x92P\x92\x90PV[__`@\x83\x85\x03\x12\x15aNRW__\xFD[\x825aN]\x81aH\xC7V[\x91P` \x83\x015aN6\x81aH\xC7V[_\x81Q\x80\x84R` \x84\x01\x93P` \x83\x01_[\x82\x81\x10\x15aJ\xF6WaN\x92\x86\x83QaI\xFEV[``\x95\x90\x95\x01\x94` \x91\x90\x91\x01\x90`\x01\x01aN\x7FV[`@\x81R_aN\xBA`@\x83\x01\x85aJ\xBBV[\x82\x81\x03` \x84\x01Ra\x1C\xDB\x81\x85aNmV[_\x81Q\x80\x84R` \x84\x01\x93P` \x83\x01_[\x82\x81\x10\x15aJ\xF6W\x81Q`\x01`\x01`\xA0\x1B\x03\x16\x86R` \x95\x86\x01\x95\x90\x91\x01\x90`\x01\x01aN\xDEV[` \x81R_a\x07\x94` \x83\x01\x84aN\xCCV[__`@\x83\x85\x03\x12\x15aO(W__\xFD[\x825`\x01`\x01`@\x1B\x03\x81\x11\x15aO=W__\xFD[aOI\x85\x82\x86\x01aK\xDEV[\x92PP` \x83\x015aN6\x81aH\xC7V[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R_\x91\x84\x01\x90`@\x84\x01\x90\x83[\x81\x81\x10\x15aO\x9AW\x83Q`\x01`\x01`@\x1B\x03\x16\x83R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01aOsV[P\x90\x95\x94PPPPPV[_____``\x86\x88\x03\x12\x15aO\xB9W__\xFD[\x855aO\xC4\x81aH\xC7V[\x94P` \x86\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aO\xDEW__\xFD[aO\xEA\x88\x82\x89\x01aK,V[\x90\x95P\x93PP`@\x86\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aP\x08W__\xFD[aP\x14\x88\x82\x89\x01aK,V[\x96\x99\x95\x98P\x93\x96P\x92\x94\x93\x92PPPV[____``\x85\x87\x03\x12\x15aP8W__\xFD[\x845aPC\x81aH\xC7V[\x93PaPQ` \x86\x01aIGV[\x92P`@\x85\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aPkW__\xFD[aPw\x87\x82\x88\x01aK,V[\x95\x98\x94\x97P\x95PPPPV[__`@\x83\x85\x03\x12\x15aP\x94W__\xFD[\x825aP\x9F\x81aH\xC7V[\x91P` \x83\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aP\xB9W__\xFD[a?\xB7\x85\x82\x86\x01aK\xDEV[` \x81R_\x82Q\x80` \x84\x01R_[\x81\x81\x10\x15aP\xF1W` \x81\x86\x01\x81\x01Q`@\x86\x84\x01\x01R\x01aP\xD4V[P_`@\x82\x85\x01\x01R`@`\x1F\x19`\x1F\x83\x01\x16\x84\x01\x01\x91PP\x92\x91PPV[__`@\x83\x85\x03\x12\x15aQ!W__\xFD[\x825aQ,\x81aH\xC7V[\x91PaJb` \x84\x01aIGV[_` \x82\x84\x03\x12\x15aQJW__\xFD[\x815`\xFF\x81\x16\x81\x14a\x07\x94W__\xFD[_``\x82\x84\x03\x12\x15aQjW__\xFD[P\x91\x90PV[_` \x82\x84\x03\x12\x15aQ\x80W__\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15aQ\x95W__\xFD[a\x08\x0C\x84\x82\x85\x01aQZV[___`\x80\x84\x86\x03\x12\x15aQ\xB3W__\xFD[\x835`\x01`\x01`@\x1B\x03\x81\x11\x15aQ\xC8W__\xFD[aQ\xD4\x86\x82\x87\x01aK\xDEV[\x93PPaI\xE3\x85` \x86\x01aIZV[` \x81R_a\x07\x94` \x83\x01\x84aNmV[___``\x84\x86\x03\x12\x15aR\x08W__\xFD[\x835aR\x13\x81aH\xC7V[\x92P` \x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aR-W__\xFD[aR9\x86\x82\x87\x01aK\xDEV[\x92PPaRH`@\x85\x01aIGV[\x90P\x92P\x92P\x92V[__`@\x83\x85\x03\x12\x15aRbW__\xFD[\x825aRm\x81aH\xC7V[\x91P` \x83\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aR\x87W__\xFD[\x83\x01`\x1F\x81\x01\x85\x13aR\x97W__\xFD[\x805aR\xA5aK\xFB\x82aK\xBCV[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x85\x01\x01\x92P\x87\x83\x11\x15aR\xC6W__\xFD[` \x84\x01[\x83\x81\x10\x15aS\xEBW\x805`\x01`\x01`@\x1B\x03\x81\x11\x15aR\xE8W__\xFD[\x85\x01`\x80\x81\x8B\x03`\x1F\x19\x01\x12\x15aR\xFDW__\xFD[aS\x05aH\xEFV[aS\x12\x8B` \x84\x01aIZV[\x81R``\x82\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aS,W__\xFD[aS;\x8C` \x83\x86\x01\x01aK\xDEV[` \x83\x01RP`\x80\x82\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aSYW__\xFD[` \x81\x84\x01\x01\x92PP\x8A`\x1F\x83\x01\x12aSpW__\xFD[\x815aS~aK\xFB\x82aK\xBCV[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x86\x01\x01\x92P\x8D\x83\x11\x15aS\x9FW__\xFD[` \x85\x01\x94P[\x82\x85\x10\x15aS\xD5W\x845`\x01`\x01`@\x1B\x03\x81\x16\x81\x14aS\xC4W__\xFD[\x82R` \x94\x85\x01\x94\x90\x91\x01\x90aS\xA6V[`@\x84\x01RPP\x84RP` \x92\x83\x01\x92\x01aR\xCBV[P\x80\x94PPPPP\x92P\x92\x90PV[___`@\x84\x86\x03\x12\x15aT\x0CW__\xFD[\x835aT\x17\x81aH\xC7V[\x92P` \x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aT1W__\xFD[\x84\x01`\x1F\x81\x01\x86\x13aTAW__\xFD[\x805`\x01`\x01`@\x1B\x03\x81\x11\x15aTVW__\xFD[\x86` \x82\x84\x01\x01\x11\x15aTgW__\xFD[\x93\x96` \x91\x90\x91\x01\x95P\x92\x93PPPV[__`@\x83\x85\x03\x12\x15aT\x89W__\xFD[\x825aT\x94\x81aH\xC7V[\x91P` \x83\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aT\xAEW__\xFD[a?\xB7\x85\x82\x86\x01aQZV[__`@\x83\x85\x03\x12\x15aT\xCBW__\xFD[\x825aT\xD6\x81aH\xC7V[\x94` \x93\x90\x93\x015\x93PPPV[_` \x82\x84\x03\x12\x15aT\xF4W__\xFD[\x81Q\x80\x15\x15\x81\x14a\x07\x94W__\xFD[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[_\x825`>\x19\x836\x03\x01\x81\x12aU+W__\xFD[\x91\x90\x91\x01\x92\x91PPV[_` \x82\x84\x03\x12\x15aUEW__\xFD[a\x07\x94\x82aIGV[`@\x81\x01a\x08\x10\x82\x84aJ\x9DV[__\x835`\x1E\x19\x846\x03\x01\x81\x12aUqW__\xFD[\x83\x01\x805\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15aU\x8AW__\xFD[` \x01\x91P`\x05\x81\x90\x1B6\x03\x82\x13\x15a\x17?W__\xFD[``\x81\x01aU\xAF\x82\x85aJ\x9DV[`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16`@\x91\x90\x91\x01R\x91\x90PV[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[\x81\x81\x03\x81\x81\x11\x15a\x08\x10Wa\x08\x10aU\xC7V[`\x01`\x01`@\x1B\x03\x82\x81\x16\x82\x82\x16\x03\x90\x81\x11\x15a\x08\x10Wa\x08\x10aU\xC7V[_`\x0F\x82\x90\x0B`\x01`\x7F\x1B\x81\x01aV&WaV&aU\xC7V[_\x03\x92\x91PPV[`\x0F\x81\x81\x0B\x90\x83\x90\x0B\x01`\x01`\x01`\x7F\x1B\x03\x81\x13`\x01`\x01`\x7F\x1B\x03\x19\x82\x12\x17\x15a\x08\x10Wa\x08\x10aU\xC7V[`\x01`\x01`\xA0\x1B\x03\x86\x16\x81R`\xC0\x81\x01aVx` \x83\x01\x87aJ\x9DV[`\x01`\x01`\xA0\x1B\x03\x94\x90\x94\x16``\x82\x01R`\x01`\x01`@\x1B\x03\x92\x90\x92\x16`\x80\x83\x01Rc\xFF\xFF\xFF\xFF\x16`\xA0\x90\x91\x01R\x92\x91PPV[__\x835`\x1E\x19\x846\x03\x01\x81\x12aV\xC1W__\xFD[\x83\x01\x805\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15aV\xDAW__\xFD[` \x01\x91P6\x81\x90\x03\x82\x13\x15a\x17?W__\xFD[\x81\x83R\x81\x81` \x85\x017P_\x82\x82\x01` \x90\x81\x01\x91\x90\x91R`\x1F\x90\x91\x01`\x1F\x19\x16\x90\x91\x01\x01\x90V[`\x01`\x01`\xA0\x1B\x03\x88\x16\x81R_`\xC0\x82\x01aW4` \x84\x01\x8AaJ\x9DV[`\xC0``\x84\x01R\x86\x90R\x86`\xE0\x83\x01_[\x88\x81\x10\x15aWuW\x825aWX\x81aH\xC7V[`\x01`\x01`\xA0\x1B\x03\x16\x82R` \x92\x83\x01\x92\x90\x91\x01\x90`\x01\x01aWEV[P\x83\x81\x03`\x80\x85\x01RaW\x88\x81\x88aL\xC4V[\x91PP\x82\x81\x03`\xA0\x84\x01RaW\x9E\x81\x85\x87aV\xEEV[\x9A\x99PPPPPPPPPPV[_` \x82\x84\x03\x12\x15aW\xBCW__\xFD[\x815a\xFF\xFF\x81\x16\x81\x14a\x07\x94W__\xFD[c\xFF\xFF\xFF\xFF\x81\x81\x16\x83\x82\x16\x01\x90\x81\x11\x15a\x08\x10Wa\x08\x10aU\xC7V[\x81\x83R` \x83\x01\x92P_\x81_[\x84\x81\x10\x15aJ\xF6Wc\xFF\xFF\xFF\xFFaX\x0C\x83aIGV[\x16\x86R` \x95\x86\x01\x95\x91\x90\x91\x01\x90`\x01\x01aW\xF6V[`\x01`\x01`\xA0\x1B\x03\x85\x81\x16\x82R\x84\x16` \x82\x01R```@\x82\x01\x81\x90R_\x90aA\xD4\x90\x83\x01\x84\x86aW\xE9V[` \x81R_a\r!` \x83\x01\x84\x86aV\xEEV[`\x01`\x01`\xA0\x1B\x03\x87\x81\x16\x82R\x86\x16` \x82\x01R`\x80`@\x82\x01\x81\x90R_\x90aX\x8D\x90\x83\x01\x86\x88aW\xE9V[\x82\x81\x03``\x84\x01RaX\xA0\x81\x85\x87aV\xEEV[\x99\x98PPPPPPPPPV[_` \x82\x84\x03\x12\x15aX\xBDW__\xFD[\x81Qa\x07\x94\x81aH\xC7V[\x80Q` \x80\x83\x01Q\x91\x90\x81\x10\x15aQjW_\x19` \x91\x90\x91\x03`\x03\x1B\x1B\x16\x91\x90PV[`@\x81R_aX\xFD`@\x83\x01\x85aN\xCCV[\x82\x81\x03` \x84\x01Ra\x1C\xDB\x81\x85aN\xCCV[_` \x82\x84\x03\x12\x15aY\x1FW__\xFD[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15aY4W__\xFD[\x82\x01`\x1F\x81\x01\x84\x13aYDW__\xFD[\x80QaYRaK\xFB\x82aK\xBCV[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x85\x01\x01\x92P\x86\x83\x11\x15aYsW__\xFD[` \x84\x01[\x83\x81\x10\x15aZ\x10W\x80Q`\x01`\x01`@\x1B\x03\x81\x11\x15aY\x95W__\xFD[\x85\x01`?\x81\x01\x89\x13aY\xA5W__\xFD[` \x81\x01QaY\xB6aK\xFB\x82aK\xBCV[\x80\x82\x82R` \x82\x01\x91P` \x80\x84`\x05\x1B\x86\x01\x01\x01\x92P\x8B\x83\x11\x15aY\xD9W__\xFD[`@\x84\x01\x93P[\x82\x84\x10\x15aY\xFBW\x83Q\x82R` \x93\x84\x01\x93\x90\x91\x01\x90aY\xE0V[\x86RPP` \x93\x84\x01\x93\x91\x90\x91\x01\x90PaYxV[P\x96\x95PPPPPPV[`\x01`\x01`\xA0\x1B\x03\x93\x84\x16\x81R\x91\x90\x92\x16` \x82\x01R`\x01`\x01`@\x1B\x03\x90\x91\x16`@\x82\x01R``\x01\x90V[_`\x01\x82\x01aZXWaZXaU\xC7V[P`\x01\x01\x90V[_\x81aZmWaZmaU\xC7V[P_\x19\x01\x90V[`\x0F\x82\x81\x0B\x90\x82\x90\x0B\x03`\x01`\x01`\x7F\x1B\x03\x19\x81\x12`\x01`\x01`\x7F\x1B\x03\x82\x13\x17\x15a\x08\x10Wa\x08\x10aU\xC7V[cNH{q`\xE0\x1B_R`!`\x04R`$_\xFD[cNH{q`\xE0\x1B_R`\x12`\x04R`$_\xFD[\x80\x82\x01\x80\x82\x11\x15a\x08\x10Wa\x08\x10aU\xC7V[\x80\x82\x01\x82\x81\x12_\x83\x12\x80\x15\x82\x16\x82\x15\x82\x16\x17\x15aZ\xFBWaZ\xFBaU\xC7V[PP\x92\x91PPV[cNH{q`\xE0\x1B_R`1`\x04R`$_\xFD[_\x82a[1WcNH{q`\xE0\x1B_R`\x12`\x04R`$_\xFD[P\x04\x90V\xFE\x14\x87\xAFT\x18\xC4~\xE5\xEAE\xEFJ\x939\x86h\x12\x08\x90wJ\x9E\x13H~a\xE9\xDC;\xAFv\xDD\xA2dipfsX\"\x12 d\xA2\xCBnA\xF7\xF8\xBD))|\x0E\xEF`\x0B\xAE\x9A\xFE\xB2\xECA\xD7\xD1\xF5TJ\xCA\x9F\xE1}F\x99dsolcC\0\x08\x1B\x003",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x608060405234801561000f575f5ffd5b50600436106102a2575f3560e01c80636cfb44811161016c578063a9821821116100d5578063c221d8ae1161008f578063c221d8ae146106da578063cd6dc687146106ed578063d3d96ff414610700578063df5cf72314610713578063f2fde38b1461073a578063f605ce081461074d578063fabc1cbc14610760575f5ffd5b8063a98218211461064c578063adc2e3d91461065f578063b2447af714610672578063b66bd98914610685578063b9fbaed114610698578063ba1a84e5146106c7575f5ffd5b8063886f119511610126578063886f1195146105bb5780638ce64854146105e25780638da5cb5b1461060257806394d7d00c14610613578063952899ee14610626578063a9333ec814610639575f5ffd5b80636cfb4481146105285780636e3492b5146105535780636e875dba14610566578063715018a61461057957806379ae50cd146105815780637bc1ef6114610594575f5ffd5b80634177a87c1161020e57806354fd4d50116101c857806354fd4d50146104b057806356c483e6146104c5578063595c6a67146104d85780635ac86ab7146104e05780635c975abb14610503578063670d3ba214610515575f5ffd5b80634177a87c146104105780634657e26a146104305780634a10ffe5146104575780634b5046ef1461047757806350feea201461048a578063547afb871461049d575f5ffd5b80632981eb771161025f5780632981eb771461034d5780632b453a9a146103895780632bab2c4a146103a9578063304c10cd146103bc57806336352057146103dc57806340120dab146103ef575f5ffd5b806310e1b9b8146102a65780631352c3e6146102cf578063136439dd146102f257806315fe502814610307578063260dc75814610327578063261f84e01461033a575b5f5ffd5b6102b96102b43660046149b7565b610773565b6040516102c69190614a29565b60405180910390f35b6102e26102dd366004614a37565b61079b565b60405190151581526020016102c6565b610305610300366004614a6b565b610816565b005b61031a610315366004614a82565b6108f0565b6040516102c69190614b00565b6102e2610335366004614b12565b610a07565b610305610348366004614b6c565b610a38565b6103747f000000000000000000000000000000000000000000000000000000000000000081565b60405163ffffffff90911681526020016102c6565b61039c610397366004614c51565b610d13565b6040516102c69190614cf4565b61039c6103b7366004614d57565b610d29565b6103cf6103ca366004614a82565b610dc8565b6040516102c69190614ddb565b6103056103ea366004614def565b610df7565b6104026103fd366004614e41565b6115db565b6040516102c6929190614ea8565b61042361041e366004614b12565b611746565b6040516102c69190614f05565b6103cf7f000000000000000000000000000000000000000000000000000000000000000081565b61046a610465366004614f17565b61176a565b6040516102c69190614f5a565b610305610485366004614fa5565b611812565b610305610498366004615025565b6118cc565b61046a6104ab366004615083565b611a2a565b6104b8611ad2565b6040516102c691906150c5565b6103056104d3366004615110565b611b02565b610305611c0a565b6102e26104ee36600461513a565b606654600160ff9092169190911b9081161490565b6066545b6040519081526020016102c6565b6102e2610523366004614a37565b611cbe565b61053b610536366004614e41565b611ccf565b6040516001600160401b0390911681526020016102c6565b610305610561366004615170565b611ce4565b610423610574366004614b12565b6120c5565b6103056120d6565b61031a61058f366004614a82565b6120e7565b6103747f000000000000000000000000000000000000000000000000000000000000000081565b6103cf7f000000000000000000000000000000000000000000000000000000000000000081565b6105f56105f03660046151a1565b6121c1565b6040516102c691906151e4565b6033546001600160a01b03166103cf565b61046a6106213660046151f6565b61226d565b610305610634366004615251565b612359565b61053b610647366004614e41565b612805565b61030561065a3660046153fa565b612834565b61030561066d366004615478565b6128e6565b610507610680366004614b12565b612c46565b610305610693366004615025565b612c68565b6106ab6106a6366004614a82565b612dc2565b60408051921515835263ffffffff9091166020830152016102c6565b6105076106d5366004614a82565b612e5a565b6104236106e8366004614a37565b612e7a565b6103056106fb3660046154ba565b612ea3565b61030561070e366004614e41565b612fc0565b6103cf7f000000000000000000000000000000000000000000000000000000000000000081565b610305610748366004614a82565b6130e7565b61053b61075b366004614e41565b613160565b61030561076e366004614a6b565b61316c565b61077b6148a8565b5f61078f8561078986613282565b856132e5565b925050505b9392505050565b6001600160a01b0382165f908152609e602052604081208190816107be85613282565b815260208082019290925260409081015f2081518083019092525460ff8116151580835261010090910463ffffffff169282019290925291508061080c5750806020015163ffffffff164311155b9150505b92915050565b60405163237dfb4760e11b81526001600160a01b037f000000000000000000000000000000000000000000000000000000000000000016906346fbf68e90610862903390600401614ddb565b602060405180830381865afa15801561087d573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906108a191906154e4565b6108be57604051631d77d47760e21b815260040160405180910390fd5b60665481811681146108e35760405163c61dca5d60e01b815260040160405180910390fd5b6108ec82613445565b5050565b6001600160a01b0381165f908152609d602052604081206060919061091490613482565b90505f816001600160401b0381111561092f5761092f6148db565b60405190808252806020026020018201604052801561097357816020015b604080518082019091525f808252602082015281526020019060019003908161094d5790505b5090505f5b828110156109ff576001600160a01b0385165f908152609d602052604090206109da906109a5908361348b565b604080518082019091525f80825260208201525060408051808201909152606082901c815263ffffffff909116602082015290565b8282815181106109ec576109ec615503565b6020908102919091010152600101610978565b509392505050565b60208082015182516001600160a01b03165f9081526098909252604082206108109163ffffffff9081169061349616565b82610a42816134ad565b610a5f5760405163932d94f760e01b815260040160405180910390fd5b6001600160a01b0384165f90815260a4602052604090205460ff16610a97576040516348f7dbb960e01b815260040160405180910390fd5b5f5b82811015610d0c575f6040518060400160405280876001600160a01b03168152602001868685818110610ace57610ace615503565b9050602002810190610ae09190615517565b610aee906020810190615535565b63ffffffff168152509050610b38816020015163ffffffff1660985f896001600160a01b03166001600160a01b031681526020019081526020015f2061355790919063ffffffff16565b610b5557604051631fb1705560e21b815260040160405180910390fd5b7f31629285ead2335ae0933f86ed2ae63321f7af77b4e6eaabc42c057880977e6c6040518060400160405280886001600160a01b03168152602001836020015163ffffffff16815250604051610bab919061554e565b60405180910390a15f610bbd82613282565b90505f5b868685818110610bd357610bd3615503565b9050602002810190610be59190615517565b610bf390602081019061555c565b9050811015610d0157610c69878786818110610c1157610c11615503565b9050602002810190610c239190615517565b610c3190602081019061555c565b83818110610c4157610c41615503565b9050602002016020810190610c569190614a82565b5f84815260996020526040902090613562565b507f7ab260fe0af193db5f4986770d831bda4ea46099dc817e8b6716dcae8af8e88b83888887818110610c9e57610c9e615503565b9050602002810190610cb09190615517565b610cbe90602081019061555c565b84818110610cce57610cce615503565b9050602002016020810190610ce39190614a82565b604051610cf19291906155a1565b60405180910390a1600101610bc1565b505050600101610a99565b5050505050565b6060610d2184848443613576565b949350505050565b6060610d3785858585613576565b90505f5b8451811015610dbf57610d67858281518110610d5957610d59615503565b60200260200101518761079b565b610db7575f5b8451811015610db5575f838381518110610d8957610d89615503565b60200260200101518281518110610da257610da2615503565b6020908102919091010152600101610d6d565b505b600101610d3b565b50949350505050565b6001600160a01b038082165f908152609760205260408120549091168015610df05780610794565b5090919050565b606654600190600290811603610e205760405163840a48d560e01b815260040160405180910390fd5b82610e2a816134ad565b610e475760405163932d94f760e01b815260040160405180910390fd5b5f6040518060400160405280866001600160a01b03168152602001856020016020810190610e759190615535565b63ffffffff1690529050610e8c606085018561555c565b9050610e9b604086018661555c565b905014610ebb576040516343714afd60e01b815260040160405180910390fd5b60208082015182516001600160a01b03165f90815260989092526040909120610eed9163ffffffff9081169061349616565b610f0a57604051631fb1705560e21b815260040160405180910390fd5b610f20610f1a6020860186614a82565b8261079b565b610f3d5760405163ebbff49760e01b815260040160405180910390fd5b5f610f4b604086018661555c565b90506001600160401b03811115610f6457610f646148db565b604051908082528060200260200182016040528015610f8d578160200160208202803683370190505b5090505f5b610f9f604087018761555c565b905081101561156d578015806110325750610fbd604087018761555c565b610fc86001846155db565b818110610fd757610fd7615503565b9050602002016020810190610fec9190614a82565b6001600160a01b0316611002604088018861555c565b8381811061101257611012615503565b90506020020160208101906110279190614a82565b6001600160a01b0316115b61104f57604051639f1c805360e01b815260040160405180910390fd5b61105c606087018761555c565b8281811061106c5761106c615503565b905060200201355f1080156110ac5750670de0b6b3a7640000611092606088018861555c565b838181106110a2576110a2615503565b9050602002013511155b6110c957604051631353603160e01b815260040160405180910390fd5b6111256110d9604088018861555c565b838181106110e9576110e9615503565b90506020020160208101906110fe9190614a82565b60995f61110a87613282565b81526020019081526020015f2061386390919063ffffffff16565b611142576040516331bc342760e11b815260040160405180910390fd5b5f8061119461115460208a018a614a82565b61115d87613282565b61116a60408c018c61555c565b8781811061117a5761117a615503565b905060200201602081019061118f9190614a82565b6132e5565b805191935091506001600160401b03165f036111b1575050611565565b5f6111ec6111c260608b018b61555c565b868181106111d2576111d2615503565b85516001600160401b031692602090910201359050613884565b83519091506112076001600160401b0380841690831661389a565b86868151811061121957611219615503565b60200260200101818152505081835f0181815161123691906155ee565b6001600160401b03169052508351829085906112539083906155ee565b6001600160401b03169052506020840180518391906112739083906155ee565b6001600160401b031690525060208301515f600f9190910b1215611378575f6112d66112a260608d018d61555c565b888181106112b2576112b2615503565b9050602002013585602001516112c79061560d565b6001600160801b031690613884565b9050806001600160401b0316846020018181516112f3919061562e565b600f0b9052505f516020615b375f395f51905f5261131460208d018d614a82565b8961132260408f018f61555c565b8a81811061133257611332615503565b90506020020160208101906113479190614a82565b611358885f015189602001516138ae565b886040015160405161136e95949392919061565b565b60405180910390a1505b6113ca61138860208c018c614a82565b61139189613282565b61139e60408e018e61555c565b898181106113ae576113ae615503565b90506020020160208101906113c39190614a82565b87876138cd565b5f516020615b375f395f51905f526113e560208c018c614a82565b886113f360408e018e61555c565b8981811061140357611403615503565b90506020020160208101906114189190614a82565b865160405161142c9493929190439061565b565b60405180910390a161147d61144460208c018c614a82565b61145160408d018d61555c565b8881811061146157611461615503565b90506020020160208101906114769190614a82565b8651613b01565b6001600160a01b037f00000000000000000000000000000000000000000000000000000000000000001663601bb36f6114b960208d018d614a82565b6114c660408e018e61555c565b898181106114d6576114d6615503565b90506020020160208101906114eb9190614a82565b875160405160e085901b6001600160e01b03191681526001600160a01b0393841660048201529290911660248301526001600160401b0380861660448401521660648201526084015f604051808303815f87803b15801561154a575f5ffd5b505af115801561155c573d5f5f3e3d5ffd5b50505050505050505b600101610f92565b507f80969ad29428d6797ee7aad084f9e4a42a82fc506dcd2ca3b6fb431f85ccebe561159c6020870187614a82565b836115aa604089018961555c565b856115b860808c018c6156ac565b6040516115cb9796959493929190615716565b60405180910390a1505050505050565b6001600160a01b0382165f908152609d60205260408120606091829161160090613482565b90505f816001600160401b0381111561161b5761161b6148db565b60405190808252806020026020018201604052801561165f57816020015b604080518082019091525f80825260208201528152602001906001900390816116395790505b5090505f826001600160401b0381111561167b5761167b6148db565b6040519080825280602002602001820160405280156116b457816020015b6116a16148a8565b8152602001906001900390816116995790505b5090505f5b83811015611737576001600160a01b0388165f908152609d602052604081206116e6906109a5908461348b565b9050808483815181106116fb576116fb615503565b602002602001018190525061171189828a610773565b83838151811061172357611723615503565b6020908102919091010152506001016116b9565b509093509150505b9250929050565b60605f61079460995f61175886613282565b81526020019081526020015f20613b63565b60605f83516001600160401b03811115611786576117866148db565b6040519080825280602002602001820160405280156117af578160200160208202803683370190505b5090505f5b84518110156109ff576117e08582815181106117d2576117d2615503565b602002602001015185612805565b8282815181106117f2576117f2615503565b6001600160401b03909216602092830291909101909101526001016117b4565b6066545f9060019081160361183a5760405163840a48d560e01b815260040160405180910390fd5b83821461185a576040516343714afd60e01b815260040160405180910390fd5b5f5b848110156118c3576118bb8787878481811061187a5761187a615503565b905060200201602081019061188f9190614a82565b8686858181106118a1576118a1615503565b90506020020160208101906118b691906157ac565b613b6f565b60010161185c565b50505050505050565b836118d6816134ad565b6118f35760405163932d94f760e01b815260040160405180910390fd5b604080518082019091526001600160a01b038616815263ffffffff851660208201525f61191f82613282565b9050611960826020015163ffffffff1660985f8a6001600160a01b03166001600160a01b031681526020019081526020015f2061349690919063ffffffff16565b61197d57604051631fb1705560e21b815260040160405180910390fd5b5f5b84811015611a205761199c868683818110610c4157610c41615503565b6119b95760405163585cfb2f60e01b815260040160405180910390fd5b7f7ab260fe0af193db5f4986770d831bda4ea46099dc817e8b6716dcae8af8e88b838787848181106119ed576119ed615503565b9050602002016020810190611a029190614a82565b604051611a109291906155a1565b60405180910390a160010161197f565b5050505050505050565b60605f82516001600160401b03811115611a4657611a466148db565b604051908082528060200260200182016040528015611a6f578160200160208202803683370190505b5090505f5b83518110156109ff57611aa085858381518110611a9357611a93615503565b6020026020010151612805565b828281518110611ab257611ab2615503565b6001600160401b0390921660209283029190910190910152600101611a74565b6060611afd7f0000000000000000000000000000000000000000000000000000000000000000613c73565b905090565b336001600160a01b037f00000000000000000000000000000000000000000000000000000000000000001614611c0057611b3b826134ad565b611b58576040516348f5c3ed60e01b815260040160405180910390fd5b6040516336b87bd760e11b81526001600160a01b037f00000000000000000000000000000000000000000000000000000000000000001690636d70f7ae90611ba4908590600401614ddb565b602060405180830381865afa158015611bbf573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611be391906154e4565b611c005760405163ccea9e6f60e01b815260040160405180910390fd5b6108ec8282613cb0565b60405163237dfb4760e11b81526001600160a01b037f000000000000000000000000000000000000000000000000000000000000000016906346fbf68e90611c56903390600401614ddb565b602060405180830381865afa158015611c71573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611c9591906154e4565b611cb257604051631d77d47760e21b815260040160405180910390fd5b611cbc5f19613445565b565b5f61079483609a5f61110a86613282565b5f5f611cdb8484613e52565b95945050505050565b606654600290600490811603611d0d5760405163840a48d560e01b815260040160405180910390fd5b611d22611d1d6020840184614a82565b6134ad565b80611d3b5750611d3b611d1d6040840160208501614a82565b611d58576040516348f5c3ed60e01b815260040160405180910390fd5b5f5b611d67604084018461555c565b9050811015612029575f6040518060400160405280856020016020810190611d8f9190614a82565b6001600160a01b03168152602001611daa604087018761555c565b85818110611dba57611dba615503565b9050602002016020810190611dcf9190615535565b63ffffffff168152509050611e1c816020015163ffffffff1660985f876020016020810190611dfe9190614a82565b6001600160a01b0316815260208101919091526040015f2090613496565b611e3957604051631fb1705560e21b815260040160405180910390fd5b609e5f611e496020870187614a82565b6001600160a01b03166001600160a01b031681526020019081526020015f205f611e7283613282565b815260208101919091526040015f205460ff16611ea2576040516325131d4f60e01b815260040160405180910390fd5b611edc611eae82613282565b609c5f611ebe6020890189614a82565b6001600160a01b0316815260208101919091526040015f2090613fc1565b50611f14611eed6020860186614a82565b609a5f611ef985613282565b81526020019081526020015f20613fcc90919063ffffffff16565b50611f226020850185614a82565b6001600160a01b03167fad34c3070be1dffbcaa499d000ba2b8d9848aefcac3059df245dd95c4ece14fe82604051611f5a919061554e565b60405180910390a2604080518082019091525f815260208101611f9d7f0000000000000000000000000000000000000000000000000000000000000000436157cd565b63ffffffff169052609e5f611fb56020880188614a82565b6001600160a01b03166001600160a01b031681526020019081526020015f205f611fde84613282565b81526020808201929092526040015f2082518154939092015163ffffffff166101000264ffffffff00199215159290921664ffffffffff199093169290921717905550600101611d5a565b5061203d6103ca6040840160208501614a82565b6001600160a01b031663303ca9566120586020850185614a82565b6120686040860160208701614a82565b612075604087018761555c565b6040518563ffffffff1660e01b81526004016120949493929190615822565b5f604051808303815f87803b1580156120ab575f5ffd5b505af11580156120bd573d5f5f3e3d5ffd5b505050505050565b6060610810609a5f61175885613282565b6120de613fe0565b611cbc5f61403a565b6001600160a01b0381165f908152609c602052604081206060919061210b90613482565b90505f816001600160401b03811115612126576121266148db565b60405190808252806020026020018201604052801561216a57816020015b604080518082019091525f80825260208201528152602001906001900390816121445790505b5090505f5b828110156109ff576001600160a01b0385165f908152609c6020526040902061219c906109a5908361348b565b8282815181106121ae576121ae615503565b602090810291909101015260010161216f565b60605f84516001600160401b038111156121dd576121dd6148db565b60405190808252806020026020018201604052801561221657816020015b6122036148a8565b8152602001906001900390816121fb5790505b5090505f5b8551811015610dbf5761224886828151811061223957612239615503565b60200260200101518686610773565b82828151811061225a5761225a615503565b602090810291909101015260010161221b565b60605f83516001600160401b03811115612289576122896148db565b6040519080825280602002602001820160405280156122b2578160200160208202803683370190505b5090505f5b8451811015610dbf576001600160a01b0386165f90815260a1602052604081208651612327928792918990869081106122f2576122f2615503565b60200260200101516001600160a01b03166001600160a01b031681526020019081526020015f2061408b90919063ffffffff16565b82828151811061233957612339615503565b6001600160401b03909216602092830291909101909101526001016122b7565b6066545f906001908116036123815760405163840a48d560e01b815260040160405180910390fd5b61238a836134ad565b6123a7576040516348f5c3ed60e01b815260040160405180910390fd5b5f5f5f6123b386612dc2565b91509150816123d55760405163fa55fc8160e01b815260040160405180910390fd5b91505f90505b8351811015610d0c578381815181106123f6576123f6615503565b6020026020010151604001515184828151811061241557612415615503565b6020026020010151602001515114612440576040516343714afd60e01b815260040160405180910390fd5b5f84828151811061245357612453615503565b602090810291909101810151518082015181516001600160a01b03165f908152609890935260409092209092506124939163ffffffff9081169061349616565b6124b057604051631fb1705560e21b815260040160405180910390fd5b5f6124bb878361079b565b90505f5b8684815181106124d1576124d1615503565b602002602001015160200151518110156127fa575f8785815181106124f8576124f8615503565b602002602001015160200151828151811061251557612515615503565b6020026020010151905061252c898261ffff613b6f565b5f5f61253b8b61078988613282565b91509150806040015163ffffffff165f1461256957604051630d8fcbe360e41b815260040160405180910390fd5b5f6125768785848961409f565b90506125bb825f01518c8a8151811061259157612591615503565b60200260200101516040015187815181106125ae576125ae615503565b60200260200101516140d5565b600f0b602083018190525f036125e457604051634606179360e11b815260040160405180910390fd5b5f8260200151600f0b12156127285780156126aa5761266561260588613282565b6001600160a01b03808f165f90815260a360209081526040808320938a16835292905220908154600160801b90819004600f0b5f818152600180860160205260409091209390935583546001600160801b03908116939091011602179055565b61268f7f0000000000000000000000000000000000000000000000000000000000000000436157cd565b61269a9060016157cd565b63ffffffff166040830152612795565b6126bc836020015183602001516138ae565b6001600160401b031660208401528a518b90899081106126de576126de615503565b60200260200101516040015185815181106126fb576126fb615503565b6020908102919091018101516001600160401b031683525f9083015263ffffffff43166040830152612795565b5f8260200151600f0b131561279557612749836020015183602001516138ae565b6001600160401b03908116602085018190528451909116101561277f57604051636c9be0bf60e01b815260040160405180910390fd5b61278989436157cd565b63ffffffff1660408301525b6127aa8c6127a289613282565b8686866138cd565b5f516020615b375f395f51905f528c88866127cc865f015187602001516138ae565b86604001516040516127e295949392919061565b565b60405180910390a15050600190920191506124bf9050565b5050506001016123db565b6001600160a01b038083165f90815260a1602090815260408083209385168352929052908120610794906140ec565b8261283e816134ad565b61285b5760405163932d94f760e01b815260040160405180910390fd5b6001600160a01b0384165f90815260a4602052604090205460ff1661289d576001600160a01b0384165f90815260a460205260409020805460ff191660011790555b836001600160a01b03167fa89c1dc243d8908a96dd84944bcc97d6bc6ac00dd78e20621576be6a3c94371384846040516128d892919061584e565b60405180910390a250505050565b60665460029060049081160361290f5760405163840a48d560e01b815260040160405180910390fd5b82612919816134ad565b6129365760405163932d94f760e01b815260040160405180910390fd5b6040516336b87bd760e11b81526001600160a01b037f00000000000000000000000000000000000000000000000000000000000000001690636d70f7ae90612982908790600401614ddb565b602060405180830381865afa15801561299d573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906129c191906154e4565b6129de5760405163ccea9e6f60e01b815260040160405180910390fd5b5f5b6129ed602085018561555c565b9050811015612bb557604080518082019091525f9080612a106020880188614a82565b6001600160a01b03168152602001868060200190612a2e919061555c565b85818110612a3e57612a3e615503565b9050602002016020810190612a539190615535565b63ffffffff90811690915260208083015183516001600160a01b03165f90815260989092526040909120929350612a8f92919081169061349616565b612aac57604051631fb1705560e21b815260040160405180910390fd5b612ab6868261079b565b15612ad457604051636c6c6e2760e11b815260040160405180910390fd5b612afd612ae082613282565b6001600160a01b0388165f908152609c6020526040902090613557565b50612b2986609a5f612b0e85613282565b81526020019081526020015f2061356290919063ffffffff16565b50856001600160a01b03167f43232edf9071753d2321e5fa7e018363ee248e5f2142e6c08edd3265bfb4895e82604051612b63919061554e565b60405180910390a26001600160a01b0386165f908152609e60205260408120600191612b8e84613282565b815260208101919091526040015f20805460ff1916911515919091179055506001016129e0565b50612bc66103ca6020850185614a82565b6001600160a01b031663c63fd50285612be26020870187614a82565b612bef602088018861555c565b612bfc60408a018a6156ac565b6040518763ffffffff1660e01b8152600401612c1d96959493929190615861565b5f604051808303815f87803b158015612c34575f5ffd5b505af1158015611a20573d5f5f3e3d5ffd5b5f610810609a5f612c5685613282565b81526020019081526020015f20613482565b83612c72816134ad565b612c8f5760405163932d94f760e01b815260040160405180910390fd5b6040805180820182526001600160a01b03871680825263ffffffff80881660208085018290525f93845260989052939091209192612cce929161349616565b612ceb57604051631fb1705560e21b815260040160405180910390fd5b5f612cf582613282565b90505f5b84811015611a2057612d3e868683818110612d1657612d16615503565b9050602002016020810190612d2b9190614a82565b5f84815260996020526040902090613fcc565b612d5b576040516331bc342760e11b815260040160405180910390fd5b7f7b4b073d80dcac55a11177d8459ad9f664ceeb91f71f27167bb14f8152a7eeee83878784818110612d8f57612d8f615503565b9050602002016020810190612da49190614a82565b604051612db29291906155a1565b60405180910390a1600101612cf9565b6001600160a01b0381165f908152609b602090815260408083208151608081018352905463ffffffff80821680845260ff600160201b8404161515958401869052600160281b8304821694840194909452600160481b909104166060820181905284939192919015801590612e415750826060015163ffffffff164310155b15612e50575050604081015160015b9590945092505050565b6001600160a01b0381165f90815260986020526040812061081090613482565b6001600160a01b0382165f908152609f602052604081206060919061080c908261175886613282565b5f54610100900460ff1615808015612ec157505f54600160ff909116105b80612eda5750303b158015612eda57505f5460ff166001145b612f425760405162461bcd60e51b815260206004820152602e60248201527f496e697469616c697a61626c653a20636f6e747261637420697320616c72656160448201526d191e481a5b9a5d1a585b1a5e995960921b60648201526084015b60405180910390fd5b5f805460ff191660011790558015612f63575f805461ff0019166101001790555b612f6c82613445565b612f758361403a565b8015612fbb575f805461ff0019169055604051600181527f7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb3847402498906020015b60405180910390a15b505050565b81612fca816134ad565b612fe75760405163932d94f760e01b815260040160405180910390fd5b60405163b526578760e01b81526001600160a01b0383169063b526578790613013908690600401614ddb565b602060405180830381865afa15801561302e573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061305291906154e4565b61306f57604051631d0b13c160e31b815260040160405180910390fd5b6001600160a01b038381165f90815260976020526040902080546001600160a01b0319169184169190911790557f2ae945c40c44dc0ec263f95609c3fdc6952e0aefa22d6374e44f2c997acedf85836130c781610dc8565b604080516001600160a01b03938416815292909116602083015201612fb2565b6130ef613fe0565b6001600160a01b0381166131545760405162461bcd60e51b815260206004820152602660248201527f4f776e61626c653a206e6577206f776e657220697320746865207a65726f206160448201526564647265737360d01b6064820152608401612f39565b61315d8161403a565b50565b5f5f610dbf8484613e52565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031663eab66d7a6040518163ffffffff1660e01b8152600401602060405180830381865afa1580156131c8573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906131ec91906158ad565b6001600160a01b0316336001600160a01b03161461321d5760405163794821ff60e01b815260040160405180910390fd5b606654801982198116146132445760405163c61dca5d60e01b815260040160405180910390fd5b606682905560405182815233907f3582d1828e26bf56bd801502bc021ac0bc8afb57c826e4986b45593c8fad389c9060200160405180910390a25050565b5f815f0151826020015163ffffffff166040516020016132cd92919060609290921b6bffffffffffffffffffffffff1916825260a01b6001600160a01b031916601482015260200190565b604051602081830303815290604052610810906158c8565b604080518082019091525f80825260208201526133006148a8565b6040805180820182526001600160a01b038088165f90815260a160209081528482209288168252919091529182208190613339906140ec565b6001600160401b0390811682526001600160a01b038981165f81815260a260209081526040808320948c168084529482528083205486169682019690965291815260a082528481208b8252825284812092815291815290839020835160608101855290549283168152600160401b8304600f0b91810191909152600160c01b90910463ffffffff169181018290529192504310156133db57909250905061343d565b6133ec815f015182602001516138ae565b6001600160401b0316815260208101515f600f9190910b121561342a5761341b826020015182602001516138ae565b6001600160401b031660208301525b5f60408201819052602082015290925090505b935093915050565b606681905560405181815233907fab40a374bc51de372200a8bc981af8c9ecdc08dfdaef0bb6e09f88f3c616ef3d9060200160405180910390a250565b5f610810825490565b5f61079483836140ff565b5f8181526001830160205260408120541515610794565b604051631beb2b9760e31b81526001600160a01b0382811660048301523360248301523060448301525f80356001600160e01b0319166064840152917f00000000000000000000000000000000000000000000000000000000000000009091169063df595cb8906084016020604051808303815f875af1158015613533573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061081091906154e4565b5f6107948383614125565b5f610794836001600160a01b038416614125565b606083516001600160401b03811115613591576135916148db565b6040519080825280602002602001820160405280156135c457816020015b60608152602001906001900390816135af5790505b5090505f7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031663f0e0e67686866040518363ffffffff1660e01b81526004016136169291906158eb565b5f60405180830381865afa158015613630573d5f5f3e3d5ffd5b505050506040513d5f823e601f3d908101601f19168201604052613657919081019061590f565b90505f5b8551811015613859575f86828151811061367757613677615503565b6020026020010151905085516001600160401b0381111561369a5761369a6148db565b6040519080825280602002602001820160405280156136c3578160200160208202803683370190505b508483815181106136d6576136d6615503565b60209081029190910101525f5b865181101561384f575f8782815181106136ff576136ff615503565b6020908102919091018101516001600160a01b038086165f90815260a184526040808220928416825291909352822090925061373a906140ec565b9050806001600160401b03165f03613753575050613847565b5f61375f858d85610773565b90508863ffffffff16816040015163ffffffff161115801561378757505f8160200151600f0b125b156137a95761379d815f015182602001516138ae565b6001600160401b031681525b80515f906137c4906001600160401b0390811690851661389a565b905061380b818989815181106137dc576137dc615503565b602002602001015187815181106137f5576137f5615503565b602002602001015161417190919063ffffffff16565b89888151811061381d5761381d615503565b6020026020010151868151811061383657613836615503565b602002602001018181525050505050505b6001016136e3565b505060010161365b565b5050949350505050565b6001600160a01b0381165f9081526001830160205260408120541515610794565b5f6107948383670de0b6b3a76400006001614185565b5f61079483670de0b6b3a7640000846141de565b5f6107946138c5836001600160401b03861661562e565b600f0b6142c3565b6020808301516001600160a01b038088165f90815260a284526040808220928816825291909352909120546001600160401b0390811691161461398f57602082810180516001600160a01b038881165f90815260a28552604080822092891682529190945292839020805467ffffffffffffffff19166001600160401b039092169190911790555190517facf9095feb3a370c9cf692421c69ef320d4db5c66e6a7d29c7694eb02364fc55916139869188918791615a1b565b60405180910390a15b6001600160a01b038086165f90815260a060209081526040808320888452825280832093871683529281529082902083518154928501519385015163ffffffff16600160c01b0263ffffffff60c01b196001600160801b038616600160401b026001600160c01b03199095166001600160401b03909316929092179390931716919091179055600f0b15613a71576001600160a01b0385165f908152609f602090815260408083208784529091529020613a499084613562565b506001600160a01b0385165f908152609d60205260409020613a6b9085613557565b50610d0c565b80516001600160401b03165f03610d0c576001600160a01b0385165f908152609f602090815260408083208784529091529020613aae9084613fcc565b506001600160a01b0385165f908152609f602090815260408083208784529091529020613ada90613482565b5f03610d0c576001600160a01b0385165f908152609d602052604090206120bd9085613fc1565b6001600160a01b038084165f90815260a160209081526040808320938616835292905220613b3090438361432e565b7f1c6458079a41077d003c11faf9bf097e693bd67979e4e6500bac7b29db779b5c838383604051612fb293929190615a1b565b60605f61079483614342565b6001600160a01b038381165f90815260a360209081526040808320938616835292905290812054600f81810b600160801b909204900b035b5f81118015613bb957508261ffff1682105b15610d0c576001600160a01b038086165f90815260a3602090815260408083209388168352929052908120613bed9061439b565b90505f5f613bfc8884896132e5565b91509150806040015163ffffffff16431015613c1a57505050610d0c565b613c2788848985856138cd565b6001600160a01b038089165f90815260a360209081526040808320938b16835292905220613c54906143ed565b50613c5e85615a47565b9450613c6984615a5f565b9350505050613ba7565b60605f613c7f83614461565b6040805160208082528183019092529192505f91906020820181803683375050509182525060208101929092525090565b6001600160a01b0382165f908152609b60209081526040918290208251608081018452905463ffffffff808216835260ff600160201b830416151593830193909352600160281b8104831693820193909352600160481b909204166060820181905215801590613d2a5750806060015163ffffffff164310155b15613d4457604081015163ffffffff168152600160208201525b63ffffffff82166040820152613d7a7f0000000000000000000000000000000000000000000000000000000000000000436157cd565b613d859060016157cd565b63ffffffff90811660608381019182526001600160a01b0386165f818152609b602090815260409182902087518154838a0151858b01519851928a1664ffffffffff1990921691909117600160201b91151591909102176cffffffffffffffff00000000001916600160281b9789169790970263ffffffff60481b191696909617600160481b968816968702179055815192835294871694820194909452928301919091527f4e85751d6331506c6c62335f207eb31f12a61e570f34f5c17640308785c6d4db9101612fb2565b6001600160a01b038281165f81815260a2602090815260408083209486168084529482528083205493835260a38252808320948352939052918220546001600160401b039091169190600f81810b600160801b909204900b03815b81811015613f7d576001600160a01b038087165f90815260a3602090815260408083209389168352929052908120613ee59083614488565b6001600160a01b038881165f90815260a0602090815260408083208584528252808320938b16835292815290829020825160608101845290546001600160401b0381168252600160401b8104600f0b92820192909252600160c01b90910463ffffffff16918101829052919250431015613f60575050613f7d565b613f6e8682602001516138ae565b95505050806001019050613ead565b506001600160a01b038086165f90815260a1602090815260408083209388168352929052208390613fad906140ec565b613fb791906155ee565b9150509250929050565b5f61079483836144f7565b5f610794836001600160a01b0384166144f7565b6033546001600160a01b03163314611cbc5760405162461bcd60e51b815260206004820181905260248201527f4f776e61626c653a2063616c6c6572206973206e6f7420746865206f776e65726044820152606401612f39565b603380546001600160a01b038381166001600160a01b0319831681179093556040519116919082907f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e0905f90a35050565b5f6107948383670de0b6b3a76400006145da565b5f6140b08460995f61110a89613282565b80156140b95750815b8015611cdb57505090516001600160401b031615159392505050565b5f6107946001600160401b03808516908416615a74565b5f61081082670de0b6b3a764000061462f565b5f825f01828154811061411457614114615503565b905f5260205f200154905092915050565b5f81815260018301602052604081205461416a57508154600181810184555f848152602080822090930184905584548482528286019093526040902091909155610810565b505f610810565b5f6107948383670de0b6b3a76400006141de565b5f5f6141928686866141de565b905060018360028111156141a8576141a8615aa1565b1480156141c457505f84806141bf576141bf615ab5565b868809115b15611cdb576141d4600182615ac9565b9695505050505050565b5f80805f19858709858702925082811083820303915050805f036142155783828161420b5761420b615ab5565b0492505050610794565b80841161425c5760405162461bcd60e51b81526020600482015260156024820152744d6174683a206d756c446976206f766572666c6f7760581b6044820152606401612f39565b5f8486880960026001871981018816978890046003810283188082028403028082028403028082028403028082028403028082028403029081029092039091025f889003889004909101858311909403939093029303949094049190911702949350505050565b5f6001600160401b0382111561432a5760405162461bcd60e51b815260206004820152602660248201527f53616665436173743a2076616c756520646f65736e27742066697420696e203660448201526534206269747360d01b6064820152608401612f39565b5090565b612fbb83836001600160401b038416614666565b6060815f0180548060200260200160405190810160405280929190818152602001828054801561438f57602002820191905f5260205f20905b81548152602001906001019080831161437b575b50505050509050919050565b5f6143b58254600f81810b600160801b909204900b131590565b156143d357604051631ed9509560e11b815260040160405180910390fd5b508054600f0b5f9081526001909101602052604090205490565b5f6144078254600f81810b600160801b909204900b131590565b1561442557604051631ed9509560e11b815260040160405180910390fd5b508054600f0b5f818152600180840160205260408220805492905583546001600160801b03191692016001600160801b03169190911790915590565b5f60ff8216601f81111561081057604051632cd44ac360e21b815260040160405180910390fd5b5f5f6144aa61449684614769565b85546144a59190600f0b615adc565b6147d2565b8454909150600160801b9004600f90810b9082900b126144dd57604051632d0483c560e21b815260040160405180910390fd5b600f0b5f9081526001939093016020525050604090205490565b5f81815260018301602052604081205480156145d1575f6145196001836155db565b85549091505f9061452c906001906155db565b905081811461458b575f865f01828154811061454a5761454a615503565b905f5260205f200154905080875f01848154811061456a5761456a615503565b5f918252602080832090910192909255918252600188019052604090208390555b855486908061459c5761459c615b03565b600190038181905f5260205f20015f90559055856001015f8681526020019081526020015f205f905560019350505050610810565b5f915050610810565b82545f90816145eb8686838561483b565b905080156146255761460f866146026001846155db565b5f91825260209091200190565b54600160201b90046001600160e01b031661078f565b5091949350505050565b81545f90801561465e57614648846146026001846155db565b54600160201b90046001600160e01b031661080c565b509092915050565b8254801561471c575f61467e856146026001856155db565b60408051808201909152905463ffffffff808216808452600160201b9092046001600160e01b0316602084015291925090851610156146d05760405163151b8e3f60e11b815260040160405180910390fd5b805163ffffffff80861691160361471a57826146f1866146026001866155db565b80546001600160e01b0392909216600160201b0263ffffffff9092169190911790555050505050565b505b506040805180820190915263ffffffff92831681526001600160e01b03918216602080830191825285546001810187555f968752952091519051909216600160201b029190921617910155565b5f6001600160ff1b0382111561432a5760405162461bcd60e51b815260206004820152602860248201527f53616665436173743a2076616c756520646f65736e27742066697420696e2061604482015267371034b73a191a9b60c11b6064820152608401612f39565b80600f81900b81146148365760405162461bcd60e51b815260206004820152602760248201527f53616665436173743a2076616c756520646f65736e27742066697420696e20316044820152663238206269747360c81b6064820152608401612f39565b919050565b5f5b818310156109ff575f614850848461488e565b5f8781526020902090915063ffffffff86169082015463ffffffff16111561487a57809250614888565b614885816001615ac9565b93505b5061483d565b5f61489c6002848418615b17565b61079490848416615ac9565b604080516060810182525f808252602082018190529181019190915290565b6001600160a01b038116811461315d575f5ffd5b634e487b7160e01b5f52604160045260245ffd5b604051606081016001600160401b0381118282101715614911576149116148db565b60405290565b604051601f8201601f191681016001600160401b038111828210171561493f5761493f6148db565b604052919050565b803563ffffffff81168114614836575f5ffd5b5f6040828403121561496a575f5ffd5b604080519081016001600160401b038111828210171561498c5761498c6148db565b604052905080823561499d816148c7565b81526149ab60208401614947565b60208201525092915050565b5f5f5f608084860312156149c9575f5ffd5b83356149d4816148c7565b92506149e3856020860161495a565b915060608401356149f3816148c7565b809150509250925092565b80516001600160401b03168252602080820151600f0b9083015260409081015163ffffffff16910152565b6060810161081082846149fe565b5f5f60608385031215614a48575f5ffd5b8235614a53816148c7565b9150614a62846020850161495a565b90509250929050565b5f60208284031215614a7b575f5ffd5b5035919050565b5f60208284031215614a92575f5ffd5b8135610794816148c7565b80516001600160a01b0316825260209081015163ffffffff16910152565b5f8151808452602084019350602083015f5b82811015614af657614ae0868351614a9d565b6040959095019460209190910190600101614acd565b5093949350505050565b602081525f6107946020830184614abb565b5f60408284031215614b22575f5ffd5b610794838361495a565b5f5f83601f840112614b3c575f5ffd5b5081356001600160401b03811115614b52575f5ffd5b6020830191508360208260051b850101111561173f575f5ffd5b5f5f5f60408486031215614b7e575f5ffd5b8335614b89816148c7565b925060208401356001600160401b03811115614ba3575f5ffd5b614baf86828701614b2c565b9497909650939450505050565b5f6001600160401b03821115614bd457614bd46148db565b5060051b60200190565b5f82601f830112614bed575f5ffd5b8135614c00614bfb82614bbc565b614917565b8082825260208201915060208360051b860101925085831115614c21575f5ffd5b602085015b83811015614c47578035614c39816148c7565b835260209283019201614c26565b5095945050505050565b5f5f5f60808486031215614c63575f5ffd5b614c6d858561495a565b925060408401356001600160401b03811115614c87575f5ffd5b614c9386828701614bde565b92505060608401356001600160401b03811115614cae575f5ffd5b614cba86828701614bde565b9150509250925092565b5f8151808452602084019350602083015f5b82811015614af6578151865260209586019590910190600101614cd6565b5f602082016020835280845180835260408501915060408160051b8601019250602086015f5b82811015614d4b57603f19878603018452614d36858351614cc4565b94506020938401939190910190600101614d1a565b50929695505050505050565b5f5f5f5f60a08587031215614d6a575f5ffd5b614d74868661495a565b935060408501356001600160401b03811115614d8e575f5ffd5b614d9a87828801614bde565b93505060608501356001600160401b03811115614db5575f5ffd5b614dc187828801614bde565b925050614dd060808601614947565b905092959194509250565b6001600160a01b0391909116815260200190565b5f5f60408385031215614e00575f5ffd5b8235614e0b816148c7565b915060208301356001600160401b03811115614e25575f5ffd5b830160a08186031215614e36575f5ffd5b809150509250929050565b5f5f60408385031215614e52575f5ffd5b8235614e5d816148c7565b91506020830135614e36816148c7565b5f8151808452602084019350602083015f5b82811015614af657614e928683516149fe565b6060959095019460209190910190600101614e7f565b604081525f614eba6040830185614abb565b8281036020840152611cdb8185614e6d565b5f8151808452602084019350602083015f5b82811015614af65781516001600160a01b0316865260209586019590910190600101614ede565b602081525f6107946020830184614ecc565b5f5f60408385031215614f28575f5ffd5b82356001600160401b03811115614f3d575f5ffd5b614f4985828601614bde565b9250506020830135614e36816148c7565b602080825282518282018190525f918401906040840190835b81811015614f9a5783516001600160401b0316835260209384019390920191600101614f73565b509095945050505050565b5f5f5f5f5f60608688031215614fb9575f5ffd5b8535614fc4816148c7565b945060208601356001600160401b03811115614fde575f5ffd5b614fea88828901614b2c565b90955093505060408601356001600160401b03811115615008575f5ffd5b61501488828901614b2c565b969995985093965092949392505050565b5f5f5f5f60608587031215615038575f5ffd5b8435615043816148c7565b935061505160208601614947565b925060408501356001600160401b0381111561506b575f5ffd5b61507787828801614b2c565b95989497509550505050565b5f5f60408385031215615094575f5ffd5b823561509f816148c7565b915060208301356001600160401b038111156150b9575f5ffd5b613fb785828601614bde565b602081525f82518060208401525f5b818110156150f157602081860181015160408684010152016150d4565b505f604082850101526040601f19601f83011684010191505092915050565b5f5f60408385031215615121575f5ffd5b823561512c816148c7565b9150614a6260208401614947565b5f6020828403121561514a575f5ffd5b813560ff81168114610794575f5ffd5b5f6060828403121561516a575f5ffd5b50919050565b5f60208284031215615180575f5ffd5b81356001600160401b03811115615195575f5ffd5b61080c8482850161515a565b5f5f5f608084860312156151b3575f5ffd5b83356001600160401b038111156151c8575f5ffd5b6151d486828701614bde565b9350506149e3856020860161495a565b602081525f6107946020830184614e6d565b5f5f5f60608486031215615208575f5ffd5b8335615213816148c7565b925060208401356001600160401b0381111561522d575f5ffd5b61523986828701614bde565b92505061524860408501614947565b90509250925092565b5f5f60408385031215615262575f5ffd5b823561526d816148c7565b915060208301356001600160401b03811115615287575f5ffd5b8301601f81018513615297575f5ffd5b80356152a5614bfb82614bbc565b8082825260208201915060208360051b8501019250878311156152c6575f5ffd5b602084015b838110156153eb5780356001600160401b038111156152e8575f5ffd5b85016080818b03601f190112156152fd575f5ffd5b6153056148ef565b6153128b6020840161495a565b815260608201356001600160401b0381111561532c575f5ffd5b61533b8c602083860101614bde565b60208301525060808201356001600160401b03811115615359575f5ffd5b6020818401019250508a601f830112615370575f5ffd5b813561537e614bfb82614bbc565b8082825260208201915060208360051b86010192508d83111561539f575f5ffd5b6020850194505b828510156153d55784356001600160401b03811681146153c4575f5ffd5b8252602094850194909101906153a6565b60408401525050845250602092830192016152cb565b50809450505050509250929050565b5f5f5f6040848603121561540c575f5ffd5b8335615417816148c7565b925060208401356001600160401b03811115615431575f5ffd5b8401601f81018613615441575f5ffd5b80356001600160401b03811115615456575f5ffd5b866020828401011115615467575f5ffd5b939660209190910195509293505050565b5f5f60408385031215615489575f5ffd5b8235615494816148c7565b915060208301356001600160401b038111156154ae575f5ffd5b613fb78582860161515a565b5f5f604083850312156154cb575f5ffd5b82356154d6816148c7565b946020939093013593505050565b5f602082840312156154f4575f5ffd5b81518015158114610794575f5ffd5b634e487b7160e01b5f52603260045260245ffd5b5f8235603e1983360301811261552b575f5ffd5b9190910192915050565b5f60208284031215615545575f5ffd5b61079482614947565b604081016108108284614a9d565b5f5f8335601e19843603018112615571575f5ffd5b8301803591506001600160401b0382111561558a575f5ffd5b6020019150600581901b360382131561173f575f5ffd5b606081016155af8285614a9d565b6001600160a01b039290921660409190910152919050565b634e487b7160e01b5f52601160045260245ffd5b81810381811115610810576108106155c7565b6001600160401b038281168282160390811115610810576108106155c7565b5f600f82900b6001607f1b8101615626576156266155c7565b5f0392915050565b600f81810b9083900b0160016001607f1b03811360016001607f1b031982121715610810576108106155c7565b6001600160a01b038616815260c081016156786020830187614a9d565b6001600160a01b039490941660608201526001600160401b0392909216608083015263ffffffff1660a09091015292915050565b5f5f8335601e198436030181126156c1575f5ffd5b8301803591506001600160401b038211156156da575f5ffd5b60200191503681900382131561173f575f5ffd5b81835281816020850137505f828201602090810191909152601f909101601f19169091010190565b6001600160a01b03881681525f60c08201615734602084018a614a9d565b60c060608401528690528660e083015f5b88811015615775578235615758816148c7565b6001600160a01b0316825260209283019290910190600101615745565b5083810360808501526157888188614cc4565b91505082810360a084015261579e8185876156ee565b9a9950505050505050505050565b5f602082840312156157bc575f5ffd5b813561ffff81168114610794575f5ffd5b63ffffffff8181168382160190811115610810576108106155c7565b8183526020830192505f815f5b84811015614af65763ffffffff61580c83614947565b16865260209586019591909101906001016157f6565b6001600160a01b038581168252841660208201526060604082018190525f906141d490830184866157e9565b602081525f610d216020830184866156ee565b6001600160a01b038781168252861660208201526080604082018190525f9061588d90830186886157e9565b82810360608401526158a08185876156ee565b9998505050505050505050565b5f602082840312156158bd575f5ffd5b8151610794816148c7565b8051602080830151919081101561516a575f1960209190910360031b1b16919050565b604081525f6158fd6040830185614ecc565b8281036020840152611cdb8185614ecc565b5f6020828403121561591f575f5ffd5b81516001600160401b03811115615934575f5ffd5b8201601f81018413615944575f5ffd5b8051615952614bfb82614bbc565b8082825260208201915060208360051b850101925086831115615973575f5ffd5b602084015b83811015615a105780516001600160401b03811115615995575f5ffd5b8501603f810189136159a5575f5ffd5b60208101516159b6614bfb82614bbc565b808282526020820191506020808460051b8601010192508b8311156159d9575f5ffd5b6040840193505b828410156159fb5783518252602093840193909101906159e0565b86525050602093840193919091019050615978565b509695505050505050565b6001600160a01b0393841681529190921660208201526001600160401b03909116604082015260600190565b5f60018201615a5857615a586155c7565b5060010190565b5f81615a6d57615a6d6155c7565b505f190190565b600f82810b9082900b0360016001607f1b0319811260016001607f1b0382131715610810576108106155c7565b634e487b7160e01b5f52602160045260245ffd5b634e487b7160e01b5f52601260045260245ffd5b80820180821115610810576108106155c7565b8082018281125f831280158216821582161715615afb57615afb6155c7565b505092915050565b634e487b7160e01b5f52603160045260245ffd5b5f82615b3157634e487b7160e01b5f52601260045260245ffd5b50049056fe1487af5418c47ee5ea45ef4a93398668120890774a9e13487e61e9dc3baf76dda264697066735822122064a2cb6e41f7f8bd29297c0eef600bae9afeb2ec41d7d1f5544aca9fe17d469964736f6c634300081b0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R4\x80\x15a\0\x0FW__\xFD[P`\x046\x10a\x02\xA2W_5`\xE0\x1C\x80cl\xFBD\x81\x11a\x01lW\x80c\xA9\x82\x18!\x11a\0\xD5W\x80c\xC2!\xD8\xAE\x11a\0\x8FW\x80c\xC2!\xD8\xAE\x14a\x06\xDAW\x80c\xCDm\xC6\x87\x14a\x06\xEDW\x80c\xD3\xD9o\xF4\x14a\x07\0W\x80c\xDF\\\xF7#\x14a\x07\x13W\x80c\xF2\xFD\xE3\x8B\x14a\x07:W\x80c\xF6\x05\xCE\x08\x14a\x07MW\x80c\xFA\xBC\x1C\xBC\x14a\x07`W__\xFD[\x80c\xA9\x82\x18!\x14a\x06LW\x80c\xAD\xC2\xE3\xD9\x14a\x06_W\x80c\xB2Dz\xF7\x14a\x06rW\x80c\xB6k\xD9\x89\x14a\x06\x85W\x80c\xB9\xFB\xAE\xD1\x14a\x06\x98W\x80c\xBA\x1A\x84\xE5\x14a\x06\xC7W__\xFD[\x80c\x88o\x11\x95\x11a\x01&W\x80c\x88o\x11\x95\x14a\x05\xBBW\x80c\x8C\xE6HT\x14a\x05\xE2W\x80c\x8D\xA5\xCB[\x14a\x06\x02W\x80c\x94\xD7\xD0\x0C\x14a\x06\x13W\x80c\x95(\x99\xEE\x14a\x06&W\x80c\xA93>\xC8\x14a\x069W__\xFD[\x80cl\xFBD\x81\x14a\x05(W\x80cn4\x92\xB5\x14a\x05SW\x80cn\x87]\xBA\x14a\x05fW\x80cqP\x18\xA6\x14a\x05yW\x80cy\xAEP\xCD\x14a\x05\x81W\x80c{\xC1\xEFa\x14a\x05\x94W__\xFD[\x80cAw\xA8|\x11a\x02\x0EW\x80cT\xFDMP\x11a\x01\xC8W\x80cT\xFDMP\x14a\x04\xB0W\x80cV\xC4\x83\xE6\x14a\x04\xC5W\x80cY\\jg\x14a\x04\xD8W\x80cZ\xC8j\xB7\x14a\x04\xE0W\x80c\\\x97Z\xBB\x14a\x05\x03W\x80cg\r;\xA2\x14a\x05\x15W__\xFD[\x80cAw\xA8|\x14a\x04\x10W\x80cFW\xE2j\x14a\x040W\x80cJ\x10\xFF\xE5\x14a\x04WW\x80cKPF\xEF\x14a\x04wW\x80cP\xFE\xEA \x14a\x04\x8AW\x80cTz\xFB\x87\x14a\x04\x9DW__\xFD[\x80c)\x81\xEBw\x11a\x02_W\x80c)\x81\xEBw\x14a\x03MW\x80c+E:\x9A\x14a\x03\x89W\x80c+\xAB,J\x14a\x03\xA9W\x80c0L\x10\xCD\x14a\x03\xBCW\x80c65 W\x14a\x03\xDCW\x80c@\x12\r\xAB\x14a\x03\xEFW__\xFD[\x80c\x10\xE1\xB9\xB8\x14a\x02\xA6W\x80c\x13R\xC3\xE6\x14a\x02\xCFW\x80c\x13d9\xDD\x14a\x02\xF2W\x80c\x15\xFEP(\x14a\x03\x07W\x80c&\r\xC7X\x14a\x03'W\x80c&\x1F\x84\xE0\x14a\x03:W[__\xFD[a\x02\xB9a\x02\xB46`\x04aI\xB7V[a\x07sV[`@Qa\x02\xC6\x91\x90aJ)V[`@Q\x80\x91\x03\x90\xF3[a\x02\xE2a\x02\xDD6`\x04aJ7V[a\x07\x9BV[`@Q\x90\x15\x15\x81R` \x01a\x02\xC6V[a\x03\x05a\x03\x006`\x04aJkV[a\x08\x16V[\0[a\x03\x1Aa\x03\x156`\x04aJ\x82V[a\x08\xF0V[`@Qa\x02\xC6\x91\x90aK\0V[a\x02\xE2a\x0356`\x04aK\x12V[a\n\x07V[a\x03\x05a\x03H6`\x04aKlV[a\n8V[a\x03t\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Qc\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x02\xC6V[a\x03\x9Ca\x03\x976`\x04aLQV[a\r\x13V[`@Qa\x02\xC6\x91\x90aL\xF4V[a\x03\x9Ca\x03\xB76`\x04aMWV[a\r)V[a\x03\xCFa\x03\xCA6`\x04aJ\x82V[a\r\xC8V[`@Qa\x02\xC6\x91\x90aM\xDBV[a\x03\x05a\x03\xEA6`\x04aM\xEFV[a\r\xF7V[a\x04\x02a\x03\xFD6`\x04aNAV[a\x15\xDBV[`@Qa\x02\xC6\x92\x91\x90aN\xA8V[a\x04#a\x04\x1E6`\x04aK\x12V[a\x17FV[`@Qa\x02\xC6\x91\x90aO\x05V[a\x03\xCF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x04ja\x04e6`\x04aO\x17V[a\x17jV[`@Qa\x02\xC6\x91\x90aOZV[a\x03\x05a\x04\x856`\x04aO\xA5V[a\x18\x12V[a\x03\x05a\x04\x986`\x04aP%V[a\x18\xCCV[a\x04ja\x04\xAB6`\x04aP\x83V[a\x1A*V[a\x04\xB8a\x1A\xD2V[`@Qa\x02\xC6\x91\x90aP\xC5V[a\x03\x05a\x04\xD36`\x04aQ\x10V[a\x1B\x02V[a\x03\x05a\x1C\nV[a\x02\xE2a\x04\xEE6`\x04aQ:V[`fT`\x01`\xFF\x90\x92\x16\x91\x90\x91\x1B\x90\x81\x16\x14\x90V[`fT[`@Q\x90\x81R` \x01a\x02\xC6V[a\x02\xE2a\x05#6`\x04aJ7V[a\x1C\xBEV[a\x05;a\x0566`\x04aNAV[a\x1C\xCFV[`@Q`\x01`\x01`@\x1B\x03\x90\x91\x16\x81R` \x01a\x02\xC6V[a\x03\x05a\x05a6`\x04aQpV[a\x1C\xE4V[a\x04#a\x05t6`\x04aK\x12V[a \xC5V[a\x03\x05a \xD6V[a\x03\x1Aa\x05\x8F6`\x04aJ\x82V[a \xE7V[a\x03t\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x03\xCF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x05\xF5a\x05\xF06`\x04aQ\xA1V[a!\xC1V[`@Qa\x02\xC6\x91\x90aQ\xE4V[`3T`\x01`\x01`\xA0\x1B\x03\x16a\x03\xCFV[a\x04ja\x06!6`\x04aQ\xF6V[a\"mV[a\x03\x05a\x0646`\x04aRQV[a#YV[a\x05;a\x06G6`\x04aNAV[a(\x05V[a\x03\x05a\x06Z6`\x04aS\xFAV[a(4V[a\x03\x05a\x06m6`\x04aTxV[a(\xE6V[a\x05\x07a\x06\x806`\x04aK\x12V[a,FV[a\x03\x05a\x06\x936`\x04aP%V[a,hV[a\x06\xABa\x06\xA66`\x04aJ\x82V[a-\xC2V[`@\x80Q\x92\x15\x15\x83Rc\xFF\xFF\xFF\xFF\x90\x91\x16` \x83\x01R\x01a\x02\xC6V[a\x05\x07a\x06\xD56`\x04aJ\x82V[a.ZV[a\x04#a\x06\xE86`\x04aJ7V[a.zV[a\x03\x05a\x06\xFB6`\x04aT\xBAV[a.\xA3V[a\x03\x05a\x07\x0E6`\x04aNAV[a/\xC0V[a\x03\xCF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x03\x05a\x07H6`\x04aJ\x82V[a0\xE7V[a\x05;a\x07[6`\x04aNAV[a1`V[a\x03\x05a\x07n6`\x04aJkV[a1lV[a\x07{aH\xA8V[_a\x07\x8F\x85a\x07\x89\x86a2\x82V[\x85a2\xE5V[\x92PPP[\x93\x92PPPV[`\x01`\x01`\xA0\x1B\x03\x82\x16_\x90\x81R`\x9E` R`@\x81 \x81\x90\x81a\x07\xBE\x85a2\x82V[\x81R` \x80\x82\x01\x92\x90\x92R`@\x90\x81\x01_ \x81Q\x80\x83\x01\x90\x92RT`\xFF\x81\x16\x15\x15\x80\x83Ra\x01\0\x90\x91\x04c\xFF\xFF\xFF\xFF\x16\x92\x82\x01\x92\x90\x92R\x91P\x80a\x08\x0CWP\x80` \x01Qc\xFF\xFF\xFF\xFF\x16C\x11\x15[\x91PP[\x92\x91PPV[`@Qc#}\xFBG`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90cF\xFB\xF6\x8E\x90a\x08b\x903\x90`\x04\x01aM\xDBV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x08}W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x08\xA1\x91\x90aT\xE4V[a\x08\xBEW`@Qc\x1Dw\xD4w`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`fT\x81\x81\x16\x81\x14a\x08\xE3W`@Qc\xC6\x1D\xCA]`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x08\xEC\x82a4EV[PPV[`\x01`\x01`\xA0\x1B\x03\x81\x16_\x90\x81R`\x9D` R`@\x81 ``\x91\x90a\t\x14\x90a4\x82V[\x90P_\x81`\x01`\x01`@\x1B\x03\x81\x11\x15a\t/Wa\t/aH\xDBV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\tsW\x81` \x01[`@\x80Q\x80\x82\x01\x90\x91R_\x80\x82R` \x82\x01R\x81R` \x01\x90`\x01\x90\x03\x90\x81a\tMW\x90P[P\x90P_[\x82\x81\x10\x15a\t\xFFW`\x01`\x01`\xA0\x1B\x03\x85\x16_\x90\x81R`\x9D` R`@\x90 a\t\xDA\x90a\t\xA5\x90\x83a4\x8BV[`@\x80Q\x80\x82\x01\x90\x91R_\x80\x82R` \x82\x01RP`@\x80Q\x80\x82\x01\x90\x91R``\x82\x90\x1C\x81Rc\xFF\xFF\xFF\xFF\x90\x91\x16` \x82\x01R\x90V[\x82\x82\x81Q\x81\x10a\t\xECWa\t\xECaU\x03V[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a\txV[P\x93\x92PPPV[` \x80\x82\x01Q\x82Q`\x01`\x01`\xA0\x1B\x03\x16_\x90\x81R`\x98\x90\x92R`@\x82 a\x08\x10\x91c\xFF\xFF\xFF\xFF\x90\x81\x16\x90a4\x96\x16V[\x82a\nB\x81a4\xADV[a\n_W`@Qc\x93-\x94\xF7`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x84\x16_\x90\x81R`\xA4` R`@\x90 T`\xFF\x16a\n\x97W`@QcH\xF7\xDB\xB9`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_[\x82\x81\x10\x15a\r\x0CW_`@Q\x80`@\x01`@R\x80\x87`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x86\x86\x85\x81\x81\x10a\n\xCEWa\n\xCEaU\x03V[\x90P` \x02\x81\x01\x90a\n\xE0\x91\x90aU\x17V[a\n\xEE\x90` \x81\x01\x90aU5V[c\xFF\xFF\xFF\xFF\x16\x81RP\x90Pa\x0B8\x81` \x01Qc\xFF\xFF\xFF\xFF\x16`\x98_\x89`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x90\x81R` \x01_ a5W\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[a\x0BUW`@Qc\x1F\xB1pU`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x7F1b\x92\x85\xEA\xD23Z\xE0\x93?\x86\xED*\xE63!\xF7\xAFw\xB4\xE6\xEA\xAB\xC4,\x05x\x80\x97~l`@Q\x80`@\x01`@R\x80\x88`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x83` \x01Qc\xFF\xFF\xFF\xFF\x16\x81RP`@Qa\x0B\xAB\x91\x90aUNV[`@Q\x80\x91\x03\x90\xA1_a\x0B\xBD\x82a2\x82V[\x90P_[\x86\x86\x85\x81\x81\x10a\x0B\xD3Wa\x0B\xD3aU\x03V[\x90P` \x02\x81\x01\x90a\x0B\xE5\x91\x90aU\x17V[a\x0B\xF3\x90` \x81\x01\x90aU\\V[\x90P\x81\x10\x15a\r\x01Wa\x0Ci\x87\x87\x86\x81\x81\x10a\x0C\x11Wa\x0C\x11aU\x03V[\x90P` \x02\x81\x01\x90a\x0C#\x91\x90aU\x17V[a\x0C1\x90` \x81\x01\x90aU\\V[\x83\x81\x81\x10a\x0CAWa\x0CAaU\x03V[\x90P` \x02\x01` \x81\x01\x90a\x0CV\x91\x90aJ\x82V[_\x84\x81R`\x99` R`@\x90 \x90a5bV[P\x7Fz\xB2`\xFE\n\xF1\x93\xDB_I\x86w\r\x83\x1B\xDAN\xA4`\x99\xDC\x81~\x8Bg\x16\xDC\xAE\x8A\xF8\xE8\x8B\x83\x88\x88\x87\x81\x81\x10a\x0C\x9EWa\x0C\x9EaU\x03V[\x90P` \x02\x81\x01\x90a\x0C\xB0\x91\x90aU\x17V[a\x0C\xBE\x90` \x81\x01\x90aU\\V[\x84\x81\x81\x10a\x0C\xCEWa\x0C\xCEaU\x03V[\x90P` \x02\x01` \x81\x01\x90a\x0C\xE3\x91\x90aJ\x82V[`@Qa\x0C\xF1\x92\x91\x90aU\xA1V[`@Q\x80\x91\x03\x90\xA1`\x01\x01a\x0B\xC1V[PPP`\x01\x01a\n\x99V[PPPPPV[``a\r!\x84\x84\x84Ca5vV[\x94\x93PPPPV[``a\r7\x85\x85\x85\x85a5vV[\x90P_[\x84Q\x81\x10\x15a\r\xBFWa\rg\x85\x82\x81Q\x81\x10a\rYWa\rYaU\x03V[` \x02` \x01\x01Q\x87a\x07\x9BV[a\r\xB7W_[\x84Q\x81\x10\x15a\r\xB5W_\x83\x83\x81Q\x81\x10a\r\x89Wa\r\x89aU\x03V[` \x02` \x01\x01Q\x82\x81Q\x81\x10a\r\xA2Wa\r\xA2aU\x03V[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a\rmV[P[`\x01\x01a\r;V[P\x94\x93PPPPV[`\x01`\x01`\xA0\x1B\x03\x80\x82\x16_\x90\x81R`\x97` R`@\x81 T\x90\x91\x16\x80\x15a\r\xF0W\x80a\x07\x94V[P\x90\x91\x90PV[`fT`\x01\x90`\x02\x90\x81\x16\x03a\x0E W`@Qc\x84\nH\xD5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x82a\x0E*\x81a4\xADV[a\x0EGW`@Qc\x93-\x94\xF7`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_`@Q\x80`@\x01`@R\x80\x86`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x85` \x01` \x81\x01\x90a\x0Eu\x91\x90aU5V[c\xFF\xFF\xFF\xFF\x16\x90R\x90Pa\x0E\x8C``\x85\x01\x85aU\\V[\x90Pa\x0E\x9B`@\x86\x01\x86aU\\V[\x90P\x14a\x0E\xBBW`@QcCqJ\xFD`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[` \x80\x82\x01Q\x82Q`\x01`\x01`\xA0\x1B\x03\x16_\x90\x81R`\x98\x90\x92R`@\x90\x91 a\x0E\xED\x91c\xFF\xFF\xFF\xFF\x90\x81\x16\x90a4\x96\x16V[a\x0F\nW`@Qc\x1F\xB1pU`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x0F a\x0F\x1A` \x86\x01\x86aJ\x82V[\x82a\x07\x9BV[a\x0F=W`@Qc\xEB\xBF\xF4\x97`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a\x0FK`@\x86\x01\x86aU\\V[\x90P`\x01`\x01`@\x1B\x03\x81\x11\x15a\x0FdWa\x0FdaH\xDBV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x0F\x8DW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P_[a\x0F\x9F`@\x87\x01\x87aU\\V[\x90P\x81\x10\x15a\x15mW\x80\x15\x80a\x102WPa\x0F\xBD`@\x87\x01\x87aU\\V[a\x0F\xC8`\x01\x84aU\xDBV[\x81\x81\x10a\x0F\xD7Wa\x0F\xD7aU\x03V[\x90P` \x02\x01` \x81\x01\x90a\x0F\xEC\x91\x90aJ\x82V[`\x01`\x01`\xA0\x1B\x03\x16a\x10\x02`@\x88\x01\x88aU\\V[\x83\x81\x81\x10a\x10\x12Wa\x10\x12aU\x03V[\x90P` \x02\x01` \x81\x01\x90a\x10'\x91\x90aJ\x82V[`\x01`\x01`\xA0\x1B\x03\x16\x11[a\x10OW`@Qc\x9F\x1C\x80S`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x10\\``\x87\x01\x87aU\\V[\x82\x81\x81\x10a\x10lWa\x10laU\x03V[\x90P` \x02\x015_\x10\x80\x15a\x10\xACWPg\r\xE0\xB6\xB3\xA7d\0\0a\x10\x92``\x88\x01\x88aU\\V[\x83\x81\x81\x10a\x10\xA2Wa\x10\xA2aU\x03V[\x90P` \x02\x015\x11\x15[a\x10\xC9W`@Qc\x13S`1`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x11%a\x10\xD9`@\x88\x01\x88aU\\V[\x83\x81\x81\x10a\x10\xE9Wa\x10\xE9aU\x03V[\x90P` \x02\x01` \x81\x01\x90a\x10\xFE\x91\x90aJ\x82V[`\x99_a\x11\n\x87a2\x82V[\x81R` \x01\x90\x81R` \x01_ a8c\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[a\x11BW`@Qc1\xBC4'`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x80a\x11\x94a\x11T` \x8A\x01\x8AaJ\x82V[a\x11]\x87a2\x82V[a\x11j`@\x8C\x01\x8CaU\\V[\x87\x81\x81\x10a\x11zWa\x11zaU\x03V[\x90P` \x02\x01` \x81\x01\x90a\x11\x8F\x91\x90aJ\x82V[a2\xE5V[\x80Q\x91\x93P\x91P`\x01`\x01`@\x1B\x03\x16_\x03a\x11\xB1WPPa\x15eV[_a\x11\xECa\x11\xC2``\x8B\x01\x8BaU\\V[\x86\x81\x81\x10a\x11\xD2Wa\x11\xD2aU\x03V[\x85Q`\x01`\x01`@\x1B\x03\x16\x92` \x90\x91\x02\x015\x90Pa8\x84V[\x83Q\x90\x91Pa\x12\x07`\x01`\x01`@\x1B\x03\x80\x84\x16\x90\x83\x16a8\x9AV[\x86\x86\x81Q\x81\x10a\x12\x19Wa\x12\x19aU\x03V[` \x02` \x01\x01\x81\x81RPP\x81\x83_\x01\x81\x81Qa\x126\x91\x90aU\xEEV[`\x01`\x01`@\x1B\x03\x16\x90RP\x83Q\x82\x90\x85\x90a\x12S\x90\x83\x90aU\xEEV[`\x01`\x01`@\x1B\x03\x16\x90RP` \x84\x01\x80Q\x83\x91\x90a\x12s\x90\x83\x90aU\xEEV[`\x01`\x01`@\x1B\x03\x16\x90RP` \x83\x01Q_`\x0F\x91\x90\x91\x0B\x12\x15a\x13xW_a\x12\xD6a\x12\xA2``\x8D\x01\x8DaU\\V[\x88\x81\x81\x10a\x12\xB2Wa\x12\xB2aU\x03V[\x90P` \x02\x015\x85` \x01Qa\x12\xC7\x90aV\rV[`\x01`\x01`\x80\x1B\x03\x16\x90a8\x84V[\x90P\x80`\x01`\x01`@\x1B\x03\x16\x84` \x01\x81\x81Qa\x12\xF3\x91\x90aV.V[`\x0F\x0B\x90RP_Q` a[7_9_Q\x90_Ra\x13\x14` \x8D\x01\x8DaJ\x82V[\x89a\x13\"`@\x8F\x01\x8FaU\\V[\x8A\x81\x81\x10a\x132Wa\x132aU\x03V[\x90P` \x02\x01` \x81\x01\x90a\x13G\x91\x90aJ\x82V[a\x13X\x88_\x01Q\x89` \x01Qa8\xAEV[\x88`@\x01Q`@Qa\x13n\x95\x94\x93\x92\x91\x90aV[V[`@Q\x80\x91\x03\x90\xA1P[a\x13\xCAa\x13\x88` \x8C\x01\x8CaJ\x82V[a\x13\x91\x89a2\x82V[a\x13\x9E`@\x8E\x01\x8EaU\\V[\x89\x81\x81\x10a\x13\xAEWa\x13\xAEaU\x03V[\x90P` \x02\x01` \x81\x01\x90a\x13\xC3\x91\x90aJ\x82V[\x87\x87a8\xCDV[_Q` a[7_9_Q\x90_Ra\x13\xE5` \x8C\x01\x8CaJ\x82V[\x88a\x13\xF3`@\x8E\x01\x8EaU\\V[\x89\x81\x81\x10a\x14\x03Wa\x14\x03aU\x03V[\x90P` \x02\x01` \x81\x01\x90a\x14\x18\x91\x90aJ\x82V[\x86Q`@Qa\x14,\x94\x93\x92\x91\x90C\x90aV[V[`@Q\x80\x91\x03\x90\xA1a\x14}a\x14D` \x8C\x01\x8CaJ\x82V[a\x14Q`@\x8D\x01\x8DaU\\V[\x88\x81\x81\x10a\x14aWa\x14aaU\x03V[\x90P` \x02\x01` \x81\x01\x90a\x14v\x91\x90aJ\x82V[\x86Qa;\x01V[`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16c`\x1B\xB3oa\x14\xB9` \x8D\x01\x8DaJ\x82V[a\x14\xC6`@\x8E\x01\x8EaU\\V[\x89\x81\x81\x10a\x14\xD6Wa\x14\xD6aU\x03V[\x90P` \x02\x01` \x81\x01\x90a\x14\xEB\x91\x90aJ\x82V[\x87Q`@Q`\xE0\x85\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x81R`\x01`\x01`\xA0\x1B\x03\x93\x84\x16`\x04\x82\x01R\x92\x90\x91\x16`$\x83\x01R`\x01`\x01`@\x1B\x03\x80\x86\x16`D\x84\x01R\x16`d\x82\x01R`\x84\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x15JW__\xFD[PZ\xF1\x15\x80\x15a\x15\\W=__>=_\xFD[PPPPPPPP[`\x01\x01a\x0F\x92V[P\x7F\x80\x96\x9A\xD2\x94(\xD6y~\xE7\xAA\xD0\x84\xF9\xE4\xA4*\x82\xFCPm\xCD,\xA3\xB6\xFBC\x1F\x85\xCC\xEB\xE5a\x15\x9C` \x87\x01\x87aJ\x82V[\x83a\x15\xAA`@\x89\x01\x89aU\\V[\x85a\x15\xB8`\x80\x8C\x01\x8CaV\xACV[`@Qa\x15\xCB\x97\x96\x95\x94\x93\x92\x91\x90aW\x16V[`@Q\x80\x91\x03\x90\xA1PPPPPPV[`\x01`\x01`\xA0\x1B\x03\x82\x16_\x90\x81R`\x9D` R`@\x81 ``\x91\x82\x91a\x16\0\x90a4\x82V[\x90P_\x81`\x01`\x01`@\x1B\x03\x81\x11\x15a\x16\x1BWa\x16\x1BaH\xDBV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x16_W\x81` \x01[`@\x80Q\x80\x82\x01\x90\x91R_\x80\x82R` \x82\x01R\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x169W\x90P[P\x90P_\x82`\x01`\x01`@\x1B\x03\x81\x11\x15a\x16{Wa\x16{aH\xDBV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x16\xB4W\x81` \x01[a\x16\xA1aH\xA8V[\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x16\x99W\x90P[P\x90P_[\x83\x81\x10\x15a\x177W`\x01`\x01`\xA0\x1B\x03\x88\x16_\x90\x81R`\x9D` R`@\x81 a\x16\xE6\x90a\t\xA5\x90\x84a4\x8BV[\x90P\x80\x84\x83\x81Q\x81\x10a\x16\xFBWa\x16\xFBaU\x03V[` \x02` \x01\x01\x81\x90RPa\x17\x11\x89\x82\x8Aa\x07sV[\x83\x83\x81Q\x81\x10a\x17#Wa\x17#aU\x03V[` \x90\x81\x02\x91\x90\x91\x01\x01RP`\x01\x01a\x16\xB9V[P\x90\x93P\x91PP[\x92P\x92\x90PV[``_a\x07\x94`\x99_a\x17X\x86a2\x82V[\x81R` \x01\x90\x81R` \x01_ a;cV[``_\x83Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x17\x86Wa\x17\x86aH\xDBV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x17\xAFW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P_[\x84Q\x81\x10\x15a\t\xFFWa\x17\xE0\x85\x82\x81Q\x81\x10a\x17\xD2Wa\x17\xD2aU\x03V[` \x02` \x01\x01Q\x85a(\x05V[\x82\x82\x81Q\x81\x10a\x17\xF2Wa\x17\xF2aU\x03V[`\x01`\x01`@\x1B\x03\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01R`\x01\x01a\x17\xB4V[`fT_\x90`\x01\x90\x81\x16\x03a\x18:W`@Qc\x84\nH\xD5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x83\x82\x14a\x18ZW`@QcCqJ\xFD`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_[\x84\x81\x10\x15a\x18\xC3Wa\x18\xBB\x87\x87\x87\x84\x81\x81\x10a\x18zWa\x18zaU\x03V[\x90P` \x02\x01` \x81\x01\x90a\x18\x8F\x91\x90aJ\x82V[\x86\x86\x85\x81\x81\x10a\x18\xA1Wa\x18\xA1aU\x03V[\x90P` \x02\x01` \x81\x01\x90a\x18\xB6\x91\x90aW\xACV[a;oV[`\x01\x01a\x18\\V[PPPPPPPV[\x83a\x18\xD6\x81a4\xADV[a\x18\xF3W`@Qc\x93-\x94\xF7`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@\x80Q\x80\x82\x01\x90\x91R`\x01`\x01`\xA0\x1B\x03\x86\x16\x81Rc\xFF\xFF\xFF\xFF\x85\x16` \x82\x01R_a\x19\x1F\x82a2\x82V[\x90Pa\x19`\x82` \x01Qc\xFF\xFF\xFF\xFF\x16`\x98_\x8A`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x90\x81R` \x01_ a4\x96\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[a\x19}W`@Qc\x1F\xB1pU`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_[\x84\x81\x10\x15a\x1A Wa\x19\x9C\x86\x86\x83\x81\x81\x10a\x0CAWa\x0CAaU\x03V[a\x19\xB9W`@QcX\\\xFB/`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x7Fz\xB2`\xFE\n\xF1\x93\xDB_I\x86w\r\x83\x1B\xDAN\xA4`\x99\xDC\x81~\x8Bg\x16\xDC\xAE\x8A\xF8\xE8\x8B\x83\x87\x87\x84\x81\x81\x10a\x19\xEDWa\x19\xEDaU\x03V[\x90P` \x02\x01` \x81\x01\x90a\x1A\x02\x91\x90aJ\x82V[`@Qa\x1A\x10\x92\x91\x90aU\xA1V[`@Q\x80\x91\x03\x90\xA1`\x01\x01a\x19\x7FV[PPPPPPPPV[``_\x82Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1AFWa\x1AFaH\xDBV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x1AoW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P_[\x83Q\x81\x10\x15a\t\xFFWa\x1A\xA0\x85\x85\x83\x81Q\x81\x10a\x1A\x93Wa\x1A\x93aU\x03V[` \x02` \x01\x01Qa(\x05V[\x82\x82\x81Q\x81\x10a\x1A\xB2Wa\x1A\xB2aU\x03V[`\x01`\x01`@\x1B\x03\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01R`\x01\x01a\x1AtV[``a\x1A\xFD\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a<sV[\x90P\x90V[3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x1C\0Wa\x1B;\x82a4\xADV[a\x1BXW`@QcH\xF5\xC3\xED`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@Qc6\xB8{\xD7`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90cmp\xF7\xAE\x90a\x1B\xA4\x90\x85\x90`\x04\x01aM\xDBV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1B\xBFW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1B\xE3\x91\x90aT\xE4V[a\x1C\0W`@Qc\xCC\xEA\x9Eo`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x08\xEC\x82\x82a<\xB0V[`@Qc#}\xFBG`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90cF\xFB\xF6\x8E\x90a\x1CV\x903\x90`\x04\x01aM\xDBV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1CqW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1C\x95\x91\x90aT\xE4V[a\x1C\xB2W`@Qc\x1Dw\xD4w`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x1C\xBC_\x19a4EV[V[_a\x07\x94\x83`\x9A_a\x11\n\x86a2\x82V[__a\x1C\xDB\x84\x84a>RV[\x95\x94PPPPPV[`fT`\x02\x90`\x04\x90\x81\x16\x03a\x1D\rW`@Qc\x84\nH\xD5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x1D\"a\x1D\x1D` \x84\x01\x84aJ\x82V[a4\xADV[\x80a\x1D;WPa\x1D;a\x1D\x1D`@\x84\x01` \x85\x01aJ\x82V[a\x1DXW`@QcH\xF5\xC3\xED`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_[a\x1Dg`@\x84\x01\x84aU\\V[\x90P\x81\x10\x15a )W_`@Q\x80`@\x01`@R\x80\x85` \x01` \x81\x01\x90a\x1D\x8F\x91\x90aJ\x82V[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01a\x1D\xAA`@\x87\x01\x87aU\\V[\x85\x81\x81\x10a\x1D\xBAWa\x1D\xBAaU\x03V[\x90P` \x02\x01` \x81\x01\x90a\x1D\xCF\x91\x90aU5V[c\xFF\xFF\xFF\xFF\x16\x81RP\x90Pa\x1E\x1C\x81` \x01Qc\xFF\xFF\xFF\xFF\x16`\x98_\x87` \x01` \x81\x01\x90a\x1D\xFE\x91\x90aJ\x82V[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x81\x01\x91\x90\x91R`@\x01_ \x90a4\x96V[a\x1E9W`@Qc\x1F\xB1pU`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x9E_a\x1EI` \x87\x01\x87aJ\x82V[`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x90\x81R` \x01_ _a\x1Er\x83a2\x82V[\x81R` \x81\x01\x91\x90\x91R`@\x01_ T`\xFF\x16a\x1E\xA2W`@Qc%\x13\x1DO`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x1E\xDCa\x1E\xAE\x82a2\x82V[`\x9C_a\x1E\xBE` \x89\x01\x89aJ\x82V[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x81\x01\x91\x90\x91R`@\x01_ \x90a?\xC1V[Pa\x1F\x14a\x1E\xED` \x86\x01\x86aJ\x82V[`\x9A_a\x1E\xF9\x85a2\x82V[\x81R` \x01\x90\x81R` \x01_ a?\xCC\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[Pa\x1F\"` \x85\x01\x85aJ\x82V[`\x01`\x01`\xA0\x1B\x03\x16\x7F\xAD4\xC3\x07\x0B\xE1\xDF\xFB\xCA\xA4\x99\xD0\0\xBA+\x8D\x98H\xAE\xFC\xAC0Y\xDF$]\xD9\\N\xCE\x14\xFE\x82`@Qa\x1FZ\x91\x90aUNV[`@Q\x80\x91\x03\x90\xA2`@\x80Q\x80\x82\x01\x90\x91R_\x81R` \x81\x01a\x1F\x9D\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0CaW\xCDV[c\xFF\xFF\xFF\xFF\x16\x90R`\x9E_a\x1F\xB5` \x88\x01\x88aJ\x82V[`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x90\x81R` \x01_ _a\x1F\xDE\x84a2\x82V[\x81R` \x80\x82\x01\x92\x90\x92R`@\x01_ \x82Q\x81T\x93\x90\x92\x01Qc\xFF\xFF\xFF\xFF\x16a\x01\0\x02d\xFF\xFF\xFF\xFF\0\x19\x92\x15\x15\x92\x90\x92\x16d\xFF\xFF\xFF\xFF\xFF\x19\x90\x93\x16\x92\x90\x92\x17\x17\x90UP`\x01\x01a\x1DZV[Pa =a\x03\xCA`@\x84\x01` \x85\x01aJ\x82V[`\x01`\x01`\xA0\x1B\x03\x16c0<\xA9Va X` \x85\x01\x85aJ\x82V[a h`@\x86\x01` \x87\x01aJ\x82V[a u`@\x87\x01\x87aU\\V[`@Q\x85c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a \x94\x94\x93\x92\x91\x90aX\"V[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a \xABW__\xFD[PZ\xF1\x15\x80\x15a \xBDW=__>=_\xFD[PPPPPPV[``a\x08\x10`\x9A_a\x17X\x85a2\x82V[a \xDEa?\xE0V[a\x1C\xBC_a@:V[`\x01`\x01`\xA0\x1B\x03\x81\x16_\x90\x81R`\x9C` R`@\x81 ``\x91\x90a!\x0B\x90a4\x82V[\x90P_\x81`\x01`\x01`@\x1B\x03\x81\x11\x15a!&Wa!&aH\xDBV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a!jW\x81` \x01[`@\x80Q\x80\x82\x01\x90\x91R_\x80\x82R` \x82\x01R\x81R` \x01\x90`\x01\x90\x03\x90\x81a!DW\x90P[P\x90P_[\x82\x81\x10\x15a\t\xFFW`\x01`\x01`\xA0\x1B\x03\x85\x16_\x90\x81R`\x9C` R`@\x90 a!\x9C\x90a\t\xA5\x90\x83a4\x8BV[\x82\x82\x81Q\x81\x10a!\xAEWa!\xAEaU\x03V[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a!oV[``_\x84Q`\x01`\x01`@\x1B\x03\x81\x11\x15a!\xDDWa!\xDDaH\xDBV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\"\x16W\x81` \x01[a\"\x03aH\xA8V[\x81R` \x01\x90`\x01\x90\x03\x90\x81a!\xFBW\x90P[P\x90P_[\x85Q\x81\x10\x15a\r\xBFWa\"H\x86\x82\x81Q\x81\x10a\"9Wa\"9aU\x03V[` \x02` \x01\x01Q\x86\x86a\x07sV[\x82\x82\x81Q\x81\x10a\"ZWa\"ZaU\x03V[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a\"\x1BV[``_\x83Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\"\x89Wa\"\x89aH\xDBV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\"\xB2W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P_[\x84Q\x81\x10\x15a\r\xBFW`\x01`\x01`\xA0\x1B\x03\x86\x16_\x90\x81R`\xA1` R`@\x81 \x86Qa#'\x92\x87\x92\x91\x89\x90\x86\x90\x81\x10a\"\xF2Wa\"\xF2aU\x03V[` \x02` \x01\x01Q`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x90\x81R` \x01_ a@\x8B\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x82\x82\x81Q\x81\x10a#9Wa#9aU\x03V[`\x01`\x01`@\x1B\x03\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01R`\x01\x01a\"\xB7V[`fT_\x90`\x01\x90\x81\x16\x03a#\x81W`@Qc\x84\nH\xD5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a#\x8A\x83a4\xADV[a#\xA7W`@QcH\xF5\xC3\xED`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[___a#\xB3\x86a-\xC2V[\x91P\x91P\x81a#\xD5W`@Qc\xFAU\xFC\x81`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x91P_\x90P[\x83Q\x81\x10\x15a\r\x0CW\x83\x81\x81Q\x81\x10a#\xF6Wa#\xF6aU\x03V[` \x02` \x01\x01Q`@\x01QQ\x84\x82\x81Q\x81\x10a$\x15Wa$\x15aU\x03V[` \x02` \x01\x01Q` \x01QQ\x14a$@W`@QcCqJ\xFD`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x84\x82\x81Q\x81\x10a$SWa$SaU\x03V[` \x90\x81\x02\x91\x90\x91\x01\x81\x01QQ\x80\x82\x01Q\x81Q`\x01`\x01`\xA0\x1B\x03\x16_\x90\x81R`\x98\x90\x93R`@\x90\x92 \x90\x92Pa$\x93\x91c\xFF\xFF\xFF\xFF\x90\x81\x16\x90a4\x96\x16V[a$\xB0W`@Qc\x1F\xB1pU`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a$\xBB\x87\x83a\x07\x9BV[\x90P_[\x86\x84\x81Q\x81\x10a$\xD1Wa$\xD1aU\x03V[` \x02` \x01\x01Q` \x01QQ\x81\x10\x15a'\xFAW_\x87\x85\x81Q\x81\x10a$\xF8Wa$\xF8aU\x03V[` \x02` \x01\x01Q` \x01Q\x82\x81Q\x81\x10a%\x15Wa%\x15aU\x03V[` \x02` \x01\x01Q\x90Pa%,\x89\x82a\xFF\xFFa;oV[__a%;\x8Ba\x07\x89\x88a2\x82V[\x91P\x91P\x80`@\x01Qc\xFF\xFF\xFF\xFF\x16_\x14a%iW`@Qc\r\x8F\xCB\xE3`\xE4\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a%v\x87\x85\x84\x89a@\x9FV[\x90Pa%\xBB\x82_\x01Q\x8C\x8A\x81Q\x81\x10a%\x91Wa%\x91aU\x03V[` \x02` \x01\x01Q`@\x01Q\x87\x81Q\x81\x10a%\xAEWa%\xAEaU\x03V[` \x02` \x01\x01Qa@\xD5V[`\x0F\x0B` \x83\x01\x81\x90R_\x03a%\xE4W`@QcF\x06\x17\x93`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x82` \x01Q`\x0F\x0B\x12\x15a'(W\x80\x15a&\xAAWa&ea&\x05\x88a2\x82V[`\x01`\x01`\xA0\x1B\x03\x80\x8F\x16_\x90\x81R`\xA3` \x90\x81R`@\x80\x83 \x93\x8A\x16\x83R\x92\x90R \x90\x81T`\x01`\x80\x1B\x90\x81\x90\x04`\x0F\x0B_\x81\x81R`\x01\x80\x86\x01` R`@\x90\x91 \x93\x90\x93U\x83T`\x01`\x01`\x80\x1B\x03\x90\x81\x16\x93\x90\x91\x01\x16\x02\x17\x90UV[a&\x8F\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0CaW\xCDV[a&\x9A\x90`\x01aW\xCDV[c\xFF\xFF\xFF\xFF\x16`@\x83\x01Ra'\x95V[a&\xBC\x83` \x01Q\x83` \x01Qa8\xAEV[`\x01`\x01`@\x1B\x03\x16` \x84\x01R\x8AQ\x8B\x90\x89\x90\x81\x10a&\xDEWa&\xDEaU\x03V[` \x02` \x01\x01Q`@\x01Q\x85\x81Q\x81\x10a&\xFBWa&\xFBaU\x03V[` \x90\x81\x02\x91\x90\x91\x01\x81\x01Q`\x01`\x01`@\x1B\x03\x16\x83R_\x90\x83\x01Rc\xFF\xFF\xFF\xFFC\x16`@\x83\x01Ra'\x95V[_\x82` \x01Q`\x0F\x0B\x13\x15a'\x95Wa'I\x83` \x01Q\x83` \x01Qa8\xAEV[`\x01`\x01`@\x1B\x03\x90\x81\x16` \x85\x01\x81\x90R\x84Q\x90\x91\x16\x10\x15a'\x7FW`@Qcl\x9B\xE0\xBF`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a'\x89\x89CaW\xCDV[c\xFF\xFF\xFF\xFF\x16`@\x83\x01R[a'\xAA\x8Ca'\xA2\x89a2\x82V[\x86\x86\x86a8\xCDV[_Q` a[7_9_Q\x90_R\x8C\x88\x86a'\xCC\x86_\x01Q\x87` \x01Qa8\xAEV[\x86`@\x01Q`@Qa'\xE2\x95\x94\x93\x92\x91\x90aV[V[`@Q\x80\x91\x03\x90\xA1PP`\x01\x90\x92\x01\x91Pa$\xBF\x90PV[PPP`\x01\x01a#\xDBV[`\x01`\x01`\xA0\x1B\x03\x80\x83\x16_\x90\x81R`\xA1` \x90\x81R`@\x80\x83 \x93\x85\x16\x83R\x92\x90R\x90\x81 a\x07\x94\x90a@\xECV[\x82a(>\x81a4\xADV[a([W`@Qc\x93-\x94\xF7`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x84\x16_\x90\x81R`\xA4` R`@\x90 T`\xFF\x16a(\x9DW`\x01`\x01`\xA0\x1B\x03\x84\x16_\x90\x81R`\xA4` R`@\x90 \x80T`\xFF\x19\x16`\x01\x17\x90U[\x83`\x01`\x01`\xA0\x1B\x03\x16\x7F\xA8\x9C\x1D\xC2C\xD8\x90\x8A\x96\xDD\x84\x94K\xCC\x97\xD6\xBCj\xC0\r\xD7\x8E b\x15v\xBEj<\x947\x13\x84\x84`@Qa(\xD8\x92\x91\x90aXNV[`@Q\x80\x91\x03\x90\xA2PPPPV[`fT`\x02\x90`\x04\x90\x81\x16\x03a)\x0FW`@Qc\x84\nH\xD5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x82a)\x19\x81a4\xADV[a)6W`@Qc\x93-\x94\xF7`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@Qc6\xB8{\xD7`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90cmp\xF7\xAE\x90a)\x82\x90\x87\x90`\x04\x01aM\xDBV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a)\x9DW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a)\xC1\x91\x90aT\xE4V[a)\xDEW`@Qc\xCC\xEA\x9Eo`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_[a)\xED` \x85\x01\x85aU\\V[\x90P\x81\x10\x15a+\xB5W`@\x80Q\x80\x82\x01\x90\x91R_\x90\x80a*\x10` \x88\x01\x88aJ\x82V[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x86\x80` \x01\x90a*.\x91\x90aU\\V[\x85\x81\x81\x10a*>Wa*>aU\x03V[\x90P` \x02\x01` \x81\x01\x90a*S\x91\x90aU5V[c\xFF\xFF\xFF\xFF\x90\x81\x16\x90\x91R` \x80\x83\x01Q\x83Q`\x01`\x01`\xA0\x1B\x03\x16_\x90\x81R`\x98\x90\x92R`@\x90\x91 \x92\x93Pa*\x8F\x92\x91\x90\x81\x16\x90a4\x96\x16V[a*\xACW`@Qc\x1F\xB1pU`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a*\xB6\x86\x82a\x07\x9BV[\x15a*\xD4W`@Qclln'`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a*\xFDa*\xE0\x82a2\x82V[`\x01`\x01`\xA0\x1B\x03\x88\x16_\x90\x81R`\x9C` R`@\x90 \x90a5WV[Pa+)\x86`\x9A_a+\x0E\x85a2\x82V[\x81R` \x01\x90\x81R` \x01_ a5b\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[P\x85`\x01`\x01`\xA0\x1B\x03\x16\x7FC#.\xDF\x90qu=#!\xE5\xFA~\x01\x83c\xEE$\x8E_!B\xE6\xC0\x8E\xDD2e\xBF\xB4\x89^\x82`@Qa+c\x91\x90aUNV[`@Q\x80\x91\x03\x90\xA2`\x01`\x01`\xA0\x1B\x03\x86\x16_\x90\x81R`\x9E` R`@\x81 `\x01\x91a+\x8E\x84a2\x82V[\x81R` \x81\x01\x91\x90\x91R`@\x01_ \x80T`\xFF\x19\x16\x91\x15\x15\x91\x90\x91\x17\x90UP`\x01\x01a)\xE0V[Pa+\xC6a\x03\xCA` \x85\x01\x85aJ\x82V[`\x01`\x01`\xA0\x1B\x03\x16c\xC6?\xD5\x02\x85a+\xE2` \x87\x01\x87aJ\x82V[a+\xEF` \x88\x01\x88aU\\V[a+\xFC`@\x8A\x01\x8AaV\xACV[`@Q\x87c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a,\x1D\x96\x95\x94\x93\x92\x91\x90aXaV[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a,4W__\xFD[PZ\xF1\x15\x80\x15a\x1A W=__>=_\xFD[_a\x08\x10`\x9A_a,V\x85a2\x82V[\x81R` \x01\x90\x81R` \x01_ a4\x82V[\x83a,r\x81a4\xADV[a,\x8FW`@Qc\x93-\x94\xF7`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@\x80Q\x80\x82\x01\x82R`\x01`\x01`\xA0\x1B\x03\x87\x16\x80\x82Rc\xFF\xFF\xFF\xFF\x80\x88\x16` \x80\x85\x01\x82\x90R_\x93\x84R`\x98\x90R\x93\x90\x91 \x91\x92a,\xCE\x92\x91a4\x96\x16V[a,\xEBW`@Qc\x1F\xB1pU`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a,\xF5\x82a2\x82V[\x90P_[\x84\x81\x10\x15a\x1A Wa->\x86\x86\x83\x81\x81\x10a-\x16Wa-\x16aU\x03V[\x90P` \x02\x01` \x81\x01\x90a-+\x91\x90aJ\x82V[_\x84\x81R`\x99` R`@\x90 \x90a?\xCCV[a-[W`@Qc1\xBC4'`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x7F{K\x07=\x80\xDC\xACU\xA1\x11w\xD8E\x9A\xD9\xF6d\xCE\xEB\x91\xF7\x1F'\x16{\xB1O\x81R\xA7\xEE\xEE\x83\x87\x87\x84\x81\x81\x10a-\x8FWa-\x8FaU\x03V[\x90P` \x02\x01` \x81\x01\x90a-\xA4\x91\x90aJ\x82V[`@Qa-\xB2\x92\x91\x90aU\xA1V[`@Q\x80\x91\x03\x90\xA1`\x01\x01a,\xF9V[`\x01`\x01`\xA0\x1B\x03\x81\x16_\x90\x81R`\x9B` \x90\x81R`@\x80\x83 \x81Q`\x80\x81\x01\x83R\x90Tc\xFF\xFF\xFF\xFF\x80\x82\x16\x80\x84R`\xFF`\x01` \x1B\x84\x04\x16\x15\x15\x95\x84\x01\x86\x90R`\x01`(\x1B\x83\x04\x82\x16\x94\x84\x01\x94\x90\x94R`\x01`H\x1B\x90\x91\x04\x16``\x82\x01\x81\x90R\x84\x93\x91\x92\x91\x90\x15\x80\x15\x90a.AWP\x82``\x01Qc\xFF\xFF\xFF\xFF\x16C\x10\x15[\x15a.PWPP`@\x81\x01Q`\x01[\x95\x90\x94P\x92PPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16_\x90\x81R`\x98` R`@\x81 a\x08\x10\x90a4\x82V[`\x01`\x01`\xA0\x1B\x03\x82\x16_\x90\x81R`\x9F` R`@\x81 ``\x91\x90a\x08\x0C\x90\x82a\x17X\x86a2\x82V[_Ta\x01\0\x90\x04`\xFF\x16\x15\x80\x80\x15a.\xC1WP_T`\x01`\xFF\x90\x91\x16\x10[\x80a.\xDAWP0;\x15\x80\x15a.\xDAWP_T`\xFF\x16`\x01\x14[a/BW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01Rm\x19\x1EH\x1A[\x9A]\x1AX[\x1A^\x99Y`\x92\x1B`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[_\x80T`\xFF\x19\x16`\x01\x17\x90U\x80\x15a/cW_\x80Ta\xFF\0\x19\x16a\x01\0\x17\x90U[a/l\x82a4EV[a/u\x83a@:V[\x80\x15a/\xBBW_\x80Ta\xFF\0\x19\x16\x90U`@Q`\x01\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01[`@Q\x80\x91\x03\x90\xA1[PPPV[\x81a/\xCA\x81a4\xADV[a/\xE7W`@Qc\x93-\x94\xF7`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@Qc\xB5&W\x87`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x83\x16\x90c\xB5&W\x87\x90a0\x13\x90\x86\x90`\x04\x01aM\xDBV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a0.W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a0R\x91\x90aT\xE4V[a0oW`@Qc\x1D\x0B\x13\xC1`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x83\x81\x16_\x90\x81R`\x97` R`@\x90 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x91\x84\x16\x91\x90\x91\x17\x90U\x7F*\xE9E\xC4\x0CD\xDC\x0E\xC2c\xF9V\t\xC3\xFD\xC6\x95.\n\xEF\xA2-ct\xE4O,\x99z\xCE\xDF\x85\x83a0\xC7\x81a\r\xC8V[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x93\x84\x16\x81R\x92\x90\x91\x16` \x83\x01R\x01a/\xB2V[a0\xEFa?\xE0V[`\x01`\x01`\xA0\x1B\x03\x81\x16a1TW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01Reddress`\xD0\x1B`d\x82\x01R`\x84\x01a/9V[a1]\x81a@:V[PV[__a\r\xBF\x84\x84a>RV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\xEA\xB6mz`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a1\xC8W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a1\xEC\x91\x90aX\xADV[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a2\x1DW`@QcyH!\xFF`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`fT\x80\x19\x82\x19\x81\x16\x14a2DW`@Qc\xC6\x1D\xCA]`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`f\x82\x90U`@Q\x82\x81R3\x90\x7F5\x82\xD1\x82\x8E&\xBFV\xBD\x80\x15\x02\xBC\x02\x1A\xC0\xBC\x8A\xFBW\xC8&\xE4\x98kEY<\x8F\xAD8\x9C\x90` \x01`@Q\x80\x91\x03\x90\xA2PPV[_\x81_\x01Q\x82` \x01Qc\xFF\xFF\xFF\xFF\x16`@Q` \x01a2\xCD\x92\x91\x90``\x92\x90\x92\x1Bk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x82R`\xA0\x1B`\x01`\x01`\xA0\x1B\x03\x19\x16`\x14\x82\x01R` \x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@Ra\x08\x10\x90aX\xC8V[`@\x80Q\x80\x82\x01\x90\x91R_\x80\x82R` \x82\x01Ra3\0aH\xA8V[`@\x80Q\x80\x82\x01\x82R`\x01`\x01`\xA0\x1B\x03\x80\x88\x16_\x90\x81R`\xA1` \x90\x81R\x84\x82 \x92\x88\x16\x82R\x91\x90\x91R\x91\x82 \x81\x90a39\x90a@\xECV[`\x01`\x01`@\x1B\x03\x90\x81\x16\x82R`\x01`\x01`\xA0\x1B\x03\x89\x81\x16_\x81\x81R`\xA2` \x90\x81R`@\x80\x83 \x94\x8C\x16\x80\x84R\x94\x82R\x80\x83 T\x86\x16\x96\x82\x01\x96\x90\x96R\x91\x81R`\xA0\x82R\x84\x81 \x8B\x82R\x82R\x84\x81 \x92\x81R\x91\x81R\x90\x83\x90 \x83Q``\x81\x01\x85R\x90T\x92\x83\x16\x81R`\x01`@\x1B\x83\x04`\x0F\x0B\x91\x81\x01\x91\x90\x91R`\x01`\xC0\x1B\x90\x91\x04c\xFF\xFF\xFF\xFF\x16\x91\x81\x01\x82\x90R\x91\x92PC\x10\x15a3\xDBW\x90\x92P\x90Pa4=V[a3\xEC\x81_\x01Q\x82` \x01Qa8\xAEV[`\x01`\x01`@\x1B\x03\x16\x81R` \x81\x01Q_`\x0F\x91\x90\x91\x0B\x12\x15a4*Wa4\x1B\x82` \x01Q\x82` \x01Qa8\xAEV[`\x01`\x01`@\x1B\x03\x16` \x83\x01R[_`@\x82\x01\x81\x90R` \x82\x01R\x90\x92P\x90P[\x93P\x93\x91PPV[`f\x81\x90U`@Q\x81\x81R3\x90\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=\x90` \x01`@Q\x80\x91\x03\x90\xA2PV[_a\x08\x10\x82T\x90V[_a\x07\x94\x83\x83a@\xFFV[_\x81\x81R`\x01\x83\x01` R`@\x81 T\x15\x15a\x07\x94V[`@Qc\x1B\xEB+\x97`\xE3\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x82\x81\x16`\x04\x83\x01R3`$\x83\x01R0`D\x83\x01R_\x805`\x01`\x01`\xE0\x1B\x03\x19\x16`d\x84\x01R\x91\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x16\x90c\xDFY\\\xB8\x90`\x84\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a53W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x08\x10\x91\x90aT\xE4V[_a\x07\x94\x83\x83aA%V[_a\x07\x94\x83`\x01`\x01`\xA0\x1B\x03\x84\x16aA%V[``\x83Q`\x01`\x01`@\x1B\x03\x81\x11\x15a5\x91Wa5\x91aH\xDBV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a5\xC4W\x81` \x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a5\xAFW\x90P[P\x90P_\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\xF0\xE0\xE6v\x86\x86`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a6\x16\x92\x91\x90aX\xEBV[_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a60W=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra6W\x91\x90\x81\x01\x90aY\x0FV[\x90P_[\x85Q\x81\x10\x15a8YW_\x86\x82\x81Q\x81\x10a6wWa6waU\x03V[` \x02` \x01\x01Q\x90P\x85Q`\x01`\x01`@\x1B\x03\x81\x11\x15a6\x9AWa6\x9AaH\xDBV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a6\xC3W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x84\x83\x81Q\x81\x10a6\xD6Wa6\xD6aU\x03V[` \x90\x81\x02\x91\x90\x91\x01\x01R_[\x86Q\x81\x10\x15a8OW_\x87\x82\x81Q\x81\x10a6\xFFWa6\xFFaU\x03V[` \x90\x81\x02\x91\x90\x91\x01\x81\x01Q`\x01`\x01`\xA0\x1B\x03\x80\x86\x16_\x90\x81R`\xA1\x84R`@\x80\x82 \x92\x84\x16\x82R\x91\x90\x93R\x82 \x90\x92Pa7:\x90a@\xECV[\x90P\x80`\x01`\x01`@\x1B\x03\x16_\x03a7SWPPa8GV[_a7_\x85\x8D\x85a\x07sV[\x90P\x88c\xFF\xFF\xFF\xFF\x16\x81`@\x01Qc\xFF\xFF\xFF\xFF\x16\x11\x15\x80\x15a7\x87WP_\x81` \x01Q`\x0F\x0B\x12[\x15a7\xA9Wa7\x9D\x81_\x01Q\x82` \x01Qa8\xAEV[`\x01`\x01`@\x1B\x03\x16\x81R[\x80Q_\x90a7\xC4\x90`\x01`\x01`@\x1B\x03\x90\x81\x16\x90\x85\x16a8\x9AV[\x90Pa8\x0B\x81\x89\x89\x81Q\x81\x10a7\xDCWa7\xDCaU\x03V[` \x02` \x01\x01Q\x87\x81Q\x81\x10a7\xF5Wa7\xF5aU\x03V[` \x02` \x01\x01QaAq\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x89\x88\x81Q\x81\x10a8\x1DWa8\x1DaU\x03V[` \x02` \x01\x01Q\x86\x81Q\x81\x10a86Wa86aU\x03V[` \x02` \x01\x01\x81\x81RPPPPPP[`\x01\x01a6\xE3V[PP`\x01\x01a6[V[PP\x94\x93PPPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16_\x90\x81R`\x01\x83\x01` R`@\x81 T\x15\x15a\x07\x94V[_a\x07\x94\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0`\x01aA\x85V[_a\x07\x94\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84aA\xDEV[_a\x07\x94a8\xC5\x83`\x01`\x01`@\x1B\x03\x86\x16aV.V[`\x0F\x0BaB\xC3V[` \x80\x83\x01Q`\x01`\x01`\xA0\x1B\x03\x80\x88\x16_\x90\x81R`\xA2\x84R`@\x80\x82 \x92\x88\x16\x82R\x91\x90\x93R\x90\x91 T`\x01`\x01`@\x1B\x03\x90\x81\x16\x91\x16\x14a9\x8FW` \x82\x81\x01\x80Q`\x01`\x01`\xA0\x1B\x03\x88\x81\x16_\x90\x81R`\xA2\x85R`@\x80\x82 \x92\x89\x16\x82R\x91\x90\x94R\x92\x83\x90 \x80Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\x01`\x01`@\x1B\x03\x90\x92\x16\x91\x90\x91\x17\x90UQ\x90Q\x7F\xAC\xF9\t_\xEB:7\x0C\x9C\xF6\x92B\x1Ci\xEF2\rM\xB5\xC6nj})\xC7iN\xB0#d\xFCU\x91a9\x86\x91\x88\x91\x87\x91aZ\x1BV[`@Q\x80\x91\x03\x90\xA1[`\x01`\x01`\xA0\x1B\x03\x80\x86\x16_\x90\x81R`\xA0` \x90\x81R`@\x80\x83 \x88\x84R\x82R\x80\x83 \x93\x87\x16\x83R\x92\x81R\x90\x82\x90 \x83Q\x81T\x92\x85\x01Q\x93\x85\x01Qc\xFF\xFF\xFF\xFF\x16`\x01`\xC0\x1B\x02c\xFF\xFF\xFF\xFF`\xC0\x1B\x19`\x01`\x01`\x80\x1B\x03\x86\x16`\x01`@\x1B\x02`\x01`\x01`\xC0\x1B\x03\x19\x90\x95\x16`\x01`\x01`@\x1B\x03\x90\x93\x16\x92\x90\x92\x17\x93\x90\x93\x17\x16\x91\x90\x91\x17\x90U`\x0F\x0B\x15a:qW`\x01`\x01`\xA0\x1B\x03\x85\x16_\x90\x81R`\x9F` \x90\x81R`@\x80\x83 \x87\x84R\x90\x91R\x90 a:I\x90\x84a5bV[P`\x01`\x01`\xA0\x1B\x03\x85\x16_\x90\x81R`\x9D` R`@\x90 a:k\x90\x85a5WV[Pa\r\x0CV[\x80Q`\x01`\x01`@\x1B\x03\x16_\x03a\r\x0CW`\x01`\x01`\xA0\x1B\x03\x85\x16_\x90\x81R`\x9F` \x90\x81R`@\x80\x83 \x87\x84R\x90\x91R\x90 a:\xAE\x90\x84a?\xCCV[P`\x01`\x01`\xA0\x1B\x03\x85\x16_\x90\x81R`\x9F` \x90\x81R`@\x80\x83 \x87\x84R\x90\x91R\x90 a:\xDA\x90a4\x82V[_\x03a\r\x0CW`\x01`\x01`\xA0\x1B\x03\x85\x16_\x90\x81R`\x9D` R`@\x90 a \xBD\x90\x85a?\xC1V[`\x01`\x01`\xA0\x1B\x03\x80\x84\x16_\x90\x81R`\xA1` \x90\x81R`@\x80\x83 \x93\x86\x16\x83R\x92\x90R a;0\x90C\x83aC.V[\x7F\x1CdX\x07\x9AA\x07}\0<\x11\xFA\xF9\xBF\t~i;\xD6yy\xE4\xE6P\x0B\xAC{)\xDBw\x9B\\\x83\x83\x83`@Qa/\xB2\x93\x92\x91\x90aZ\x1BV[``_a\x07\x94\x83aCBV[`\x01`\x01`\xA0\x1B\x03\x83\x81\x16_\x90\x81R`\xA3` \x90\x81R`@\x80\x83 \x93\x86\x16\x83R\x92\x90R\x90\x81 T`\x0F\x81\x81\x0B`\x01`\x80\x1B\x90\x92\x04\x90\x0B\x03[_\x81\x11\x80\x15a;\xB9WP\x82a\xFF\xFF\x16\x82\x10[\x15a\r\x0CW`\x01`\x01`\xA0\x1B\x03\x80\x86\x16_\x90\x81R`\xA3` \x90\x81R`@\x80\x83 \x93\x88\x16\x83R\x92\x90R\x90\x81 a;\xED\x90aC\x9BV[\x90P__a;\xFC\x88\x84\x89a2\xE5V[\x91P\x91P\x80`@\x01Qc\xFF\xFF\xFF\xFF\x16C\x10\x15a<\x1AWPPPa\r\x0CV[a<'\x88\x84\x89\x85\x85a8\xCDV[`\x01`\x01`\xA0\x1B\x03\x80\x89\x16_\x90\x81R`\xA3` \x90\x81R`@\x80\x83 \x93\x8B\x16\x83R\x92\x90R a<T\x90aC\xEDV[Pa<^\x85aZGV[\x94Pa<i\x84aZ_V[\x93PPPPa;\xA7V[``_a<\x7F\x83aDaV[`@\x80Q` \x80\x82R\x81\x83\x01\x90\x92R\x91\x92P_\x91\x90` \x82\x01\x81\x806\x837PPP\x91\x82RP` \x81\x01\x92\x90\x92RP\x90V[`\x01`\x01`\xA0\x1B\x03\x82\x16_\x90\x81R`\x9B` \x90\x81R`@\x91\x82\x90 \x82Q`\x80\x81\x01\x84R\x90Tc\xFF\xFF\xFF\xFF\x80\x82\x16\x83R`\xFF`\x01` \x1B\x83\x04\x16\x15\x15\x93\x83\x01\x93\x90\x93R`\x01`(\x1B\x81\x04\x83\x16\x93\x82\x01\x93\x90\x93R`\x01`H\x1B\x90\x92\x04\x16``\x82\x01\x81\x90R\x15\x80\x15\x90a=*WP\x80``\x01Qc\xFF\xFF\xFF\xFF\x16C\x10\x15[\x15a=DW`@\x81\x01Qc\xFF\xFF\xFF\xFF\x16\x81R`\x01` \x82\x01R[c\xFF\xFF\xFF\xFF\x82\x16`@\x82\x01Ra=z\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0CaW\xCDV[a=\x85\x90`\x01aW\xCDV[c\xFF\xFF\xFF\xFF\x90\x81\x16``\x83\x81\x01\x91\x82R`\x01`\x01`\xA0\x1B\x03\x86\x16_\x81\x81R`\x9B` \x90\x81R`@\x91\x82\x90 \x87Q\x81T\x83\x8A\x01Q\x85\x8B\x01Q\x98Q\x92\x8A\x16d\xFF\xFF\xFF\xFF\xFF\x19\x90\x92\x16\x91\x90\x91\x17`\x01` \x1B\x91\x15\x15\x91\x90\x91\x02\x17l\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\x19\x16`\x01`(\x1B\x97\x89\x16\x97\x90\x97\x02c\xFF\xFF\xFF\xFF`H\x1B\x19\x16\x96\x90\x96\x17`\x01`H\x1B\x96\x88\x16\x96\x87\x02\x17\x90U\x81Q\x92\x83R\x94\x87\x16\x94\x82\x01\x94\x90\x94R\x92\x83\x01\x91\x90\x91R\x7FN\x85u\x1Dc1Pllb3_ ~\xB3\x1F\x12\xA6\x1EW\x0F4\xF5\xC1v@0\x87\x85\xC6\xD4\xDB\x91\x01a/\xB2V[`\x01`\x01`\xA0\x1B\x03\x82\x81\x16_\x81\x81R`\xA2` \x90\x81R`@\x80\x83 \x94\x86\x16\x80\x84R\x94\x82R\x80\x83 T\x93\x83R`\xA3\x82R\x80\x83 \x94\x83R\x93\x90R\x91\x82 T`\x01`\x01`@\x1B\x03\x90\x91\x16\x91\x90`\x0F\x81\x81\x0B`\x01`\x80\x1B\x90\x92\x04\x90\x0B\x03\x81[\x81\x81\x10\x15a?}W`\x01`\x01`\xA0\x1B\x03\x80\x87\x16_\x90\x81R`\xA3` \x90\x81R`@\x80\x83 \x93\x89\x16\x83R\x92\x90R\x90\x81 a>\xE5\x90\x83aD\x88V[`\x01`\x01`\xA0\x1B\x03\x88\x81\x16_\x90\x81R`\xA0` \x90\x81R`@\x80\x83 \x85\x84R\x82R\x80\x83 \x93\x8B\x16\x83R\x92\x81R\x90\x82\x90 \x82Q``\x81\x01\x84R\x90T`\x01`\x01`@\x1B\x03\x81\x16\x82R`\x01`@\x1B\x81\x04`\x0F\x0B\x92\x82\x01\x92\x90\x92R`\x01`\xC0\x1B\x90\x91\x04c\xFF\xFF\xFF\xFF\x16\x91\x81\x01\x82\x90R\x91\x92PC\x10\x15a?`WPPa?}V[a?n\x86\x82` \x01Qa8\xAEV[\x95PPP\x80`\x01\x01\x90Pa>\xADV[P`\x01`\x01`\xA0\x1B\x03\x80\x86\x16_\x90\x81R`\xA1` \x90\x81R`@\x80\x83 \x93\x88\x16\x83R\x92\x90R \x83\x90a?\xAD\x90a@\xECV[a?\xB7\x91\x90aU\xEEV[\x91PP\x92P\x92\x90PV[_a\x07\x94\x83\x83aD\xF7V[_a\x07\x94\x83`\x01`\x01`\xA0\x1B\x03\x84\x16aD\xF7V[`3T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x1C\xBCW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a/9V[`3\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x90\x93U`@Q\x91\x16\x91\x90\x82\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90_\x90\xA3PPV[_a\x07\x94\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0aE\xDAV[_a@\xB0\x84`\x99_a\x11\n\x89a2\x82V[\x80\x15a@\xB9WP\x81[\x80\x15a\x1C\xDBWPP\x90Q`\x01`\x01`@\x1B\x03\x16\x15\x15\x93\x92PPPV[_a\x07\x94`\x01`\x01`@\x1B\x03\x80\x85\x16\x90\x84\x16aZtV[_a\x08\x10\x82g\r\xE0\xB6\xB3\xA7d\0\0aF/V[_\x82_\x01\x82\x81T\x81\x10aA\x14WaA\x14aU\x03V[\x90_R` _ \x01T\x90P\x92\x91PPV[_\x81\x81R`\x01\x83\x01` R`@\x81 TaAjWP\x81T`\x01\x81\x81\x01\x84U_\x84\x81R` \x80\x82 \x90\x93\x01\x84\x90U\x84T\x84\x82R\x82\x86\x01\x90\x93R`@\x90 \x91\x90\x91Ua\x08\x10V[P_a\x08\x10V[_a\x07\x94\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0aA\xDEV[__aA\x92\x86\x86\x86aA\xDEV[\x90P`\x01\x83`\x02\x81\x11\x15aA\xA8WaA\xA8aZ\xA1V[\x14\x80\x15aA\xC4WP_\x84\x80aA\xBFWaA\xBFaZ\xB5V[\x86\x88\t\x11[\x15a\x1C\xDBWaA\xD4`\x01\x82aZ\xC9V[\x96\x95PPPPPPV[_\x80\x80_\x19\x85\x87\t\x85\x87\x02\x92P\x82\x81\x10\x83\x82\x03\x03\x91PP\x80_\x03aB\x15W\x83\x82\x81aB\x0BWaB\x0BaZ\xB5V[\x04\x92PPPa\x07\x94V[\x80\x84\x11aB\\W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x15`$\x82\x01RtMath: mulDiv overflow`X\x1B`D\x82\x01R`d\x01a/9V[_\x84\x86\x88\t`\x02`\x01\x87\x19\x81\x01\x88\x16\x97\x88\x90\x04`\x03\x81\x02\x83\x18\x80\x82\x02\x84\x03\x02\x80\x82\x02\x84\x03\x02\x80\x82\x02\x84\x03\x02\x80\x82\x02\x84\x03\x02\x80\x82\x02\x84\x03\x02\x90\x81\x02\x90\x92\x03\x90\x91\x02_\x88\x90\x03\x88\x90\x04\x90\x91\x01\x85\x83\x11\x90\x94\x03\x93\x90\x93\x02\x93\x03\x94\x90\x94\x04\x91\x90\x91\x17\x02\x94\x93PPPPV[_`\x01`\x01`@\x1B\x03\x82\x11\x15aC*W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FSafeCast: value doesn't fit in 6`D\x82\x01Re4 bits`\xD0\x1B`d\x82\x01R`\x84\x01a/9V[P\x90V[a/\xBB\x83\x83`\x01`\x01`@\x1B\x03\x84\x16aFfV[``\x81_\x01\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15aC\x8FW` \x02\x82\x01\x91\x90_R` _ \x90[\x81T\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11aC{W[PPPPP\x90P\x91\x90PV[_aC\xB5\x82T`\x0F\x81\x81\x0B`\x01`\x80\x1B\x90\x92\x04\x90\x0B\x13\x15\x90V[\x15aC\xD3W`@Qc\x1E\xD9P\x95`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[P\x80T`\x0F\x0B_\x90\x81R`\x01\x90\x91\x01` R`@\x90 T\x90V[_aD\x07\x82T`\x0F\x81\x81\x0B`\x01`\x80\x1B\x90\x92\x04\x90\x0B\x13\x15\x90V[\x15aD%W`@Qc\x1E\xD9P\x95`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[P\x80T`\x0F\x0B_\x81\x81R`\x01\x80\x84\x01` R`@\x82 \x80T\x92\x90U\x83T`\x01`\x01`\x80\x1B\x03\x19\x16\x92\x01`\x01`\x01`\x80\x1B\x03\x16\x91\x90\x91\x17\x90\x91U\x90V[_`\xFF\x82\x16`\x1F\x81\x11\x15a\x08\x10W`@Qc,\xD4J\xC3`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[__aD\xAAaD\x96\x84aGiV[\x85TaD\xA5\x91\x90`\x0F\x0BaZ\xDCV[aG\xD2V[\x84T\x90\x91P`\x01`\x80\x1B\x90\x04`\x0F\x90\x81\x0B\x90\x82\x90\x0B\x12aD\xDDW`@Qc-\x04\x83\xC5`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x0F\x0B_\x90\x81R`\x01\x93\x90\x93\x01` RPP`@\x90 T\x90V[_\x81\x81R`\x01\x83\x01` R`@\x81 T\x80\x15aE\xD1W_aE\x19`\x01\x83aU\xDBV[\x85T\x90\x91P_\x90aE,\x90`\x01\x90aU\xDBV[\x90P\x81\x81\x14aE\x8BW_\x86_\x01\x82\x81T\x81\x10aEJWaEJaU\x03V[\x90_R` _ \x01T\x90P\x80\x87_\x01\x84\x81T\x81\x10aEjWaEjaU\x03V[_\x91\x82R` \x80\x83 \x90\x91\x01\x92\x90\x92U\x91\x82R`\x01\x88\x01\x90R`@\x90 \x83\x90U[\x85T\x86\x90\x80aE\x9CWaE\x9Ca[\x03V[`\x01\x90\x03\x81\x81\x90_R` _ \x01_\x90U\x90U\x85`\x01\x01_\x86\x81R` \x01\x90\x81R` \x01_ _\x90U`\x01\x93PPPPa\x08\x10V[_\x91PPa\x08\x10V[\x82T_\x90\x81aE\xEB\x86\x86\x83\x85aH;V[\x90P\x80\x15aF%WaF\x0F\x86aF\x02`\x01\x84aU\xDBV[_\x91\x82R` \x90\x91 \x01\x90V[T`\x01` \x1B\x90\x04`\x01`\x01`\xE0\x1B\x03\x16a\x07\x8FV[P\x91\x94\x93PPPPV[\x81T_\x90\x80\x15aF^WaFH\x84aF\x02`\x01\x84aU\xDBV[T`\x01` \x1B\x90\x04`\x01`\x01`\xE0\x1B\x03\x16a\x08\x0CV[P\x90\x92\x91PPV[\x82T\x80\x15aG\x1CW_aF~\x85aF\x02`\x01\x85aU\xDBV[`@\x80Q\x80\x82\x01\x90\x91R\x90Tc\xFF\xFF\xFF\xFF\x80\x82\x16\x80\x84R`\x01` \x1B\x90\x92\x04`\x01`\x01`\xE0\x1B\x03\x16` \x84\x01R\x91\x92P\x90\x85\x16\x10\x15aF\xD0W`@Qc\x15\x1B\x8E?`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80Qc\xFF\xFF\xFF\xFF\x80\x86\x16\x91\x16\x03aG\x1AW\x82aF\xF1\x86aF\x02`\x01\x86aU\xDBV[\x80T`\x01`\x01`\xE0\x1B\x03\x92\x90\x92\x16`\x01` \x1B\x02c\xFF\xFF\xFF\xFF\x90\x92\x16\x91\x90\x91\x17\x90UPPPPPV[P[P`@\x80Q\x80\x82\x01\x90\x91Rc\xFF\xFF\xFF\xFF\x92\x83\x16\x81R`\x01`\x01`\xE0\x1B\x03\x91\x82\x16` \x80\x83\x01\x91\x82R\x85T`\x01\x81\x01\x87U_\x96\x87R\x95 \x91Q\x90Q\x90\x92\x16`\x01` \x1B\x02\x91\x90\x92\x16\x17\x91\x01UV[_`\x01`\x01`\xFF\x1B\x03\x82\x11\x15aC*W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`(`$\x82\x01R\x7FSafeCast: value doesn't fit in a`D\x82\x01Rg7\x104\xB7:\x19\x1A\x9B`\xC1\x1B`d\x82\x01R`\x84\x01a/9V[\x80`\x0F\x81\x90\x0B\x81\x14aH6W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`'`$\x82\x01R\x7FSafeCast: value doesn't fit in 1`D\x82\x01Rf28 bits`\xC8\x1B`d\x82\x01R`\x84\x01a/9V[\x91\x90PV[_[\x81\x83\x10\x15a\t\xFFW_aHP\x84\x84aH\x8EV[_\x87\x81R` \x90 \x90\x91Pc\xFF\xFF\xFF\xFF\x86\x16\x90\x82\x01Tc\xFF\xFF\xFF\xFF\x16\x11\x15aHzW\x80\x92PaH\x88V[aH\x85\x81`\x01aZ\xC9V[\x93P[PaH=V[_aH\x9C`\x02\x84\x84\x18a[\x17V[a\x07\x94\x90\x84\x84\x16aZ\xC9V[`@\x80Q``\x81\x01\x82R_\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x91\x90\x91R\x90V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a1]W__\xFD[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[`@Q``\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15aI\x11WaI\x11aH\xDBV[`@R\x90V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15aI?WaI?aH\xDBV[`@R\x91\x90PV[\x805c\xFF\xFF\xFF\xFF\x81\x16\x81\x14aH6W__\xFD[_`@\x82\x84\x03\x12\x15aIjW__\xFD[`@\x80Q\x90\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15aI\x8CWaI\x8CaH\xDBV[`@R\x90P\x80\x825aI\x9D\x81aH\xC7V[\x81RaI\xAB` \x84\x01aIGV[` \x82\x01RP\x92\x91PPV[___`\x80\x84\x86\x03\x12\x15aI\xC9W__\xFD[\x835aI\xD4\x81aH\xC7V[\x92PaI\xE3\x85` \x86\x01aIZV[\x91P``\x84\x015aI\xF3\x81aH\xC7V[\x80\x91PP\x92P\x92P\x92V[\x80Q`\x01`\x01`@\x1B\x03\x16\x82R` \x80\x82\x01Q`\x0F\x0B\x90\x83\x01R`@\x90\x81\x01Qc\xFF\xFF\xFF\xFF\x16\x91\x01RV[``\x81\x01a\x08\x10\x82\x84aI\xFEV[__``\x83\x85\x03\x12\x15aJHW__\xFD[\x825aJS\x81aH\xC7V[\x91PaJb\x84` \x85\x01aIZV[\x90P\x92P\x92\x90PV[_` \x82\x84\x03\x12\x15aJ{W__\xFD[P5\x91\x90PV[_` \x82\x84\x03\x12\x15aJ\x92W__\xFD[\x815a\x07\x94\x81aH\xC7V[\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x82R` \x90\x81\x01Qc\xFF\xFF\xFF\xFF\x16\x91\x01RV[_\x81Q\x80\x84R` \x84\x01\x93P` \x83\x01_[\x82\x81\x10\x15aJ\xF6WaJ\xE0\x86\x83QaJ\x9DV[`@\x95\x90\x95\x01\x94` \x91\x90\x91\x01\x90`\x01\x01aJ\xCDV[P\x93\x94\x93PPPPV[` \x81R_a\x07\x94` \x83\x01\x84aJ\xBBV[_`@\x82\x84\x03\x12\x15aK\"W__\xFD[a\x07\x94\x83\x83aIZV[__\x83`\x1F\x84\x01\x12aK<W__\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15aKRW__\xFD[` \x83\x01\x91P\x83` \x82`\x05\x1B\x85\x01\x01\x11\x15a\x17?W__\xFD[___`@\x84\x86\x03\x12\x15aK~W__\xFD[\x835aK\x89\x81aH\xC7V[\x92P` \x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aK\xA3W__\xFD[aK\xAF\x86\x82\x87\x01aK,V[\x94\x97\x90\x96P\x93\x94PPPPV[_`\x01`\x01`@\x1B\x03\x82\x11\x15aK\xD4WaK\xD4aH\xDBV[P`\x05\x1B` \x01\x90V[_\x82`\x1F\x83\x01\x12aK\xEDW__\xFD[\x815aL\0aK\xFB\x82aK\xBCV[aI\x17V[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x86\x01\x01\x92P\x85\x83\x11\x15aL!W__\xFD[` \x85\x01[\x83\x81\x10\x15aLGW\x805aL9\x81aH\xC7V[\x83R` \x92\x83\x01\x92\x01aL&V[P\x95\x94PPPPPV[___`\x80\x84\x86\x03\x12\x15aLcW__\xFD[aLm\x85\x85aIZV[\x92P`@\x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aL\x87W__\xFD[aL\x93\x86\x82\x87\x01aK\xDEV[\x92PP``\x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aL\xAEW__\xFD[aL\xBA\x86\x82\x87\x01aK\xDEV[\x91PP\x92P\x92P\x92V[_\x81Q\x80\x84R` \x84\x01\x93P` \x83\x01_[\x82\x81\x10\x15aJ\xF6W\x81Q\x86R` \x95\x86\x01\x95\x90\x91\x01\x90`\x01\x01aL\xD6V[_` \x82\x01` \x83R\x80\x84Q\x80\x83R`@\x85\x01\x91P`@\x81`\x05\x1B\x86\x01\x01\x92P` \x86\x01_[\x82\x81\x10\x15aMKW`?\x19\x87\x86\x03\x01\x84RaM6\x85\x83QaL\xC4V[\x94P` \x93\x84\x01\x93\x91\x90\x91\x01\x90`\x01\x01aM\x1AV[P\x92\x96\x95PPPPPPV[____`\xA0\x85\x87\x03\x12\x15aMjW__\xFD[aMt\x86\x86aIZV[\x93P`@\x85\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aM\x8EW__\xFD[aM\x9A\x87\x82\x88\x01aK\xDEV[\x93PP``\x85\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aM\xB5W__\xFD[aM\xC1\x87\x82\x88\x01aK\xDEV[\x92PPaM\xD0`\x80\x86\x01aIGV[\x90P\x92\x95\x91\x94P\x92PV[`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x81R` \x01\x90V[__`@\x83\x85\x03\x12\x15aN\0W__\xFD[\x825aN\x0B\x81aH\xC7V[\x91P` \x83\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aN%W__\xFD[\x83\x01`\xA0\x81\x86\x03\x12\x15aN6W__\xFD[\x80\x91PP\x92P\x92\x90PV[__`@\x83\x85\x03\x12\x15aNRW__\xFD[\x825aN]\x81aH\xC7V[\x91P` \x83\x015aN6\x81aH\xC7V[_\x81Q\x80\x84R` \x84\x01\x93P` \x83\x01_[\x82\x81\x10\x15aJ\xF6WaN\x92\x86\x83QaI\xFEV[``\x95\x90\x95\x01\x94` \x91\x90\x91\x01\x90`\x01\x01aN\x7FV[`@\x81R_aN\xBA`@\x83\x01\x85aJ\xBBV[\x82\x81\x03` \x84\x01Ra\x1C\xDB\x81\x85aNmV[_\x81Q\x80\x84R` \x84\x01\x93P` \x83\x01_[\x82\x81\x10\x15aJ\xF6W\x81Q`\x01`\x01`\xA0\x1B\x03\x16\x86R` \x95\x86\x01\x95\x90\x91\x01\x90`\x01\x01aN\xDEV[` \x81R_a\x07\x94` \x83\x01\x84aN\xCCV[__`@\x83\x85\x03\x12\x15aO(W__\xFD[\x825`\x01`\x01`@\x1B\x03\x81\x11\x15aO=W__\xFD[aOI\x85\x82\x86\x01aK\xDEV[\x92PP` \x83\x015aN6\x81aH\xC7V[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R_\x91\x84\x01\x90`@\x84\x01\x90\x83[\x81\x81\x10\x15aO\x9AW\x83Q`\x01`\x01`@\x1B\x03\x16\x83R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01aOsV[P\x90\x95\x94PPPPPV[_____``\x86\x88\x03\x12\x15aO\xB9W__\xFD[\x855aO\xC4\x81aH\xC7V[\x94P` \x86\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aO\xDEW__\xFD[aO\xEA\x88\x82\x89\x01aK,V[\x90\x95P\x93PP`@\x86\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aP\x08W__\xFD[aP\x14\x88\x82\x89\x01aK,V[\x96\x99\x95\x98P\x93\x96P\x92\x94\x93\x92PPPV[____``\x85\x87\x03\x12\x15aP8W__\xFD[\x845aPC\x81aH\xC7V[\x93PaPQ` \x86\x01aIGV[\x92P`@\x85\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aPkW__\xFD[aPw\x87\x82\x88\x01aK,V[\x95\x98\x94\x97P\x95PPPPV[__`@\x83\x85\x03\x12\x15aP\x94W__\xFD[\x825aP\x9F\x81aH\xC7V[\x91P` \x83\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aP\xB9W__\xFD[a?\xB7\x85\x82\x86\x01aK\xDEV[` \x81R_\x82Q\x80` \x84\x01R_[\x81\x81\x10\x15aP\xF1W` \x81\x86\x01\x81\x01Q`@\x86\x84\x01\x01R\x01aP\xD4V[P_`@\x82\x85\x01\x01R`@`\x1F\x19`\x1F\x83\x01\x16\x84\x01\x01\x91PP\x92\x91PPV[__`@\x83\x85\x03\x12\x15aQ!W__\xFD[\x825aQ,\x81aH\xC7V[\x91PaJb` \x84\x01aIGV[_` \x82\x84\x03\x12\x15aQJW__\xFD[\x815`\xFF\x81\x16\x81\x14a\x07\x94W__\xFD[_``\x82\x84\x03\x12\x15aQjW__\xFD[P\x91\x90PV[_` \x82\x84\x03\x12\x15aQ\x80W__\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15aQ\x95W__\xFD[a\x08\x0C\x84\x82\x85\x01aQZV[___`\x80\x84\x86\x03\x12\x15aQ\xB3W__\xFD[\x835`\x01`\x01`@\x1B\x03\x81\x11\x15aQ\xC8W__\xFD[aQ\xD4\x86\x82\x87\x01aK\xDEV[\x93PPaI\xE3\x85` \x86\x01aIZV[` \x81R_a\x07\x94` \x83\x01\x84aNmV[___``\x84\x86\x03\x12\x15aR\x08W__\xFD[\x835aR\x13\x81aH\xC7V[\x92P` \x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aR-W__\xFD[aR9\x86\x82\x87\x01aK\xDEV[\x92PPaRH`@\x85\x01aIGV[\x90P\x92P\x92P\x92V[__`@\x83\x85\x03\x12\x15aRbW__\xFD[\x825aRm\x81aH\xC7V[\x91P` \x83\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aR\x87W__\xFD[\x83\x01`\x1F\x81\x01\x85\x13aR\x97W__\xFD[\x805aR\xA5aK\xFB\x82aK\xBCV[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x85\x01\x01\x92P\x87\x83\x11\x15aR\xC6W__\xFD[` \x84\x01[\x83\x81\x10\x15aS\xEBW\x805`\x01`\x01`@\x1B\x03\x81\x11\x15aR\xE8W__\xFD[\x85\x01`\x80\x81\x8B\x03`\x1F\x19\x01\x12\x15aR\xFDW__\xFD[aS\x05aH\xEFV[aS\x12\x8B` \x84\x01aIZV[\x81R``\x82\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aS,W__\xFD[aS;\x8C` \x83\x86\x01\x01aK\xDEV[` \x83\x01RP`\x80\x82\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aSYW__\xFD[` \x81\x84\x01\x01\x92PP\x8A`\x1F\x83\x01\x12aSpW__\xFD[\x815aS~aK\xFB\x82aK\xBCV[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x86\x01\x01\x92P\x8D\x83\x11\x15aS\x9FW__\xFD[` \x85\x01\x94P[\x82\x85\x10\x15aS\xD5W\x845`\x01`\x01`@\x1B\x03\x81\x16\x81\x14aS\xC4W__\xFD[\x82R` \x94\x85\x01\x94\x90\x91\x01\x90aS\xA6V[`@\x84\x01RPP\x84RP` \x92\x83\x01\x92\x01aR\xCBV[P\x80\x94PPPPP\x92P\x92\x90PV[___`@\x84\x86\x03\x12\x15aT\x0CW__\xFD[\x835aT\x17\x81aH\xC7V[\x92P` \x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aT1W__\xFD[\x84\x01`\x1F\x81\x01\x86\x13aTAW__\xFD[\x805`\x01`\x01`@\x1B\x03\x81\x11\x15aTVW__\xFD[\x86` \x82\x84\x01\x01\x11\x15aTgW__\xFD[\x93\x96` \x91\x90\x91\x01\x95P\x92\x93PPPV[__`@\x83\x85\x03\x12\x15aT\x89W__\xFD[\x825aT\x94\x81aH\xC7V[\x91P` \x83\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aT\xAEW__\xFD[a?\xB7\x85\x82\x86\x01aQZV[__`@\x83\x85\x03\x12\x15aT\xCBW__\xFD[\x825aT\xD6\x81aH\xC7V[\x94` \x93\x90\x93\x015\x93PPPV[_` \x82\x84\x03\x12\x15aT\xF4W__\xFD[\x81Q\x80\x15\x15\x81\x14a\x07\x94W__\xFD[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[_\x825`>\x19\x836\x03\x01\x81\x12aU+W__\xFD[\x91\x90\x91\x01\x92\x91PPV[_` \x82\x84\x03\x12\x15aUEW__\xFD[a\x07\x94\x82aIGV[`@\x81\x01a\x08\x10\x82\x84aJ\x9DV[__\x835`\x1E\x19\x846\x03\x01\x81\x12aUqW__\xFD[\x83\x01\x805\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15aU\x8AW__\xFD[` \x01\x91P`\x05\x81\x90\x1B6\x03\x82\x13\x15a\x17?W__\xFD[``\x81\x01aU\xAF\x82\x85aJ\x9DV[`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16`@\x91\x90\x91\x01R\x91\x90PV[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[\x81\x81\x03\x81\x81\x11\x15a\x08\x10Wa\x08\x10aU\xC7V[`\x01`\x01`@\x1B\x03\x82\x81\x16\x82\x82\x16\x03\x90\x81\x11\x15a\x08\x10Wa\x08\x10aU\xC7V[_`\x0F\x82\x90\x0B`\x01`\x7F\x1B\x81\x01aV&WaV&aU\xC7V[_\x03\x92\x91PPV[`\x0F\x81\x81\x0B\x90\x83\x90\x0B\x01`\x01`\x01`\x7F\x1B\x03\x81\x13`\x01`\x01`\x7F\x1B\x03\x19\x82\x12\x17\x15a\x08\x10Wa\x08\x10aU\xC7V[`\x01`\x01`\xA0\x1B\x03\x86\x16\x81R`\xC0\x81\x01aVx` \x83\x01\x87aJ\x9DV[`\x01`\x01`\xA0\x1B\x03\x94\x90\x94\x16``\x82\x01R`\x01`\x01`@\x1B\x03\x92\x90\x92\x16`\x80\x83\x01Rc\xFF\xFF\xFF\xFF\x16`\xA0\x90\x91\x01R\x92\x91PPV[__\x835`\x1E\x19\x846\x03\x01\x81\x12aV\xC1W__\xFD[\x83\x01\x805\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15aV\xDAW__\xFD[` \x01\x91P6\x81\x90\x03\x82\x13\x15a\x17?W__\xFD[\x81\x83R\x81\x81` \x85\x017P_\x82\x82\x01` \x90\x81\x01\x91\x90\x91R`\x1F\x90\x91\x01`\x1F\x19\x16\x90\x91\x01\x01\x90V[`\x01`\x01`\xA0\x1B\x03\x88\x16\x81R_`\xC0\x82\x01aW4` \x84\x01\x8AaJ\x9DV[`\xC0``\x84\x01R\x86\x90R\x86`\xE0\x83\x01_[\x88\x81\x10\x15aWuW\x825aWX\x81aH\xC7V[`\x01`\x01`\xA0\x1B\x03\x16\x82R` \x92\x83\x01\x92\x90\x91\x01\x90`\x01\x01aWEV[P\x83\x81\x03`\x80\x85\x01RaW\x88\x81\x88aL\xC4V[\x91PP\x82\x81\x03`\xA0\x84\x01RaW\x9E\x81\x85\x87aV\xEEV[\x9A\x99PPPPPPPPPPV[_` \x82\x84\x03\x12\x15aW\xBCW__\xFD[\x815a\xFF\xFF\x81\x16\x81\x14a\x07\x94W__\xFD[c\xFF\xFF\xFF\xFF\x81\x81\x16\x83\x82\x16\x01\x90\x81\x11\x15a\x08\x10Wa\x08\x10aU\xC7V[\x81\x83R` \x83\x01\x92P_\x81_[\x84\x81\x10\x15aJ\xF6Wc\xFF\xFF\xFF\xFFaX\x0C\x83aIGV[\x16\x86R` \x95\x86\x01\x95\x91\x90\x91\x01\x90`\x01\x01aW\xF6V[`\x01`\x01`\xA0\x1B\x03\x85\x81\x16\x82R\x84\x16` \x82\x01R```@\x82\x01\x81\x90R_\x90aA\xD4\x90\x83\x01\x84\x86aW\xE9V[` \x81R_a\r!` \x83\x01\x84\x86aV\xEEV[`\x01`\x01`\xA0\x1B\x03\x87\x81\x16\x82R\x86\x16` \x82\x01R`\x80`@\x82\x01\x81\x90R_\x90aX\x8D\x90\x83\x01\x86\x88aW\xE9V[\x82\x81\x03``\x84\x01RaX\xA0\x81\x85\x87aV\xEEV[\x99\x98PPPPPPPPPV[_` \x82\x84\x03\x12\x15aX\xBDW__\xFD[\x81Qa\x07\x94\x81aH\xC7V[\x80Q` \x80\x83\x01Q\x91\x90\x81\x10\x15aQjW_\x19` \x91\x90\x91\x03`\x03\x1B\x1B\x16\x91\x90PV[`@\x81R_aX\xFD`@\x83\x01\x85aN\xCCV[\x82\x81\x03` \x84\x01Ra\x1C\xDB\x81\x85aN\xCCV[_` \x82\x84\x03\x12\x15aY\x1FW__\xFD[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15aY4W__\xFD[\x82\x01`\x1F\x81\x01\x84\x13aYDW__\xFD[\x80QaYRaK\xFB\x82aK\xBCV[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x85\x01\x01\x92P\x86\x83\x11\x15aYsW__\xFD[` \x84\x01[\x83\x81\x10\x15aZ\x10W\x80Q`\x01`\x01`@\x1B\x03\x81\x11\x15aY\x95W__\xFD[\x85\x01`?\x81\x01\x89\x13aY\xA5W__\xFD[` \x81\x01QaY\xB6aK\xFB\x82aK\xBCV[\x80\x82\x82R` \x82\x01\x91P` \x80\x84`\x05\x1B\x86\x01\x01\x01\x92P\x8B\x83\x11\x15aY\xD9W__\xFD[`@\x84\x01\x93P[\x82\x84\x10\x15aY\xFBW\x83Q\x82R` \x93\x84\x01\x93\x90\x91\x01\x90aY\xE0V[\x86RPP` \x93\x84\x01\x93\x91\x90\x91\x01\x90PaYxV[P\x96\x95PPPPPPV[`\x01`\x01`\xA0\x1B\x03\x93\x84\x16\x81R\x91\x90\x92\x16` \x82\x01R`\x01`\x01`@\x1B\x03\x90\x91\x16`@\x82\x01R``\x01\x90V[_`\x01\x82\x01aZXWaZXaU\xC7V[P`\x01\x01\x90V[_\x81aZmWaZmaU\xC7V[P_\x19\x01\x90V[`\x0F\x82\x81\x0B\x90\x82\x90\x0B\x03`\x01`\x01`\x7F\x1B\x03\x19\x81\x12`\x01`\x01`\x7F\x1B\x03\x82\x13\x17\x15a\x08\x10Wa\x08\x10aU\xC7V[cNH{q`\xE0\x1B_R`!`\x04R`$_\xFD[cNH{q`\xE0\x1B_R`\x12`\x04R`$_\xFD[\x80\x82\x01\x80\x82\x11\x15a\x08\x10Wa\x08\x10aU\xC7V[\x80\x82\x01\x82\x81\x12_\x83\x12\x80\x15\x82\x16\x82\x15\x82\x16\x17\x15aZ\xFBWaZ\xFBaU\xC7V[PP\x92\x91PPV[cNH{q`\xE0\x1B_R`1`\x04R`$_\xFD[_\x82a[1WcNH{q`\xE0\x1B_R`\x12`\x04R`$_\xFD[P\x04\x90V\xFE\x14\x87\xAFT\x18\xC4~\xE5\xEAE\xEFJ\x939\x86h\x12\x08\x90wJ\x9E\x13H~a\xE9\xDC;\xAFv\xDD\xA2dipfsX\"\x12 d\xA2\xCBnA\xF7\xF8\xBD))|\x0E\xEF`\x0B\xAE\x9A\xFE\xB2\xECA\xD7\xD1\xF5TJ\xCA\x9F\xE1}F\x99dsolcC\0\x08\x1B\x003",
    );
    /**```solidity
struct OperatorSet { address avs; uint32 id; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct OperatorSet {
        #[allow(missing_docs)]
        pub avs: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub id: u32,
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
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address, u32);
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<OperatorSet> for UnderlyingRustTuple<'_> {
            fn from(value: OperatorSet) -> Self {
                (value.avs, value.id)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for OperatorSet {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { avs: tuple.0, id: tuple.1 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for OperatorSet {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for OperatorSet {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.avs,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.id),
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
        impl alloy_sol_types::SolType for OperatorSet {
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
        impl alloy_sol_types::SolStruct for OperatorSet {
            const NAME: &'static str = "OperatorSet";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "OperatorSet(address avs,uint32 id)",
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
                            &self.avs,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.id)
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for OperatorSet {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.avs,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(&rust.id)
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
                    &rust.avs,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(&rust.id, out);
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
    /**Custom error with signature `AlreadyMemberOfSet()` and selector `0xd8d8dc4e`.
```solidity
error AlreadyMemberOfSet();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct AlreadyMemberOfSet {}
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
        impl ::core::convert::From<AlreadyMemberOfSet> for UnderlyingRustTuple<'_> {
            fn from(value: AlreadyMemberOfSet) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for AlreadyMemberOfSet {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for AlreadyMemberOfSet {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "AlreadyMemberOfSet()";
            const SELECTOR: [u8; 4] = [216u8, 216u8, 220u8, 78u8];
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
    /**Custom error with signature `Empty()` and selector `0x3db2a12a`.
```solidity
error Empty();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct Empty {}
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
        impl ::core::convert::From<Empty> for UnderlyingRustTuple<'_> {
            fn from(value: Empty) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for Empty {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for Empty {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "Empty()";
            const SELECTOR: [u8; 4] = [61u8, 178u8, 161u8, 42u8];
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
    /**Custom error with signature `InsufficientMagnitude()` and selector `0x6c9be0bf`.
```solidity
error InsufficientMagnitude();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InsufficientMagnitude {}
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
        impl ::core::convert::From<InsufficientMagnitude> for UnderlyingRustTuple<'_> {
            fn from(value: InsufficientMagnitude) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InsufficientMagnitude {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InsufficientMagnitude {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InsufficientMagnitude()";
            const SELECTOR: [u8; 4] = [108u8, 155u8, 224u8, 191u8];
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
    /**Custom error with signature `InvalidAVSRegistrar()` and selector `0xe8589e08`.
```solidity
error InvalidAVSRegistrar();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidAVSRegistrar {}
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
        impl ::core::convert::From<InvalidAVSRegistrar> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidAVSRegistrar) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidAVSRegistrar {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidAVSRegistrar {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidAVSRegistrar()";
            const SELECTOR: [u8; 4] = [232u8, 88u8, 158u8, 8u8];
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
    /**Custom error with signature `InvalidCaller()` and selector `0x48f5c3ed`.
```solidity
error InvalidCaller();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidCaller {}
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
        impl ::core::convert::From<InvalidCaller> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidCaller) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidCaller {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidCaller {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidCaller()";
            const SELECTOR: [u8; 4] = [72u8, 245u8, 195u8, 237u8];
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
    /**Custom error with signature `InvalidOperator()` and selector `0xccea9e6f`.
```solidity
error InvalidOperator();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidOperator {}
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
        impl ::core::convert::From<InvalidOperator> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidOperator) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidOperator {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidOperator {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidOperator()";
            const SELECTOR: [u8; 4] = [204u8, 234u8, 158u8, 111u8];
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
    /**Custom error with signature `InvalidOperatorSet()` and selector `0x7ec5c154`.
```solidity
error InvalidOperatorSet();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidOperatorSet {}
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
        impl ::core::convert::From<InvalidOperatorSet> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidOperatorSet) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidOperatorSet {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidOperatorSet {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidOperatorSet()";
            const SELECTOR: [u8; 4] = [126u8, 197u8, 193u8, 84u8];
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
    /**Custom error with signature `InvalidWadToSlash()` and selector `0x13536031`.
```solidity
error InvalidWadToSlash();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidWadToSlash {}
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
        impl ::core::convert::From<InvalidWadToSlash> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidWadToSlash) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidWadToSlash {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidWadToSlash {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidWadToSlash()";
            const SELECTOR: [u8; 4] = [19u8, 83u8, 96u8, 49u8];
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
    /**Custom error with signature `ModificationAlreadyPending()` and selector `0xd8fcbe30`.
```solidity
error ModificationAlreadyPending();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ModificationAlreadyPending {}
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
        impl ::core::convert::From<ModificationAlreadyPending>
        for UnderlyingRustTuple<'_> {
            fn from(value: ModificationAlreadyPending) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for ModificationAlreadyPending {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for ModificationAlreadyPending {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ModificationAlreadyPending()";
            const SELECTOR: [u8; 4] = [216u8, 252u8, 190u8, 48u8];
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
    /**Custom error with signature `NonexistentAVSMetadata()` and selector `0x48f7dbb9`.
```solidity
error NonexistentAVSMetadata();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct NonexistentAVSMetadata {}
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
        impl ::core::convert::From<NonexistentAVSMetadata> for UnderlyingRustTuple<'_> {
            fn from(value: NonexistentAVSMetadata) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for NonexistentAVSMetadata {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for NonexistentAVSMetadata {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "NonexistentAVSMetadata()";
            const SELECTOR: [u8; 4] = [72u8, 247u8, 219u8, 185u8];
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
    /**Custom error with signature `NotMemberOfSet()` and selector `0x25131d4f`.
```solidity
error NotMemberOfSet();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct NotMemberOfSet {}
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
        impl ::core::convert::From<NotMemberOfSet> for UnderlyingRustTuple<'_> {
            fn from(value: NotMemberOfSet) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for NotMemberOfSet {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for NotMemberOfSet {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "NotMemberOfSet()";
            const SELECTOR: [u8; 4] = [37u8, 19u8, 29u8, 79u8];
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
    /**Custom error with signature `OperatorNotSlashable()` and selector `0xebbff497`.
```solidity
error OperatorNotSlashable();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct OperatorNotSlashable {}
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
        impl ::core::convert::From<OperatorNotSlashable> for UnderlyingRustTuple<'_> {
            fn from(value: OperatorNotSlashable) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for OperatorNotSlashable {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for OperatorNotSlashable {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "OperatorNotSlashable()";
            const SELECTOR: [u8; 4] = [235u8, 191u8, 244u8, 151u8];
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
    /**Custom error with signature `OutOfBounds()` and selector `0xb4120f14`.
```solidity
error OutOfBounds();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct OutOfBounds {}
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
        impl ::core::convert::From<OutOfBounds> for UnderlyingRustTuple<'_> {
            fn from(value: OutOfBounds) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for OutOfBounds {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for OutOfBounds {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "OutOfBounds()";
            const SELECTOR: [u8; 4] = [180u8, 18u8, 15u8, 20u8];
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
    /**Custom error with signature `SameMagnitude()` and selector `0x8c0c2f26`.
```solidity
error SameMagnitude();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct SameMagnitude {}
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
        impl ::core::convert::From<SameMagnitude> for UnderlyingRustTuple<'_> {
            fn from(value: SameMagnitude) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for SameMagnitude {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for SameMagnitude {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "SameMagnitude()";
            const SELECTOR: [u8; 4] = [140u8, 12u8, 47u8, 38u8];
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
    /**Custom error with signature `StrategiesMustBeInAscendingOrder()` and selector `0x9f1c8053`.
```solidity
error StrategiesMustBeInAscendingOrder();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct StrategiesMustBeInAscendingOrder {}
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
        impl ::core::convert::From<StrategiesMustBeInAscendingOrder>
        for UnderlyingRustTuple<'_> {
            fn from(value: StrategiesMustBeInAscendingOrder) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for StrategiesMustBeInAscendingOrder {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for StrategiesMustBeInAscendingOrder {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "StrategiesMustBeInAscendingOrder()";
            const SELECTOR: [u8; 4] = [159u8, 28u8, 128u8, 83u8];
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
    /**Custom error with signature `StrategyAlreadyInOperatorSet()` and selector `0x585cfb2f`.
```solidity
error StrategyAlreadyInOperatorSet();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct StrategyAlreadyInOperatorSet {}
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
        impl ::core::convert::From<StrategyAlreadyInOperatorSet>
        for UnderlyingRustTuple<'_> {
            fn from(value: StrategyAlreadyInOperatorSet) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for StrategyAlreadyInOperatorSet {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for StrategyAlreadyInOperatorSet {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "StrategyAlreadyInOperatorSet()";
            const SELECTOR: [u8; 4] = [88u8, 92u8, 251u8, 47u8];
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
    /**Custom error with signature `StrategyNotInOperatorSet()` and selector `0x6378684e`.
```solidity
error StrategyNotInOperatorSet();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct StrategyNotInOperatorSet {}
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
        impl ::core::convert::From<StrategyNotInOperatorSet>
        for UnderlyingRustTuple<'_> {
            fn from(value: StrategyNotInOperatorSet) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for StrategyNotInOperatorSet {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for StrategyNotInOperatorSet {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "StrategyNotInOperatorSet()";
            const SELECTOR: [u8; 4] = [99u8, 120u8, 104u8, 78u8];
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
    /**Custom error with signature `UninitializedAllocationDelay()` and selector `0xfa55fc81`.
```solidity
error UninitializedAllocationDelay();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct UninitializedAllocationDelay {}
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
        impl ::core::convert::From<UninitializedAllocationDelay>
        for UnderlyingRustTuple<'_> {
            fn from(value: UninitializedAllocationDelay) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for UninitializedAllocationDelay {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for UninitializedAllocationDelay {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "UninitializedAllocationDelay()";
            const SELECTOR: [u8; 4] = [250u8, 85u8, 252u8, 129u8];
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
    /**Event with signature `AVSMetadataURIUpdated(address,string)` and selector `0xa89c1dc243d8908a96dd84944bcc97d6bc6ac00dd78e20621576be6a3c943713`.
```solidity
event AVSMetadataURIUpdated(address indexed avs, string metadataURI);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct AVSMetadataURIUpdated {
        #[allow(missing_docs)]
        pub avs: alloy::sol_types::private::Address,
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
        impl alloy_sol_types::SolEvent for AVSMetadataURIUpdated {
            type DataTuple<'a> = (alloy::sol_types::sol_data::String,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "AVSMetadataURIUpdated(address,string)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                168u8,
                156u8,
                29u8,
                194u8,
                67u8,
                216u8,
                144u8,
                138u8,
                150u8,
                221u8,
                132u8,
                148u8,
                75u8,
                204u8,
                151u8,
                214u8,
                188u8,
                106u8,
                192u8,
                13u8,
                215u8,
                142u8,
                32u8,
                98u8,
                21u8,
                118u8,
                190u8,
                106u8,
                60u8,
                148u8,
                55u8,
                19u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    avs: topics.1,
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
                (Self::SIGNATURE_HASH.into(), self.avs.clone())
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
                    &self.avs,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for AVSMetadataURIUpdated {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&AVSMetadataURIUpdated> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &AVSMetadataURIUpdated) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `AVSRegistrarSet(address,address)` and selector `0x2ae945c40c44dc0ec263f95609c3fdc6952e0aefa22d6374e44f2c997acedf85`.
```solidity
event AVSRegistrarSet(address avs, address registrar);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct AVSRegistrarSet {
        #[allow(missing_docs)]
        pub avs: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub registrar: alloy::sol_types::private::Address,
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
        impl alloy_sol_types::SolEvent for AVSRegistrarSet {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "AVSRegistrarSet(address,address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                42u8,
                233u8,
                69u8,
                196u8,
                12u8,
                68u8,
                220u8,
                14u8,
                194u8,
                99u8,
                249u8,
                86u8,
                9u8,
                195u8,
                253u8,
                198u8,
                149u8,
                46u8,
                10u8,
                239u8,
                162u8,
                45u8,
                99u8,
                116u8,
                228u8,
                79u8,
                44u8,
                153u8,
                122u8,
                206u8,
                223u8,
                133u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    avs: data.0,
                    registrar: data.1,
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
                        &self.avs,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.registrar,
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
        impl alloy_sol_types::private::IntoLogData for AVSRegistrarSet {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&AVSRegistrarSet> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &AVSRegistrarSet) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `AllocationDelaySet(address,uint32,uint32)` and selector `0x4e85751d6331506c6c62335f207eb31f12a61e570f34f5c17640308785c6d4db`.
```solidity
event AllocationDelaySet(address operator, uint32 delay, uint32 effectBlock);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct AllocationDelaySet {
        #[allow(missing_docs)]
        pub operator: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub delay: u32,
        #[allow(missing_docs)]
        pub effectBlock: u32,
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
        impl alloy_sol_types::SolEvent for AllocationDelaySet {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<32>,
                alloy::sol_types::sol_data::Uint<32>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "AllocationDelaySet(address,uint32,uint32)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                78u8,
                133u8,
                117u8,
                29u8,
                99u8,
                49u8,
                80u8,
                108u8,
                108u8,
                98u8,
                51u8,
                95u8,
                32u8,
                126u8,
                179u8,
                31u8,
                18u8,
                166u8,
                30u8,
                87u8,
                15u8,
                52u8,
                245u8,
                193u8,
                118u8,
                64u8,
                48u8,
                135u8,
                133u8,
                198u8,
                212u8,
                219u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    operator: data.0,
                    delay: data.1,
                    effectBlock: data.2,
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
                        &self.operator,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.delay),
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.effectBlock),
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
        impl alloy_sol_types::private::IntoLogData for AllocationDelaySet {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&AllocationDelaySet> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &AllocationDelaySet) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `AllocationUpdated(address,(address,uint32),address,uint64,uint32)` and selector `0x1487af5418c47ee5ea45ef4a93398668120890774a9e13487e61e9dc3baf76dd`.
```solidity
event AllocationUpdated(address operator, OperatorSet operatorSet, address strategy, uint64 magnitude, uint32 effectBlock);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct AllocationUpdated {
        #[allow(missing_docs)]
        pub operator: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub operatorSet: <OperatorSet as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub strategy: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub magnitude: u64,
        #[allow(missing_docs)]
        pub effectBlock: u32,
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
        impl alloy_sol_types::SolEvent for AllocationUpdated {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                OperatorSet,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<64>,
                alloy::sol_types::sol_data::Uint<32>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "AllocationUpdated(address,(address,uint32),address,uint64,uint32)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                20u8,
                135u8,
                175u8,
                84u8,
                24u8,
                196u8,
                126u8,
                229u8,
                234u8,
                69u8,
                239u8,
                74u8,
                147u8,
                57u8,
                134u8,
                104u8,
                18u8,
                8u8,
                144u8,
                119u8,
                74u8,
                158u8,
                19u8,
                72u8,
                126u8,
                97u8,
                233u8,
                220u8,
                59u8,
                175u8,
                118u8,
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
                    operator: data.0,
                    operatorSet: data.1,
                    strategy: data.2,
                    magnitude: data.3,
                    effectBlock: data.4,
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
                        &self.operator,
                    ),
                    <OperatorSet as alloy_sol_types::SolType>::tokenize(
                        &self.operatorSet,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.strategy,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::tokenize(&self.magnitude),
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.effectBlock),
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
        impl alloy_sol_types::private::IntoLogData for AllocationUpdated {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&AllocationUpdated> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &AllocationUpdated) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `EncumberedMagnitudeUpdated(address,address,uint64)` and selector `0xacf9095feb3a370c9cf692421c69ef320d4db5c66e6a7d29c7694eb02364fc55`.
```solidity
event EncumberedMagnitudeUpdated(address operator, address strategy, uint64 encumberedMagnitude);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct EncumberedMagnitudeUpdated {
        #[allow(missing_docs)]
        pub operator: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub strategy: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub encumberedMagnitude: u64,
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
        impl alloy_sol_types::SolEvent for EncumberedMagnitudeUpdated {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<64>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "EncumberedMagnitudeUpdated(address,address,uint64)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                172u8,
                249u8,
                9u8,
                95u8,
                235u8,
                58u8,
                55u8,
                12u8,
                156u8,
                246u8,
                146u8,
                66u8,
                28u8,
                105u8,
                239u8,
                50u8,
                13u8,
                77u8,
                181u8,
                198u8,
                110u8,
                106u8,
                125u8,
                41u8,
                199u8,
                105u8,
                78u8,
                176u8,
                35u8,
                100u8,
                252u8,
                85u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    operator: data.0,
                    strategy: data.1,
                    encumberedMagnitude: data.2,
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
                        &self.operator,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.strategy,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::tokenize(&self.encumberedMagnitude),
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
        impl alloy_sol_types::private::IntoLogData for EncumberedMagnitudeUpdated {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&EncumberedMagnitudeUpdated> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(
                this: &EncumberedMagnitudeUpdated,
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
    /**Event with signature `MaxMagnitudeUpdated(address,address,uint64)` and selector `0x1c6458079a41077d003c11faf9bf097e693bd67979e4e6500bac7b29db779b5c`.
```solidity
event MaxMagnitudeUpdated(address operator, address strategy, uint64 maxMagnitude);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct MaxMagnitudeUpdated {
        #[allow(missing_docs)]
        pub operator: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub strategy: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub maxMagnitude: u64,
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
        impl alloy_sol_types::SolEvent for MaxMagnitudeUpdated {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<64>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "MaxMagnitudeUpdated(address,address,uint64)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                28u8,
                100u8,
                88u8,
                7u8,
                154u8,
                65u8,
                7u8,
                125u8,
                0u8,
                60u8,
                17u8,
                250u8,
                249u8,
                191u8,
                9u8,
                126u8,
                105u8,
                59u8,
                214u8,
                121u8,
                121u8,
                228u8,
                230u8,
                80u8,
                11u8,
                172u8,
                123u8,
                41u8,
                219u8,
                119u8,
                155u8,
                92u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    operator: data.0,
                    strategy: data.1,
                    maxMagnitude: data.2,
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
                        &self.operator,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.strategy,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::tokenize(&self.maxMagnitude),
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
        impl alloy_sol_types::private::IntoLogData for MaxMagnitudeUpdated {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&MaxMagnitudeUpdated> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &MaxMagnitudeUpdated) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `OperatorAddedToOperatorSet(address,(address,uint32))` and selector `0x43232edf9071753d2321e5fa7e018363ee248e5f2142e6c08edd3265bfb4895e`.
```solidity
event OperatorAddedToOperatorSet(address indexed operator, OperatorSet operatorSet);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct OperatorAddedToOperatorSet {
        #[allow(missing_docs)]
        pub operator: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub operatorSet: <OperatorSet as alloy::sol_types::SolType>::RustType,
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
        impl alloy_sol_types::SolEvent for OperatorAddedToOperatorSet {
            type DataTuple<'a> = (OperatorSet,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "OperatorAddedToOperatorSet(address,(address,uint32))";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                67u8,
                35u8,
                46u8,
                223u8,
                144u8,
                113u8,
                117u8,
                61u8,
                35u8,
                33u8,
                229u8,
                250u8,
                126u8,
                1u8,
                131u8,
                99u8,
                238u8,
                36u8,
                142u8,
                95u8,
                33u8,
                66u8,
                230u8,
                192u8,
                142u8,
                221u8,
                50u8,
                101u8,
                191u8,
                180u8,
                137u8,
                94u8,
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
                    operatorSet: data.0,
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
                (<OperatorSet as alloy_sol_types::SolType>::tokenize(&self.operatorSet),)
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
        impl alloy_sol_types::private::IntoLogData for OperatorAddedToOperatorSet {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&OperatorAddedToOperatorSet> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(
                this: &OperatorAddedToOperatorSet,
            ) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `OperatorRemovedFromOperatorSet(address,(address,uint32))` and selector `0xad34c3070be1dffbcaa499d000ba2b8d9848aefcac3059df245dd95c4ece14fe`.
```solidity
event OperatorRemovedFromOperatorSet(address indexed operator, OperatorSet operatorSet);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct OperatorRemovedFromOperatorSet {
        #[allow(missing_docs)]
        pub operator: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub operatorSet: <OperatorSet as alloy::sol_types::SolType>::RustType,
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
        impl alloy_sol_types::SolEvent for OperatorRemovedFromOperatorSet {
            type DataTuple<'a> = (OperatorSet,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "OperatorRemovedFromOperatorSet(address,(address,uint32))";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                173u8,
                52u8,
                195u8,
                7u8,
                11u8,
                225u8,
                223u8,
                251u8,
                202u8,
                164u8,
                153u8,
                208u8,
                0u8,
                186u8,
                43u8,
                141u8,
                152u8,
                72u8,
                174u8,
                252u8,
                172u8,
                48u8,
                89u8,
                223u8,
                36u8,
                93u8,
                217u8,
                92u8,
                78u8,
                206u8,
                20u8,
                254u8,
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
                    operatorSet: data.0,
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
                (<OperatorSet as alloy_sol_types::SolType>::tokenize(&self.operatorSet),)
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
        impl alloy_sol_types::private::IntoLogData for OperatorRemovedFromOperatorSet {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&OperatorRemovedFromOperatorSet>
        for alloy_sol_types::private::LogData {
            #[inline]
            fn from(
                this: &OperatorRemovedFromOperatorSet,
            ) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `OperatorSetCreated((address,uint32))` and selector `0x31629285ead2335ae0933f86ed2ae63321f7af77b4e6eaabc42c057880977e6c`.
```solidity
event OperatorSetCreated(OperatorSet operatorSet);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct OperatorSetCreated {
        #[allow(missing_docs)]
        pub operatorSet: <OperatorSet as alloy::sol_types::SolType>::RustType,
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
        impl alloy_sol_types::SolEvent for OperatorSetCreated {
            type DataTuple<'a> = (OperatorSet,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "OperatorSetCreated((address,uint32))";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                49u8,
                98u8,
                146u8,
                133u8,
                234u8,
                210u8,
                51u8,
                90u8,
                224u8,
                147u8,
                63u8,
                134u8,
                237u8,
                42u8,
                230u8,
                51u8,
                33u8,
                247u8,
                175u8,
                119u8,
                180u8,
                230u8,
                234u8,
                171u8,
                196u8,
                44u8,
                5u8,
                120u8,
                128u8,
                151u8,
                126u8,
                108u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { operatorSet: data.0 }
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
                (<OperatorSet as alloy_sol_types::SolType>::tokenize(&self.operatorSet),)
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
        impl alloy_sol_types::private::IntoLogData for OperatorSetCreated {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&OperatorSetCreated> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &OperatorSetCreated) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `OperatorSlashed(address,(address,uint32),address[],uint256[],string)` and selector `0x80969ad29428d6797ee7aad084f9e4a42a82fc506dcd2ca3b6fb431f85ccebe5`.
```solidity
event OperatorSlashed(address operator, OperatorSet operatorSet, address[] strategies, uint256[] wadSlashed, string description);
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
        pub operator: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub operatorSet: <OperatorSet as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub strategies: alloy::sol_types::private::Vec<
            alloy::sol_types::private::Address,
        >,
        #[allow(missing_docs)]
        pub wadSlashed: alloy::sol_types::private::Vec<
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
                alloy::sol_types::sol_data::Address,
                OperatorSet,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,
                alloy::sol_types::sol_data::String,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "OperatorSlashed(address,(address,uint32),address[],uint256[],string)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                128u8,
                150u8,
                154u8,
                210u8,
                148u8,
                40u8,
                214u8,
                121u8,
                126u8,
                231u8,
                170u8,
                208u8,
                132u8,
                249u8,
                228u8,
                164u8,
                42u8,
                130u8,
                252u8,
                80u8,
                109u8,
                205u8,
                44u8,
                163u8,
                182u8,
                251u8,
                67u8,
                31u8,
                133u8,
                204u8,
                235u8,
                229u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    operator: data.0,
                    operatorSet: data.1,
                    strategies: data.2,
                    wadSlashed: data.3,
                    description: data.4,
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
                        &self.operator,
                    ),
                    <OperatorSet as alloy_sol_types::SolType>::tokenize(
                        &self.operatorSet,
                    ),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Address,
                    > as alloy_sol_types::SolType>::tokenize(&self.strategies),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<256>,
                    > as alloy_sol_types::SolType>::tokenize(&self.wadSlashed),
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
    /**Event with signature `StrategyAddedToOperatorSet((address,uint32),address)` and selector `0x7ab260fe0af193db5f4986770d831bda4ea46099dc817e8b6716dcae8af8e88b`.
```solidity
event StrategyAddedToOperatorSet(OperatorSet operatorSet, address strategy);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct StrategyAddedToOperatorSet {
        #[allow(missing_docs)]
        pub operatorSet: <OperatorSet as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub strategy: alloy::sol_types::private::Address,
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
        impl alloy_sol_types::SolEvent for StrategyAddedToOperatorSet {
            type DataTuple<'a> = (OperatorSet, alloy::sol_types::sol_data::Address);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "StrategyAddedToOperatorSet((address,uint32),address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                122u8,
                178u8,
                96u8,
                254u8,
                10u8,
                241u8,
                147u8,
                219u8,
                95u8,
                73u8,
                134u8,
                119u8,
                13u8,
                131u8,
                27u8,
                218u8,
                78u8,
                164u8,
                96u8,
                153u8,
                220u8,
                129u8,
                126u8,
                139u8,
                103u8,
                22u8,
                220u8,
                174u8,
                138u8,
                248u8,
                232u8,
                139u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    operatorSet: data.0,
                    strategy: data.1,
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
                    <OperatorSet as alloy_sol_types::SolType>::tokenize(
                        &self.operatorSet,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.strategy,
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
        impl alloy_sol_types::private::IntoLogData for StrategyAddedToOperatorSet {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&StrategyAddedToOperatorSet> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(
                this: &StrategyAddedToOperatorSet,
            ) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `StrategyRemovedFromOperatorSet((address,uint32),address)` and selector `0x7b4b073d80dcac55a11177d8459ad9f664ceeb91f71f27167bb14f8152a7eeee`.
```solidity
event StrategyRemovedFromOperatorSet(OperatorSet operatorSet, address strategy);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct StrategyRemovedFromOperatorSet {
        #[allow(missing_docs)]
        pub operatorSet: <OperatorSet as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub strategy: alloy::sol_types::private::Address,
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
        impl alloy_sol_types::SolEvent for StrategyRemovedFromOperatorSet {
            type DataTuple<'a> = (OperatorSet, alloy::sol_types::sol_data::Address);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "StrategyRemovedFromOperatorSet((address,uint32),address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                123u8,
                75u8,
                7u8,
                61u8,
                128u8,
                220u8,
                172u8,
                85u8,
                161u8,
                17u8,
                119u8,
                216u8,
                69u8,
                154u8,
                217u8,
                246u8,
                100u8,
                206u8,
                235u8,
                145u8,
                247u8,
                31u8,
                39u8,
                22u8,
                123u8,
                177u8,
                79u8,
                129u8,
                82u8,
                167u8,
                238u8,
                238u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    operatorSet: data.0,
                    strategy: data.1,
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
                    <OperatorSet as alloy_sol_types::SolType>::tokenize(
                        &self.operatorSet,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.strategy,
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
        impl alloy_sol_types::private::IntoLogData for StrategyRemovedFromOperatorSet {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&StrategyRemovedFromOperatorSet>
        for alloy_sol_types::private::LogData {
            #[inline]
            fn from(
                this: &StrategyRemovedFromOperatorSet,
            ) -> alloy_sol_types::private::LogData {
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
constructor(address _delegation, address _pauserRegistry, address _permissionController, uint32 _DEALLOCATION_DELAY, uint32 _ALLOCATION_CONFIGURATION_DELAY, string _version);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct constructorCall {
        #[allow(missing_docs)]
        pub _delegation: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub _pauserRegistry: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub _permissionController: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub _DEALLOCATION_DELAY: u32,
        #[allow(missing_docs)]
        pub _ALLOCATION_CONFIGURATION_DELAY: u32,
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
                alloy::sol_types::sol_data::Uint<32>,
                alloy::sol_types::sol_data::Uint<32>,
                alloy::sol_types::sol_data::String,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Address,
                u32,
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
                        value._delegation,
                        value._pauserRegistry,
                        value._permissionController,
                        value._DEALLOCATION_DELAY,
                        value._ALLOCATION_CONFIGURATION_DELAY,
                        value._version,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for constructorCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _delegation: tuple.0,
                        _pauserRegistry: tuple.1,
                        _permissionController: tuple.2,
                        _DEALLOCATION_DELAY: tuple.3,
                        _ALLOCATION_CONFIGURATION_DELAY: tuple.4,
                        _version: tuple.5,
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
                alloy::sol_types::sol_data::Uint<32>,
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
                        &self._delegation,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._pauserRegistry,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._permissionController,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self._DEALLOCATION_DELAY),
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(
                        &self._ALLOCATION_CONFIGURATION_DELAY,
                    ),
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        &self._version,
                    ),
                )
            }
        }
    };
    /**Function with signature `ALLOCATION_CONFIGURATION_DELAY()` and selector `0x7bc1ef61`.
```solidity
function ALLOCATION_CONFIGURATION_DELAY() external view returns (uint32);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ALLOCATION_CONFIGURATION_DELAYCall {}
    ///Container type for the return parameters of the [`ALLOCATION_CONFIGURATION_DELAY()`](ALLOCATION_CONFIGURATION_DELAYCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ALLOCATION_CONFIGURATION_DELAYReturn {
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
            impl ::core::convert::From<ALLOCATION_CONFIGURATION_DELAYCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: ALLOCATION_CONFIGURATION_DELAYCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for ALLOCATION_CONFIGURATION_DELAYCall {
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
            impl ::core::convert::From<ALLOCATION_CONFIGURATION_DELAYReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: ALLOCATION_CONFIGURATION_DELAYReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for ALLOCATION_CONFIGURATION_DELAYReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for ALLOCATION_CONFIGURATION_DELAYCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = ALLOCATION_CONFIGURATION_DELAYReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ALLOCATION_CONFIGURATION_DELAY()";
            const SELECTOR: [u8; 4] = [123u8, 193u8, 239u8, 97u8];
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
    /**Function with signature `DEALLOCATION_DELAY()` and selector `0x2981eb77`.
```solidity
function DEALLOCATION_DELAY() external view returns (uint32);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct DEALLOCATION_DELAYCall {}
    ///Container type for the return parameters of the [`DEALLOCATION_DELAY()`](DEALLOCATION_DELAYCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct DEALLOCATION_DELAYReturn {
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
            impl ::core::convert::From<DEALLOCATION_DELAYCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: DEALLOCATION_DELAYCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for DEALLOCATION_DELAYCall {
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
            impl ::core::convert::From<DEALLOCATION_DELAYReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: DEALLOCATION_DELAYReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for DEALLOCATION_DELAYReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for DEALLOCATION_DELAYCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = DEALLOCATION_DELAYReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "DEALLOCATION_DELAY()";
            const SELECTOR: [u8; 4] = [41u8, 129u8, 235u8, 119u8];
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
    /**Function with signature `addStrategiesToOperatorSet(address,uint32,address[])` and selector `0x50feea20`.
```solidity
function addStrategiesToOperatorSet(address avs, uint32 operatorSetId, address[] memory strategies) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct addStrategiesToOperatorSetCall {
        #[allow(missing_docs)]
        pub avs: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub operatorSetId: u32,
        #[allow(missing_docs)]
        pub strategies: alloy::sol_types::private::Vec<
            alloy::sol_types::private::Address,
        >,
    }
    ///Container type for the return parameters of the [`addStrategiesToOperatorSet(address,uint32,address[])`](addStrategiesToOperatorSetCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct addStrategiesToOperatorSetReturn {}
    #[allow(
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
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                u32,
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
            impl ::core::convert::From<addStrategiesToOperatorSetCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: addStrategiesToOperatorSetCall) -> Self {
                    (value.avs, value.operatorSetId, value.strategies)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for addStrategiesToOperatorSetCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        avs: tuple.0,
                        operatorSetId: tuple.1,
                        strategies: tuple.2,
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
            impl ::core::convert::From<addStrategiesToOperatorSetReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: addStrategiesToOperatorSetReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for addStrategiesToOperatorSetReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for addStrategiesToOperatorSetCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<32>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = addStrategiesToOperatorSetReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "addStrategiesToOperatorSet(address,uint32,address[])";
            const SELECTOR: [u8; 4] = [80u8, 254u8, 234u8, 32u8];
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
                        &self.avs,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.operatorSetId),
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
    /**Function with signature `clearDeallocationQueue(address,address[],uint16[])` and selector `0x4b5046ef`.
```solidity
function clearDeallocationQueue(address operator, address[] memory strategies, uint16[] memory numToClear) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct clearDeallocationQueueCall {
        #[allow(missing_docs)]
        pub operator: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub strategies: alloy::sol_types::private::Vec<
            alloy::sol_types::private::Address,
        >,
        #[allow(missing_docs)]
        pub numToClear: alloy::sol_types::private::Vec<u16>,
    }
    ///Container type for the return parameters of the [`clearDeallocationQueue(address,address[],uint16[])`](clearDeallocationQueueCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct clearDeallocationQueueReturn {}
    #[allow(
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
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<16>>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
                alloy::sol_types::private::Vec<u16>,
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
            impl ::core::convert::From<clearDeallocationQueueCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: clearDeallocationQueueCall) -> Self {
                    (value.operator, value.strategies, value.numToClear)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for clearDeallocationQueueCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        operator: tuple.0,
                        strategies: tuple.1,
                        numToClear: tuple.2,
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
            impl ::core::convert::From<clearDeallocationQueueReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: clearDeallocationQueueReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for clearDeallocationQueueReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for clearDeallocationQueueCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<16>>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = clearDeallocationQueueReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "clearDeallocationQueue(address,address[],uint16[])";
            const SELECTOR: [u8; 4] = [75u8, 80u8, 70u8, 239u8];
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
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<16>,
                    > as alloy_sol_types::SolType>::tokenize(&self.numToClear),
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
    /**Function with signature `createOperatorSets(address,(uint32,address[])[])` and selector `0x261f84e0`.
```solidity
function createOperatorSets(address avs, IAllocationManagerTypes.CreateSetParams[] memory params) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct createOperatorSetsCall {
        #[allow(missing_docs)]
        pub avs: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub params: alloy::sol_types::private::Vec<
            <IAllocationManagerTypes::CreateSetParams as alloy::sol_types::SolType>::RustType,
        >,
    }
    ///Container type for the return parameters of the [`createOperatorSets(address,(uint32,address[])[])`](createOperatorSetsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct createOperatorSetsReturn {}
    #[allow(
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
                alloy::sol_types::sol_data::Array<
                    IAllocationManagerTypes::CreateSetParams,
                >,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Vec<
                    <IAllocationManagerTypes::CreateSetParams as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<createOperatorSetsCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: createOperatorSetsCall) -> Self {
                    (value.avs, value.params)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for createOperatorSetsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        avs: tuple.0,
                        params: tuple.1,
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
            impl ::core::convert::From<createOperatorSetsReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: createOperatorSetsReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for createOperatorSetsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for createOperatorSetsCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Array<
                    IAllocationManagerTypes::CreateSetParams,
                >,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = createOperatorSetsReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "createOperatorSets(address,(uint32,address[])[])";
            const SELECTOR: [u8; 4] = [38u8, 31u8, 132u8, 224u8];
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
                        &self.avs,
                    ),
                    <alloy::sol_types::sol_data::Array<
                        IAllocationManagerTypes::CreateSetParams,
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
    /**Function with signature `delegation()` and selector `0xdf5cf723`.
```solidity
function delegation() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct delegationCall {}
    ///Container type for the return parameters of the [`delegation()`](delegationCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct delegationReturn {
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
            impl ::core::convert::From<delegationCall> for UnderlyingRustTuple<'_> {
                fn from(value: delegationCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for delegationCall {
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
            impl ::core::convert::From<delegationReturn> for UnderlyingRustTuple<'_> {
                fn from(value: delegationReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for delegationReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for delegationCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = delegationReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "delegation()";
            const SELECTOR: [u8; 4] = [223u8, 92u8, 247u8, 35u8];
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
    /**Function with signature `deregisterFromOperatorSets((address,address,uint32[]))` and selector `0x6e3492b5`.
```solidity
function deregisterFromOperatorSets(IAllocationManagerTypes.DeregisterParams memory params) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct deregisterFromOperatorSetsCall {
        #[allow(missing_docs)]
        pub params: <IAllocationManagerTypes::DeregisterParams as alloy::sol_types::SolType>::RustType,
    }
    ///Container type for the return parameters of the [`deregisterFromOperatorSets((address,address,uint32[]))`](deregisterFromOperatorSetsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct deregisterFromOperatorSetsReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (IAllocationManagerTypes::DeregisterParams,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <IAllocationManagerTypes::DeregisterParams as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<deregisterFromOperatorSetsCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: deregisterFromOperatorSetsCall) -> Self {
                    (value.params,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for deregisterFromOperatorSetsCall {
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
            impl ::core::convert::From<deregisterFromOperatorSetsReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: deregisterFromOperatorSetsReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for deregisterFromOperatorSetsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for deregisterFromOperatorSetsCall {
            type Parameters<'a> = (IAllocationManagerTypes::DeregisterParams,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = deregisterFromOperatorSetsReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "deregisterFromOperatorSets((address,address,uint32[]))";
            const SELECTOR: [u8; 4] = [110u8, 52u8, 146u8, 181u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <IAllocationManagerTypes::DeregisterParams as alloy_sol_types::SolType>::tokenize(
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
    /**Function with signature `getAVSRegistrar(address)` and selector `0x304c10cd`.
```solidity
function getAVSRegistrar(address avs) external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getAVSRegistrarCall {
        #[allow(missing_docs)]
        pub avs: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`getAVSRegistrar(address)`](getAVSRegistrarCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getAVSRegistrarReturn {
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
            impl ::core::convert::From<getAVSRegistrarCall> for UnderlyingRustTuple<'_> {
                fn from(value: getAVSRegistrarCall) -> Self {
                    (value.avs,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getAVSRegistrarCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { avs: tuple.0 }
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
            impl ::core::convert::From<getAVSRegistrarReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getAVSRegistrarReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getAVSRegistrarReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getAVSRegistrarCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getAVSRegistrarReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getAVSRegistrar(address)";
            const SELECTOR: [u8; 4] = [48u8, 76u8, 16u8, 205u8];
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
                        &self.avs,
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
    /**Function with signature `getAllocatableMagnitude(address,address)` and selector `0x6cfb4481`.
```solidity
function getAllocatableMagnitude(address operator, address strategy) external view returns (uint64);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getAllocatableMagnitudeCall {
        #[allow(missing_docs)]
        pub operator: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub strategy: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`getAllocatableMagnitude(address,address)`](getAllocatableMagnitudeCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getAllocatableMagnitudeReturn {
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
            impl ::core::convert::From<getAllocatableMagnitudeCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getAllocatableMagnitudeCall) -> Self {
                    (value.operator, value.strategy)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getAllocatableMagnitudeCall {
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
            impl ::core::convert::From<getAllocatableMagnitudeReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getAllocatableMagnitudeReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getAllocatableMagnitudeReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getAllocatableMagnitudeCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getAllocatableMagnitudeReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<64>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getAllocatableMagnitude(address,address)";
            const SELECTOR: [u8; 4] = [108u8, 251u8, 68u8, 129u8];
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
    /**Function with signature `getAllocatedSets(address)` and selector `0x15fe5028`.
```solidity
function getAllocatedSets(address operator) external view returns (OperatorSet[] memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getAllocatedSetsCall {
        #[allow(missing_docs)]
        pub operator: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`getAllocatedSets(address)`](getAllocatedSetsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getAllocatedSetsReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::Vec<
            <OperatorSet as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<getAllocatedSetsCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getAllocatedSetsCall) -> Self {
                    (value.operator,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getAllocatedSetsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { operator: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<OperatorSet>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<
                    <OperatorSet as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<getAllocatedSetsReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getAllocatedSetsReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getAllocatedSetsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getAllocatedSetsCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getAllocatedSetsReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Array<OperatorSet>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getAllocatedSets(address)";
            const SELECTOR: [u8; 4] = [21u8, 254u8, 80u8, 40u8];
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
    /**Function with signature `getAllocatedStake((address,uint32),address[],address[])` and selector `0x2b453a9a`.
```solidity
function getAllocatedStake(OperatorSet memory operatorSet, address[] memory operators, address[] memory strategies) external view returns (uint256[][] memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getAllocatedStakeCall {
        #[allow(missing_docs)]
        pub operatorSet: <OperatorSet as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub operators: alloy::sol_types::private::Vec<
            alloy::sol_types::private::Address,
        >,
        #[allow(missing_docs)]
        pub strategies: alloy::sol_types::private::Vec<
            alloy::sol_types::private::Address,
        >,
    }
    ///Container type for the return parameters of the [`getAllocatedStake((address,uint32),address[],address[])`](getAllocatedStakeCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getAllocatedStakeReturn {
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
                OperatorSet,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <OperatorSet as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<getAllocatedStakeCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getAllocatedStakeCall) -> Self {
                    (value.operatorSet, value.operators, value.strategies)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getAllocatedStakeCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        operatorSet: tuple.0,
                        operators: tuple.1,
                        strategies: tuple.2,
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
            impl ::core::convert::From<getAllocatedStakeReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getAllocatedStakeReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getAllocatedStakeReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getAllocatedStakeCall {
            type Parameters<'a> = (
                OperatorSet,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getAllocatedStakeReturn;
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
            const SIGNATURE: &'static str = "getAllocatedStake((address,uint32),address[],address[])";
            const SELECTOR: [u8; 4] = [43u8, 69u8, 58u8, 154u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <OperatorSet as alloy_sol_types::SolType>::tokenize(
                        &self.operatorSet,
                    ),
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
    /**Function with signature `getAllocatedStrategies(address,(address,uint32))` and selector `0xc221d8ae`.
```solidity
function getAllocatedStrategies(address operator, OperatorSet memory operatorSet) external view returns (address[] memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getAllocatedStrategiesCall {
        #[allow(missing_docs)]
        pub operator: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub operatorSet: <OperatorSet as alloy::sol_types::SolType>::RustType,
    }
    ///Container type for the return parameters of the [`getAllocatedStrategies(address,(address,uint32))`](getAllocatedStrategiesCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getAllocatedStrategiesReturn {
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
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                OperatorSet,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                <OperatorSet as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<getAllocatedStrategiesCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getAllocatedStrategiesCall) -> Self {
                    (value.operator, value.operatorSet)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getAllocatedStrategiesCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        operator: tuple.0,
                        operatorSet: tuple.1,
                    }
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
            impl ::core::convert::From<getAllocatedStrategiesReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getAllocatedStrategiesReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getAllocatedStrategiesReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getAllocatedStrategiesCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address, OperatorSet);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getAllocatedStrategiesReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getAllocatedStrategies(address,(address,uint32))";
            const SELECTOR: [u8; 4] = [194u8, 33u8, 216u8, 174u8];
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
                    <OperatorSet as alloy_sol_types::SolType>::tokenize(
                        &self.operatorSet,
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
    /**Function with signature `getAllocation(address,(address,uint32),address)` and selector `0x10e1b9b8`.
```solidity
function getAllocation(address operator, OperatorSet memory operatorSet, address strategy) external view returns (IAllocationManagerTypes.Allocation memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getAllocationCall {
        #[allow(missing_docs)]
        pub operator: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub operatorSet: <OperatorSet as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub strategy: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`getAllocation(address,(address,uint32),address)`](getAllocationCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getAllocationReturn {
        #[allow(missing_docs)]
        pub _0: <IAllocationManagerTypes::Allocation as alloy::sol_types::SolType>::RustType,
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
                OperatorSet,
                alloy::sol_types::sol_data::Address,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                <OperatorSet as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<getAllocationCall> for UnderlyingRustTuple<'_> {
                fn from(value: getAllocationCall) -> Self {
                    (value.operator, value.operatorSet, value.strategy)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getAllocationCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        operator: tuple.0,
                        operatorSet: tuple.1,
                        strategy: tuple.2,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (IAllocationManagerTypes::Allocation,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <IAllocationManagerTypes::Allocation as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<getAllocationReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getAllocationReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getAllocationReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getAllocationCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                OperatorSet,
                alloy::sol_types::sol_data::Address,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getAllocationReturn;
            type ReturnTuple<'a> = (IAllocationManagerTypes::Allocation,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getAllocation(address,(address,uint32),address)";
            const SELECTOR: [u8; 4] = [16u8, 225u8, 185u8, 184u8];
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
                    <OperatorSet as alloy_sol_types::SolType>::tokenize(
                        &self.operatorSet,
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
    /**Function with signature `getAllocationDelay(address)` and selector `0xb9fbaed1`.
```solidity
function getAllocationDelay(address operator) external view returns (bool, uint32);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getAllocationDelayCall {
        #[allow(missing_docs)]
        pub operator: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`getAllocationDelay(address)`](getAllocationDelayCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getAllocationDelayReturn {
        #[allow(missing_docs)]
        pub _0: bool,
        #[allow(missing_docs)]
        pub _1: u32,
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
            impl ::core::convert::From<getAllocationDelayCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getAllocationDelayCall) -> Self {
                    (value.operator,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getAllocationDelayCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { operator: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Bool,
                alloy::sol_types::sol_data::Uint<32>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (bool, u32);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getAllocationDelayReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getAllocationDelayReturn) -> Self {
                    (value._0, value._1)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getAllocationDelayReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0, _1: tuple.1 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getAllocationDelayCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getAllocationDelayReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Bool,
                alloy::sol_types::sol_data::Uint<32>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getAllocationDelay(address)";
            const SELECTOR: [u8; 4] = [185u8, 251u8, 174u8, 209u8];
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
    /**Function with signature `getAllocations(address[],(address,uint32),address)` and selector `0x8ce64854`.
```solidity
function getAllocations(address[] memory operators, OperatorSet memory operatorSet, address strategy) external view returns (IAllocationManagerTypes.Allocation[] memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getAllocationsCall {
        #[allow(missing_docs)]
        pub operators: alloy::sol_types::private::Vec<
            alloy::sol_types::private::Address,
        >,
        #[allow(missing_docs)]
        pub operatorSet: <OperatorSet as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub strategy: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`getAllocations(address[],(address,uint32),address)`](getAllocationsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getAllocationsReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::Vec<
            <IAllocationManagerTypes::Allocation as alloy::sol_types::SolType>::RustType,
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
                OperatorSet,
                alloy::sol_types::sol_data::Address,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
                <OperatorSet as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<getAllocationsCall> for UnderlyingRustTuple<'_> {
                fn from(value: getAllocationsCall) -> Self {
                    (value.operators, value.operatorSet, value.strategy)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getAllocationsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        operators: tuple.0,
                        operatorSet: tuple.1,
                        strategy: tuple.2,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<IAllocationManagerTypes::Allocation>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<
                    <IAllocationManagerTypes::Allocation as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<getAllocationsReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getAllocationsReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getAllocationsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getAllocationsCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
                OperatorSet,
                alloy::sol_types::sol_data::Address,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getAllocationsReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<IAllocationManagerTypes::Allocation>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getAllocations(address[],(address,uint32),address)";
            const SELECTOR: [u8; 4] = [140u8, 230u8, 72u8, 84u8];
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
                    <OperatorSet as alloy_sol_types::SolType>::tokenize(
                        &self.operatorSet,
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
    /**Function with signature `getEncumberedMagnitude(address,address)` and selector `0xf605ce08`.
```solidity
function getEncumberedMagnitude(address operator, address strategy) external view returns (uint64);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getEncumberedMagnitudeCall {
        #[allow(missing_docs)]
        pub operator: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub strategy: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`getEncumberedMagnitude(address,address)`](getEncumberedMagnitudeCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getEncumberedMagnitudeReturn {
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
            impl ::core::convert::From<getEncumberedMagnitudeCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getEncumberedMagnitudeCall) -> Self {
                    (value.operator, value.strategy)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getEncumberedMagnitudeCall {
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
            impl ::core::convert::From<getEncumberedMagnitudeReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getEncumberedMagnitudeReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getEncumberedMagnitudeReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getEncumberedMagnitudeCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getEncumberedMagnitudeReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<64>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getEncumberedMagnitude(address,address)";
            const SELECTOR: [u8; 4] = [246u8, 5u8, 206u8, 8u8];
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
    /**Function with signature `getMaxMagnitude(address,address)` and selector `0xa9333ec8`.
```solidity
function getMaxMagnitude(address operator, address strategy) external view returns (uint64);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getMaxMagnitudeCall {
        #[allow(missing_docs)]
        pub operator: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub strategy: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`getMaxMagnitude(address,address)`](getMaxMagnitudeCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getMaxMagnitudeReturn {
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
            impl ::core::convert::From<getMaxMagnitudeCall> for UnderlyingRustTuple<'_> {
                fn from(value: getMaxMagnitudeCall) -> Self {
                    (value.operator, value.strategy)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getMaxMagnitudeCall {
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
            impl ::core::convert::From<getMaxMagnitudeReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getMaxMagnitudeReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getMaxMagnitudeReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getMaxMagnitudeCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getMaxMagnitudeReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<64>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getMaxMagnitude(address,address)";
            const SELECTOR: [u8; 4] = [169u8, 51u8, 62u8, 200u8];
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
    /**Function with signature `getMaxMagnitudes(address[],address)` and selector `0x4a10ffe5`.
```solidity
function getMaxMagnitudes(address[] memory operators, address strategy) external view returns (uint64[] memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getMaxMagnitudes_0Call {
        #[allow(missing_docs)]
        pub operators: alloy::sol_types::private::Vec<
            alloy::sol_types::private::Address,
        >,
        #[allow(missing_docs)]
        pub strategy: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`getMaxMagnitudes(address[],address)`](getMaxMagnitudes_0Call) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getMaxMagnitudes_0Return {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::Vec<u64>,
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
                alloy::sol_types::sol_data::Address,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
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
            impl ::core::convert::From<getMaxMagnitudes_0Call>
            for UnderlyingRustTuple<'_> {
                fn from(value: getMaxMagnitudes_0Call) -> Self {
                    (value.operators, value.strategy)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getMaxMagnitudes_0Call {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        operators: tuple.0,
                        strategy: tuple.1,
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
            impl ::core::convert::From<getMaxMagnitudes_0Return>
            for UnderlyingRustTuple<'_> {
                fn from(value: getMaxMagnitudes_0Return) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getMaxMagnitudes_0Return {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getMaxMagnitudes_0Call {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
                alloy::sol_types::sol_data::Address,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getMaxMagnitudes_0Return;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<64>>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getMaxMagnitudes(address[],address)";
            const SELECTOR: [u8; 4] = [74u8, 16u8, 255u8, 229u8];
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
    /**Function with signature `getMaxMagnitudes(address,address[])` and selector `0x547afb87`.
```solidity
function getMaxMagnitudes(address operator, address[] memory strategies) external view returns (uint64[] memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getMaxMagnitudes_1Call {
        #[allow(missing_docs)]
        pub operator: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub strategies: alloy::sol_types::private::Vec<
            alloy::sol_types::private::Address,
        >,
    }
    ///Container type for the return parameters of the [`getMaxMagnitudes(address,address[])`](getMaxMagnitudes_1Call) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getMaxMagnitudes_1Return {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::Vec<u64>,
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
            impl ::core::convert::From<getMaxMagnitudes_1Call>
            for UnderlyingRustTuple<'_> {
                fn from(value: getMaxMagnitudes_1Call) -> Self {
                    (value.operator, value.strategies)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getMaxMagnitudes_1Call {
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
            impl ::core::convert::From<getMaxMagnitudes_1Return>
            for UnderlyingRustTuple<'_> {
                fn from(value: getMaxMagnitudes_1Return) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getMaxMagnitudes_1Return {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getMaxMagnitudes_1Call {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getMaxMagnitudes_1Return;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<64>>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getMaxMagnitudes(address,address[])";
            const SELECTOR: [u8; 4] = [84u8, 122u8, 251u8, 135u8];
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
    /**Function with signature `getMaxMagnitudesAtBlock(address,address[],uint32)` and selector `0x94d7d00c`.
```solidity
function getMaxMagnitudesAtBlock(address operator, address[] memory strategies, uint32 blockNumber) external view returns (uint64[] memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getMaxMagnitudesAtBlockCall {
        #[allow(missing_docs)]
        pub operator: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub strategies: alloy::sol_types::private::Vec<
            alloy::sol_types::private::Address,
        >,
        #[allow(missing_docs)]
        pub blockNumber: u32,
    }
    ///Container type for the return parameters of the [`getMaxMagnitudesAtBlock(address,address[],uint32)`](getMaxMagnitudesAtBlockCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getMaxMagnitudesAtBlockReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::Vec<u64>,
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
                alloy::sol_types::sol_data::Uint<32>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
                u32,
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
            impl ::core::convert::From<getMaxMagnitudesAtBlockCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getMaxMagnitudesAtBlockCall) -> Self {
                    (value.operator, value.strategies, value.blockNumber)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getMaxMagnitudesAtBlockCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        operator: tuple.0,
                        strategies: tuple.1,
                        blockNumber: tuple.2,
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
            impl ::core::convert::From<getMaxMagnitudesAtBlockReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getMaxMagnitudesAtBlockReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getMaxMagnitudesAtBlockReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getMaxMagnitudesAtBlockCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
                alloy::sol_types::sol_data::Uint<32>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getMaxMagnitudesAtBlockReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<64>>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getMaxMagnitudesAtBlock(address,address[],uint32)";
            const SELECTOR: [u8; 4] = [148u8, 215u8, 208u8, 12u8];
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
                    <alloy::sol_types::sol_data::Uint<
                        32,
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
    /**Function with signature `getMemberCount((address,uint32))` and selector `0xb2447af7`.
```solidity
function getMemberCount(OperatorSet memory operatorSet) external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getMemberCountCall {
        #[allow(missing_docs)]
        pub operatorSet: <OperatorSet as alloy::sol_types::SolType>::RustType,
    }
    ///Container type for the return parameters of the [`getMemberCount((address,uint32))`](getMemberCountCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getMemberCountReturn {
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
            type UnderlyingSolTuple<'a> = (OperatorSet,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <OperatorSet as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<getMemberCountCall> for UnderlyingRustTuple<'_> {
                fn from(value: getMemberCountCall) -> Self {
                    (value.operatorSet,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getMemberCountCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { operatorSet: tuple.0 }
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
            impl ::core::convert::From<getMemberCountReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getMemberCountReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getMemberCountReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getMemberCountCall {
            type Parameters<'a> = (OperatorSet,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getMemberCountReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getMemberCount((address,uint32))";
            const SELECTOR: [u8; 4] = [178u8, 68u8, 122u8, 247u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (<OperatorSet as alloy_sol_types::SolType>::tokenize(&self.operatorSet),)
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
    /**Function with signature `getMembers((address,uint32))` and selector `0x6e875dba`.
```solidity
function getMembers(OperatorSet memory operatorSet) external view returns (address[] memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getMembersCall {
        #[allow(missing_docs)]
        pub operatorSet: <OperatorSet as alloy::sol_types::SolType>::RustType,
    }
    ///Container type for the return parameters of the [`getMembers((address,uint32))`](getMembersCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getMembersReturn {
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
            type UnderlyingSolTuple<'a> = (OperatorSet,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <OperatorSet as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<getMembersCall> for UnderlyingRustTuple<'_> {
                fn from(value: getMembersCall) -> Self {
                    (value.operatorSet,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getMembersCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { operatorSet: tuple.0 }
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
            impl ::core::convert::From<getMembersReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getMembersReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getMembersReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getMembersCall {
            type Parameters<'a> = (OperatorSet,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getMembersReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getMembers((address,uint32))";
            const SELECTOR: [u8; 4] = [110u8, 135u8, 93u8, 186u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (<OperatorSet as alloy_sol_types::SolType>::tokenize(&self.operatorSet),)
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
    /**Function with signature `getMinimumSlashableStake((address,uint32),address[],address[],uint32)` and selector `0x2bab2c4a`.
```solidity
function getMinimumSlashableStake(OperatorSet memory operatorSet, address[] memory operators, address[] memory strategies, uint32 futureBlock) external view returns (uint256[][] memory slashableStake);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getMinimumSlashableStakeCall {
        #[allow(missing_docs)]
        pub operatorSet: <OperatorSet as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub operators: alloy::sol_types::private::Vec<
            alloy::sol_types::private::Address,
        >,
        #[allow(missing_docs)]
        pub strategies: alloy::sol_types::private::Vec<
            alloy::sol_types::private::Address,
        >,
        #[allow(missing_docs)]
        pub futureBlock: u32,
    }
    ///Container type for the return parameters of the [`getMinimumSlashableStake((address,uint32),address[],address[],uint32)`](getMinimumSlashableStakeCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getMinimumSlashableStakeReturn {
        #[allow(missing_docs)]
        pub slashableStake: alloy::sol_types::private::Vec<
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
                OperatorSet,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
                alloy::sol_types::sol_data::Uint<32>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <OperatorSet as alloy::sol_types::SolType>::RustType,
                alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
                alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
                u32,
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
            impl ::core::convert::From<getMinimumSlashableStakeCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getMinimumSlashableStakeCall) -> Self {
                    (
                        value.operatorSet,
                        value.operators,
                        value.strategies,
                        value.futureBlock,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getMinimumSlashableStakeCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        operatorSet: tuple.0,
                        operators: tuple.1,
                        strategies: tuple.2,
                        futureBlock: tuple.3,
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
            impl ::core::convert::From<getMinimumSlashableStakeReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getMinimumSlashableStakeReturn) -> Self {
                    (value.slashableStake,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getMinimumSlashableStakeReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { slashableStake: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getMinimumSlashableStakeCall {
            type Parameters<'a> = (
                OperatorSet,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
                alloy::sol_types::sol_data::Uint<32>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getMinimumSlashableStakeReturn;
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
            const SIGNATURE: &'static str = "getMinimumSlashableStake((address,uint32),address[],address[],uint32)";
            const SELECTOR: [u8; 4] = [43u8, 171u8, 44u8, 74u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <OperatorSet as alloy_sol_types::SolType>::tokenize(
                        &self.operatorSet,
                    ),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Address,
                    > as alloy_sol_types::SolType>::tokenize(&self.operators),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Address,
                    > as alloy_sol_types::SolType>::tokenize(&self.strategies),
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.futureBlock),
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
    /**Function with signature `getOperatorSetCount(address)` and selector `0xba1a84e5`.
```solidity
function getOperatorSetCount(address avs) external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getOperatorSetCountCall {
        #[allow(missing_docs)]
        pub avs: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`getOperatorSetCount(address)`](getOperatorSetCountCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getOperatorSetCountReturn {
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
            impl ::core::convert::From<getOperatorSetCountCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getOperatorSetCountCall) -> Self {
                    (value.avs,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getOperatorSetCountCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { avs: tuple.0 }
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
            impl ::core::convert::From<getOperatorSetCountReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getOperatorSetCountReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getOperatorSetCountReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getOperatorSetCountCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getOperatorSetCountReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getOperatorSetCount(address)";
            const SELECTOR: [u8; 4] = [186u8, 26u8, 132u8, 229u8];
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
                        &self.avs,
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
    /**Function with signature `getRegisteredSets(address)` and selector `0x79ae50cd`.
```solidity
function getRegisteredSets(address operator) external view returns (OperatorSet[] memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getRegisteredSetsCall {
        #[allow(missing_docs)]
        pub operator: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`getRegisteredSets(address)`](getRegisteredSetsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getRegisteredSetsReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::Vec<
            <OperatorSet as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<getRegisteredSetsCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getRegisteredSetsCall) -> Self {
                    (value.operator,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getRegisteredSetsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { operator: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<OperatorSet>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<
                    <OperatorSet as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<getRegisteredSetsReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getRegisteredSetsReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getRegisteredSetsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getRegisteredSetsCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getRegisteredSetsReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Array<OperatorSet>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getRegisteredSets(address)";
            const SELECTOR: [u8; 4] = [121u8, 174u8, 80u8, 205u8];
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
    /**Function with signature `getStrategiesInOperatorSet((address,uint32))` and selector `0x4177a87c`.
```solidity
function getStrategiesInOperatorSet(OperatorSet memory operatorSet) external view returns (address[] memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getStrategiesInOperatorSetCall {
        #[allow(missing_docs)]
        pub operatorSet: <OperatorSet as alloy::sol_types::SolType>::RustType,
    }
    ///Container type for the return parameters of the [`getStrategiesInOperatorSet((address,uint32))`](getStrategiesInOperatorSetCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getStrategiesInOperatorSetReturn {
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
            type UnderlyingSolTuple<'a> = (OperatorSet,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <OperatorSet as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<getStrategiesInOperatorSetCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getStrategiesInOperatorSetCall) -> Self {
                    (value.operatorSet,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getStrategiesInOperatorSetCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { operatorSet: tuple.0 }
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
            impl ::core::convert::From<getStrategiesInOperatorSetReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getStrategiesInOperatorSetReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getStrategiesInOperatorSetReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getStrategiesInOperatorSetCall {
            type Parameters<'a> = (OperatorSet,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getStrategiesInOperatorSetReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getStrategiesInOperatorSet((address,uint32))";
            const SELECTOR: [u8; 4] = [65u8, 119u8, 168u8, 124u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (<OperatorSet as alloy_sol_types::SolType>::tokenize(&self.operatorSet),)
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
    /**Function with signature `getStrategyAllocations(address,address)` and selector `0x40120dab`.
```solidity
function getStrategyAllocations(address operator, address strategy) external view returns (OperatorSet[] memory, IAllocationManagerTypes.Allocation[] memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getStrategyAllocationsCall {
        #[allow(missing_docs)]
        pub operator: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub strategy: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`getStrategyAllocations(address,address)`](getStrategyAllocationsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getStrategyAllocationsReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::Vec<
            <OperatorSet as alloy::sol_types::SolType>::RustType,
        >,
        #[allow(missing_docs)]
        pub _1: alloy::sol_types::private::Vec<
            <IAllocationManagerTypes::Allocation as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<getStrategyAllocationsCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getStrategyAllocationsCall) -> Self {
                    (value.operator, value.strategy)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getStrategyAllocationsCall {
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
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<OperatorSet>,
                alloy::sol_types::sol_data::Array<IAllocationManagerTypes::Allocation>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<
                    <OperatorSet as alloy::sol_types::SolType>::RustType,
                >,
                alloy::sol_types::private::Vec<
                    <IAllocationManagerTypes::Allocation as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<getStrategyAllocationsReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getStrategyAllocationsReturn) -> Self {
                    (value._0, value._1)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getStrategyAllocationsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0, _1: tuple.1 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getStrategyAllocationsCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getStrategyAllocationsReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<OperatorSet>,
                alloy::sol_types::sol_data::Array<IAllocationManagerTypes::Allocation>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getStrategyAllocations(address,address)";
            const SELECTOR: [u8; 4] = [64u8, 18u8, 13u8, 171u8];
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
    /**Function with signature `isMemberOfOperatorSet(address,(address,uint32))` and selector `0x670d3ba2`.
```solidity
function isMemberOfOperatorSet(address operator, OperatorSet memory operatorSet) external view returns (bool);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isMemberOfOperatorSetCall {
        #[allow(missing_docs)]
        pub operator: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub operatorSet: <OperatorSet as alloy::sol_types::SolType>::RustType,
    }
    ///Container type for the return parameters of the [`isMemberOfOperatorSet(address,(address,uint32))`](isMemberOfOperatorSetCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isMemberOfOperatorSetReturn {
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
                alloy::sol_types::sol_data::Address,
                OperatorSet,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                <OperatorSet as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<isMemberOfOperatorSetCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: isMemberOfOperatorSetCall) -> Self {
                    (value.operator, value.operatorSet)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for isMemberOfOperatorSetCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        operator: tuple.0,
                        operatorSet: tuple.1,
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
            impl ::core::convert::From<isMemberOfOperatorSetReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: isMemberOfOperatorSetReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for isMemberOfOperatorSetReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for isMemberOfOperatorSetCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address, OperatorSet);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = isMemberOfOperatorSetReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "isMemberOfOperatorSet(address,(address,uint32))";
            const SELECTOR: [u8; 4] = [103u8, 13u8, 59u8, 162u8];
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
                    <OperatorSet as alloy_sol_types::SolType>::tokenize(
                        &self.operatorSet,
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
    /**Function with signature `isOperatorSet((address,uint32))` and selector `0x260dc758`.
```solidity
function isOperatorSet(OperatorSet memory operatorSet) external view returns (bool);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isOperatorSetCall {
        #[allow(missing_docs)]
        pub operatorSet: <OperatorSet as alloy::sol_types::SolType>::RustType,
    }
    ///Container type for the return parameters of the [`isOperatorSet((address,uint32))`](isOperatorSetCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isOperatorSetReturn {
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
            type UnderlyingSolTuple<'a> = (OperatorSet,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <OperatorSet as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<isOperatorSetCall> for UnderlyingRustTuple<'_> {
                fn from(value: isOperatorSetCall) -> Self {
                    (value.operatorSet,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for isOperatorSetCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { operatorSet: tuple.0 }
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
            impl ::core::convert::From<isOperatorSetReturn> for UnderlyingRustTuple<'_> {
                fn from(value: isOperatorSetReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for isOperatorSetReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for isOperatorSetCall {
            type Parameters<'a> = (OperatorSet,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = isOperatorSetReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "isOperatorSet((address,uint32))";
            const SELECTOR: [u8; 4] = [38u8, 13u8, 199u8, 88u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (<OperatorSet as alloy_sol_types::SolType>::tokenize(&self.operatorSet),)
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
    /**Function with signature `isOperatorSlashable(address,(address,uint32))` and selector `0x1352c3e6`.
```solidity
function isOperatorSlashable(address operator, OperatorSet memory operatorSet) external view returns (bool);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isOperatorSlashableCall {
        #[allow(missing_docs)]
        pub operator: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub operatorSet: <OperatorSet as alloy::sol_types::SolType>::RustType,
    }
    ///Container type for the return parameters of the [`isOperatorSlashable(address,(address,uint32))`](isOperatorSlashableCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isOperatorSlashableReturn {
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
                alloy::sol_types::sol_data::Address,
                OperatorSet,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                <OperatorSet as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<isOperatorSlashableCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: isOperatorSlashableCall) -> Self {
                    (value.operator, value.operatorSet)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for isOperatorSlashableCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        operator: tuple.0,
                        operatorSet: tuple.1,
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
            impl ::core::convert::From<isOperatorSlashableReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: isOperatorSlashableReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for isOperatorSlashableReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for isOperatorSlashableCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address, OperatorSet);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = isOperatorSlashableReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "isOperatorSlashable(address,(address,uint32))";
            const SELECTOR: [u8; 4] = [19u8, 82u8, 195u8, 230u8];
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
                    <OperatorSet as alloy_sol_types::SolType>::tokenize(
                        &self.operatorSet,
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
    /**Function with signature `modifyAllocations(address,((address,uint32),address[],uint64[])[])` and selector `0x952899ee`.
```solidity
function modifyAllocations(address operator, IAllocationManagerTypes.AllocateParams[] memory params) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct modifyAllocationsCall {
        #[allow(missing_docs)]
        pub operator: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub params: alloy::sol_types::private::Vec<
            <IAllocationManagerTypes::AllocateParams as alloy::sol_types::SolType>::RustType,
        >,
    }
    ///Container type for the return parameters of the [`modifyAllocations(address,((address,uint32),address[],uint64[])[])`](modifyAllocationsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct modifyAllocationsReturn {}
    #[allow(
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
                alloy::sol_types::sol_data::Array<
                    IAllocationManagerTypes::AllocateParams,
                >,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Vec<
                    <IAllocationManagerTypes::AllocateParams as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<modifyAllocationsCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: modifyAllocationsCall) -> Self {
                    (value.operator, value.params)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for modifyAllocationsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        operator: tuple.0,
                        params: tuple.1,
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
            impl ::core::convert::From<modifyAllocationsReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: modifyAllocationsReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for modifyAllocationsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for modifyAllocationsCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Array<
                    IAllocationManagerTypes::AllocateParams,
                >,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = modifyAllocationsReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "modifyAllocations(address,((address,uint32),address[],uint64[])[])";
            const SELECTOR: [u8; 4] = [149u8, 40u8, 153u8, 238u8];
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
                        IAllocationManagerTypes::AllocateParams,
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
    /**Function with signature `registerForOperatorSets(address,(address,uint32[],bytes))` and selector `0xadc2e3d9`.
```solidity
function registerForOperatorSets(address operator, IAllocationManagerTypes.RegisterParams memory params) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct registerForOperatorSetsCall {
        #[allow(missing_docs)]
        pub operator: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub params: <IAllocationManagerTypes::RegisterParams as alloy::sol_types::SolType>::RustType,
    }
    ///Container type for the return parameters of the [`registerForOperatorSets(address,(address,uint32[],bytes))`](registerForOperatorSetsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct registerForOperatorSetsReturn {}
    #[allow(
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
                IAllocationManagerTypes::RegisterParams,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                <IAllocationManagerTypes::RegisterParams as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<registerForOperatorSetsCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: registerForOperatorSetsCall) -> Self {
                    (value.operator, value.params)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for registerForOperatorSetsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        operator: tuple.0,
                        params: tuple.1,
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
            impl ::core::convert::From<registerForOperatorSetsReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: registerForOperatorSetsReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for registerForOperatorSetsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for registerForOperatorSetsCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                IAllocationManagerTypes::RegisterParams,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = registerForOperatorSetsReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "registerForOperatorSets(address,(address,uint32[],bytes))";
            const SELECTOR: [u8; 4] = [173u8, 194u8, 227u8, 217u8];
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
                    <IAllocationManagerTypes::RegisterParams as alloy_sol_types::SolType>::tokenize(
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
    /**Function with signature `removeStrategiesFromOperatorSet(address,uint32,address[])` and selector `0xb66bd989`.
```solidity
function removeStrategiesFromOperatorSet(address avs, uint32 operatorSetId, address[] memory strategies) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct removeStrategiesFromOperatorSetCall {
        #[allow(missing_docs)]
        pub avs: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub operatorSetId: u32,
        #[allow(missing_docs)]
        pub strategies: alloy::sol_types::private::Vec<
            alloy::sol_types::private::Address,
        >,
    }
    ///Container type for the return parameters of the [`removeStrategiesFromOperatorSet(address,uint32,address[])`](removeStrategiesFromOperatorSetCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct removeStrategiesFromOperatorSetReturn {}
    #[allow(
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
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                u32,
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
            impl ::core::convert::From<removeStrategiesFromOperatorSetCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: removeStrategiesFromOperatorSetCall) -> Self {
                    (value.avs, value.operatorSetId, value.strategies)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for removeStrategiesFromOperatorSetCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        avs: tuple.0,
                        operatorSetId: tuple.1,
                        strategies: tuple.2,
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
            impl ::core::convert::From<removeStrategiesFromOperatorSetReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: removeStrategiesFromOperatorSetReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for removeStrategiesFromOperatorSetReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for removeStrategiesFromOperatorSetCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<32>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = removeStrategiesFromOperatorSetReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "removeStrategiesFromOperatorSet(address,uint32,address[])";
            const SELECTOR: [u8; 4] = [182u8, 107u8, 217u8, 137u8];
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
                        &self.avs,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.operatorSetId),
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
    /**Function with signature `setAVSRegistrar(address,address)` and selector `0xd3d96ff4`.
```solidity
function setAVSRegistrar(address avs, address registrar) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setAVSRegistrarCall {
        #[allow(missing_docs)]
        pub avs: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub registrar: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`setAVSRegistrar(address,address)`](setAVSRegistrarCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setAVSRegistrarReturn {}
    #[allow(
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
            impl ::core::convert::From<setAVSRegistrarCall> for UnderlyingRustTuple<'_> {
                fn from(value: setAVSRegistrarCall) -> Self {
                    (value.avs, value.registrar)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for setAVSRegistrarCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        avs: tuple.0,
                        registrar: tuple.1,
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
            impl ::core::convert::From<setAVSRegistrarReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: setAVSRegistrarReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for setAVSRegistrarReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for setAVSRegistrarCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = setAVSRegistrarReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "setAVSRegistrar(address,address)";
            const SELECTOR: [u8; 4] = [211u8, 217u8, 111u8, 244u8];
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
                        &self.avs,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.registrar,
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
    /**Function with signature `setAllocationDelay(address,uint32)` and selector `0x56c483e6`.
```solidity
function setAllocationDelay(address operator, uint32 delay) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setAllocationDelayCall {
        #[allow(missing_docs)]
        pub operator: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub delay: u32,
    }
    ///Container type for the return parameters of the [`setAllocationDelay(address,uint32)`](setAllocationDelayCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setAllocationDelayReturn {}
    #[allow(
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
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address, u32);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<setAllocationDelayCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: setAllocationDelayCall) -> Self {
                    (value.operator, value.delay)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for setAllocationDelayCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        operator: tuple.0,
                        delay: tuple.1,
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
            impl ::core::convert::From<setAllocationDelayReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: setAllocationDelayReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for setAllocationDelayReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for setAllocationDelayCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<32>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = setAllocationDelayReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "setAllocationDelay(address,uint32)";
            const SELECTOR: [u8; 4] = [86u8, 196u8, 131u8, 230u8];
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
                    <alloy::sol_types::sol_data::Uint<
                        32,
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
    /**Function with signature `slashOperator(address,(address,uint32,address[],uint256[],string))` and selector `0x36352057`.
```solidity
function slashOperator(address avs, IAllocationManagerTypes.SlashingParams memory params) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct slashOperatorCall {
        #[allow(missing_docs)]
        pub avs: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub params: <IAllocationManagerTypes::SlashingParams as alloy::sol_types::SolType>::RustType,
    }
    ///Container type for the return parameters of the [`slashOperator(address,(address,uint32,address[],uint256[],string))`](slashOperatorCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct slashOperatorReturn {}
    #[allow(
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
                IAllocationManagerTypes::SlashingParams,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
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
            impl ::core::convert::From<slashOperatorCall> for UnderlyingRustTuple<'_> {
                fn from(value: slashOperatorCall) -> Self {
                    (value.avs, value.params)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for slashOperatorCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        avs: tuple.0,
                        params: tuple.1,
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
            impl ::core::convert::From<slashOperatorReturn> for UnderlyingRustTuple<'_> {
                fn from(value: slashOperatorReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for slashOperatorReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for slashOperatorCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                IAllocationManagerTypes::SlashingParams,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = slashOperatorReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "slashOperator(address,(address,uint32,address[],uint256[],string))";
            const SELECTOR: [u8; 4] = [54u8, 53u8, 32u8, 87u8];
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
                        &self.avs,
                    ),
                    <IAllocationManagerTypes::SlashingParams as alloy_sol_types::SolType>::tokenize(
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
    /**Function with signature `updateAVSMetadataURI(address,string)` and selector `0xa9821821`.
```solidity
function updateAVSMetadataURI(address avs, string memory metadataURI) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct updateAVSMetadataURICall {
        #[allow(missing_docs)]
        pub avs: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub metadataURI: alloy::sol_types::private::String,
    }
    ///Container type for the return parameters of the [`updateAVSMetadataURI(address,string)`](updateAVSMetadataURICall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct updateAVSMetadataURIReturn {}
    #[allow(
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
            impl ::core::convert::From<updateAVSMetadataURICall>
            for UnderlyingRustTuple<'_> {
                fn from(value: updateAVSMetadataURICall) -> Self {
                    (value.avs, value.metadataURI)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for updateAVSMetadataURICall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        avs: tuple.0,
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
            impl ::core::convert::From<updateAVSMetadataURIReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: updateAVSMetadataURIReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for updateAVSMetadataURIReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for updateAVSMetadataURICall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::String,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = updateAVSMetadataURIReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "updateAVSMetadataURI(address,string)";
            const SELECTOR: [u8; 4] = [169u8, 130u8, 24u8, 33u8];
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
                        &self.avs,
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
    ///Container for all the [`AllocationManager`](self) function calls.
    pub enum AllocationManagerCalls {
        #[allow(missing_docs)]
        ALLOCATION_CONFIGURATION_DELAY(ALLOCATION_CONFIGURATION_DELAYCall),
        #[allow(missing_docs)]
        DEALLOCATION_DELAY(DEALLOCATION_DELAYCall),
        #[allow(missing_docs)]
        addStrategiesToOperatorSet(addStrategiesToOperatorSetCall),
        #[allow(missing_docs)]
        clearDeallocationQueue(clearDeallocationQueueCall),
        #[allow(missing_docs)]
        createOperatorSets(createOperatorSetsCall),
        #[allow(missing_docs)]
        delegation(delegationCall),
        #[allow(missing_docs)]
        deregisterFromOperatorSets(deregisterFromOperatorSetsCall),
        #[allow(missing_docs)]
        getAVSRegistrar(getAVSRegistrarCall),
        #[allow(missing_docs)]
        getAllocatableMagnitude(getAllocatableMagnitudeCall),
        #[allow(missing_docs)]
        getAllocatedSets(getAllocatedSetsCall),
        #[allow(missing_docs)]
        getAllocatedStake(getAllocatedStakeCall),
        #[allow(missing_docs)]
        getAllocatedStrategies(getAllocatedStrategiesCall),
        #[allow(missing_docs)]
        getAllocation(getAllocationCall),
        #[allow(missing_docs)]
        getAllocationDelay(getAllocationDelayCall),
        #[allow(missing_docs)]
        getAllocations(getAllocationsCall),
        #[allow(missing_docs)]
        getEncumberedMagnitude(getEncumberedMagnitudeCall),
        #[allow(missing_docs)]
        getMaxMagnitude(getMaxMagnitudeCall),
        #[allow(missing_docs)]
        getMaxMagnitudes_0(getMaxMagnitudes_0Call),
        #[allow(missing_docs)]
        getMaxMagnitudes_1(getMaxMagnitudes_1Call),
        #[allow(missing_docs)]
        getMaxMagnitudesAtBlock(getMaxMagnitudesAtBlockCall),
        #[allow(missing_docs)]
        getMemberCount(getMemberCountCall),
        #[allow(missing_docs)]
        getMembers(getMembersCall),
        #[allow(missing_docs)]
        getMinimumSlashableStake(getMinimumSlashableStakeCall),
        #[allow(missing_docs)]
        getOperatorSetCount(getOperatorSetCountCall),
        #[allow(missing_docs)]
        getRegisteredSets(getRegisteredSetsCall),
        #[allow(missing_docs)]
        getStrategiesInOperatorSet(getStrategiesInOperatorSetCall),
        #[allow(missing_docs)]
        getStrategyAllocations(getStrategyAllocationsCall),
        #[allow(missing_docs)]
        initialize(initializeCall),
        #[allow(missing_docs)]
        isMemberOfOperatorSet(isMemberOfOperatorSetCall),
        #[allow(missing_docs)]
        isOperatorSet(isOperatorSetCall),
        #[allow(missing_docs)]
        isOperatorSlashable(isOperatorSlashableCall),
        #[allow(missing_docs)]
        modifyAllocations(modifyAllocationsCall),
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
        permissionController(permissionControllerCall),
        #[allow(missing_docs)]
        registerForOperatorSets(registerForOperatorSetsCall),
        #[allow(missing_docs)]
        removeStrategiesFromOperatorSet(removeStrategiesFromOperatorSetCall),
        #[allow(missing_docs)]
        renounceOwnership(renounceOwnershipCall),
        #[allow(missing_docs)]
        setAVSRegistrar(setAVSRegistrarCall),
        #[allow(missing_docs)]
        setAllocationDelay(setAllocationDelayCall),
        #[allow(missing_docs)]
        slashOperator(slashOperatorCall),
        #[allow(missing_docs)]
        transferOwnership(transferOwnershipCall),
        #[allow(missing_docs)]
        unpause(unpauseCall),
        #[allow(missing_docs)]
        updateAVSMetadataURI(updateAVSMetadataURICall),
        #[allow(missing_docs)]
        version(versionCall),
    }
    #[automatically_derived]
    impl AllocationManagerCalls {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [16u8, 225u8, 185u8, 184u8],
            [19u8, 82u8, 195u8, 230u8],
            [19u8, 100u8, 57u8, 221u8],
            [21u8, 254u8, 80u8, 40u8],
            [38u8, 13u8, 199u8, 88u8],
            [38u8, 31u8, 132u8, 224u8],
            [41u8, 129u8, 235u8, 119u8],
            [43u8, 69u8, 58u8, 154u8],
            [43u8, 171u8, 44u8, 74u8],
            [48u8, 76u8, 16u8, 205u8],
            [54u8, 53u8, 32u8, 87u8],
            [64u8, 18u8, 13u8, 171u8],
            [65u8, 119u8, 168u8, 124u8],
            [70u8, 87u8, 226u8, 106u8],
            [74u8, 16u8, 255u8, 229u8],
            [75u8, 80u8, 70u8, 239u8],
            [80u8, 254u8, 234u8, 32u8],
            [84u8, 122u8, 251u8, 135u8],
            [84u8, 253u8, 77u8, 80u8],
            [86u8, 196u8, 131u8, 230u8],
            [89u8, 92u8, 106u8, 103u8],
            [90u8, 200u8, 106u8, 183u8],
            [92u8, 151u8, 90u8, 187u8],
            [103u8, 13u8, 59u8, 162u8],
            [108u8, 251u8, 68u8, 129u8],
            [110u8, 52u8, 146u8, 181u8],
            [110u8, 135u8, 93u8, 186u8],
            [113u8, 80u8, 24u8, 166u8],
            [121u8, 174u8, 80u8, 205u8],
            [123u8, 193u8, 239u8, 97u8],
            [136u8, 111u8, 17u8, 149u8],
            [140u8, 230u8, 72u8, 84u8],
            [141u8, 165u8, 203u8, 91u8],
            [148u8, 215u8, 208u8, 12u8],
            [149u8, 40u8, 153u8, 238u8],
            [169u8, 51u8, 62u8, 200u8],
            [169u8, 130u8, 24u8, 33u8],
            [173u8, 194u8, 227u8, 217u8],
            [178u8, 68u8, 122u8, 247u8],
            [182u8, 107u8, 217u8, 137u8],
            [185u8, 251u8, 174u8, 209u8],
            [186u8, 26u8, 132u8, 229u8],
            [194u8, 33u8, 216u8, 174u8],
            [205u8, 109u8, 198u8, 135u8],
            [211u8, 217u8, 111u8, 244u8],
            [223u8, 92u8, 247u8, 35u8],
            [242u8, 253u8, 227u8, 139u8],
            [246u8, 5u8, 206u8, 8u8],
            [250u8, 188u8, 28u8, 188u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for AllocationManagerCalls {
        const NAME: &'static str = "AllocationManagerCalls";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 49usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::ALLOCATION_CONFIGURATION_DELAY(_) => {
                    <ALLOCATION_CONFIGURATION_DELAYCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::DEALLOCATION_DELAY(_) => {
                    <DEALLOCATION_DELAYCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::addStrategiesToOperatorSet(_) => {
                    <addStrategiesToOperatorSetCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::clearDeallocationQueue(_) => {
                    <clearDeallocationQueueCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::createOperatorSets(_) => {
                    <createOperatorSetsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::delegation(_) => {
                    <delegationCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::deregisterFromOperatorSets(_) => {
                    <deregisterFromOperatorSetsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getAVSRegistrar(_) => {
                    <getAVSRegistrarCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getAllocatableMagnitude(_) => {
                    <getAllocatableMagnitudeCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getAllocatedSets(_) => {
                    <getAllocatedSetsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getAllocatedStake(_) => {
                    <getAllocatedStakeCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getAllocatedStrategies(_) => {
                    <getAllocatedStrategiesCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getAllocation(_) => {
                    <getAllocationCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getAllocationDelay(_) => {
                    <getAllocationDelayCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getAllocations(_) => {
                    <getAllocationsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getEncumberedMagnitude(_) => {
                    <getEncumberedMagnitudeCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getMaxMagnitude(_) => {
                    <getMaxMagnitudeCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getMaxMagnitudes_0(_) => {
                    <getMaxMagnitudes_0Call as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getMaxMagnitudes_1(_) => {
                    <getMaxMagnitudes_1Call as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getMaxMagnitudesAtBlock(_) => {
                    <getMaxMagnitudesAtBlockCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getMemberCount(_) => {
                    <getMemberCountCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getMembers(_) => {
                    <getMembersCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getMinimumSlashableStake(_) => {
                    <getMinimumSlashableStakeCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getOperatorSetCount(_) => {
                    <getOperatorSetCountCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getRegisteredSets(_) => {
                    <getRegisteredSetsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getStrategiesInOperatorSet(_) => {
                    <getStrategiesInOperatorSetCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getStrategyAllocations(_) => {
                    <getStrategyAllocationsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::initialize(_) => {
                    <initializeCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::isMemberOfOperatorSet(_) => {
                    <isMemberOfOperatorSetCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::isOperatorSet(_) => {
                    <isOperatorSetCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::isOperatorSlashable(_) => {
                    <isOperatorSlashableCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::modifyAllocations(_) => {
                    <modifyAllocationsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::owner(_) => <ownerCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::pause(_) => <pauseCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::pauseAll(_) => <pauseAllCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::paused_0(_) => <paused_0Call as alloy_sol_types::SolCall>::SELECTOR,
                Self::paused_1(_) => <paused_1Call as alloy_sol_types::SolCall>::SELECTOR,
                Self::pauserRegistry(_) => {
                    <pauserRegistryCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::permissionController(_) => {
                    <permissionControllerCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::registerForOperatorSets(_) => {
                    <registerForOperatorSetsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::removeStrategiesFromOperatorSet(_) => {
                    <removeStrategiesFromOperatorSetCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::renounceOwnership(_) => {
                    <renounceOwnershipCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::setAVSRegistrar(_) => {
                    <setAVSRegistrarCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::setAllocationDelay(_) => {
                    <setAllocationDelayCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::slashOperator(_) => {
                    <slashOperatorCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::transferOwnership(_) => {
                    <transferOwnershipCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::unpause(_) => <unpauseCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::updateAVSMetadataURI(_) => {
                    <updateAVSMetadataURICall as alloy_sol_types::SolCall>::SELECTOR
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
            ) -> alloy_sol_types::Result<AllocationManagerCalls>] = &[
                {
                    fn getAllocation(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AllocationManagerCalls> {
                        <getAllocationCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AllocationManagerCalls::getAllocation)
                    }
                    getAllocation
                },
                {
                    fn isOperatorSlashable(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AllocationManagerCalls> {
                        <isOperatorSlashableCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AllocationManagerCalls::isOperatorSlashable)
                    }
                    isOperatorSlashable
                },
                {
                    fn pause(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AllocationManagerCalls> {
                        <pauseCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AllocationManagerCalls::pause)
                    }
                    pause
                },
                {
                    fn getAllocatedSets(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AllocationManagerCalls> {
                        <getAllocatedSetsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AllocationManagerCalls::getAllocatedSets)
                    }
                    getAllocatedSets
                },
                {
                    fn isOperatorSet(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AllocationManagerCalls> {
                        <isOperatorSetCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AllocationManagerCalls::isOperatorSet)
                    }
                    isOperatorSet
                },
                {
                    fn createOperatorSets(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AllocationManagerCalls> {
                        <createOperatorSetsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AllocationManagerCalls::createOperatorSets)
                    }
                    createOperatorSets
                },
                {
                    fn DEALLOCATION_DELAY(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AllocationManagerCalls> {
                        <DEALLOCATION_DELAYCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AllocationManagerCalls::DEALLOCATION_DELAY)
                    }
                    DEALLOCATION_DELAY
                },
                {
                    fn getAllocatedStake(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AllocationManagerCalls> {
                        <getAllocatedStakeCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AllocationManagerCalls::getAllocatedStake)
                    }
                    getAllocatedStake
                },
                {
                    fn getMinimumSlashableStake(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AllocationManagerCalls> {
                        <getMinimumSlashableStakeCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AllocationManagerCalls::getMinimumSlashableStake)
                    }
                    getMinimumSlashableStake
                },
                {
                    fn getAVSRegistrar(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AllocationManagerCalls> {
                        <getAVSRegistrarCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AllocationManagerCalls::getAVSRegistrar)
                    }
                    getAVSRegistrar
                },
                {
                    fn slashOperator(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AllocationManagerCalls> {
                        <slashOperatorCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AllocationManagerCalls::slashOperator)
                    }
                    slashOperator
                },
                {
                    fn getStrategyAllocations(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AllocationManagerCalls> {
                        <getStrategyAllocationsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AllocationManagerCalls::getStrategyAllocations)
                    }
                    getStrategyAllocations
                },
                {
                    fn getStrategiesInOperatorSet(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AllocationManagerCalls> {
                        <getStrategiesInOperatorSetCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AllocationManagerCalls::getStrategiesInOperatorSet)
                    }
                    getStrategiesInOperatorSet
                },
                {
                    fn permissionController(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AllocationManagerCalls> {
                        <permissionControllerCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AllocationManagerCalls::permissionController)
                    }
                    permissionController
                },
                {
                    fn getMaxMagnitudes_0(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AllocationManagerCalls> {
                        <getMaxMagnitudes_0Call as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AllocationManagerCalls::getMaxMagnitudes_0)
                    }
                    getMaxMagnitudes_0
                },
                {
                    fn clearDeallocationQueue(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AllocationManagerCalls> {
                        <clearDeallocationQueueCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AllocationManagerCalls::clearDeallocationQueue)
                    }
                    clearDeallocationQueue
                },
                {
                    fn addStrategiesToOperatorSet(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AllocationManagerCalls> {
                        <addStrategiesToOperatorSetCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AllocationManagerCalls::addStrategiesToOperatorSet)
                    }
                    addStrategiesToOperatorSet
                },
                {
                    fn getMaxMagnitudes_1(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AllocationManagerCalls> {
                        <getMaxMagnitudes_1Call as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AllocationManagerCalls::getMaxMagnitudes_1)
                    }
                    getMaxMagnitudes_1
                },
                {
                    fn version(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AllocationManagerCalls> {
                        <versionCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AllocationManagerCalls::version)
                    }
                    version
                },
                {
                    fn setAllocationDelay(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AllocationManagerCalls> {
                        <setAllocationDelayCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AllocationManagerCalls::setAllocationDelay)
                    }
                    setAllocationDelay
                },
                {
                    fn pauseAll(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AllocationManagerCalls> {
                        <pauseAllCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AllocationManagerCalls::pauseAll)
                    }
                    pauseAll
                },
                {
                    fn paused_0(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AllocationManagerCalls> {
                        <paused_0Call as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AllocationManagerCalls::paused_0)
                    }
                    paused_0
                },
                {
                    fn paused_1(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AllocationManagerCalls> {
                        <paused_1Call as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AllocationManagerCalls::paused_1)
                    }
                    paused_1
                },
                {
                    fn isMemberOfOperatorSet(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AllocationManagerCalls> {
                        <isMemberOfOperatorSetCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AllocationManagerCalls::isMemberOfOperatorSet)
                    }
                    isMemberOfOperatorSet
                },
                {
                    fn getAllocatableMagnitude(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AllocationManagerCalls> {
                        <getAllocatableMagnitudeCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AllocationManagerCalls::getAllocatableMagnitude)
                    }
                    getAllocatableMagnitude
                },
                {
                    fn deregisterFromOperatorSets(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AllocationManagerCalls> {
                        <deregisterFromOperatorSetsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AllocationManagerCalls::deregisterFromOperatorSets)
                    }
                    deregisterFromOperatorSets
                },
                {
                    fn getMembers(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AllocationManagerCalls> {
                        <getMembersCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AllocationManagerCalls::getMembers)
                    }
                    getMembers
                },
                {
                    fn renounceOwnership(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AllocationManagerCalls> {
                        <renounceOwnershipCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AllocationManagerCalls::renounceOwnership)
                    }
                    renounceOwnership
                },
                {
                    fn getRegisteredSets(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AllocationManagerCalls> {
                        <getRegisteredSetsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AllocationManagerCalls::getRegisteredSets)
                    }
                    getRegisteredSets
                },
                {
                    fn ALLOCATION_CONFIGURATION_DELAY(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AllocationManagerCalls> {
                        <ALLOCATION_CONFIGURATION_DELAYCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AllocationManagerCalls::ALLOCATION_CONFIGURATION_DELAY)
                    }
                    ALLOCATION_CONFIGURATION_DELAY
                },
                {
                    fn pauserRegistry(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AllocationManagerCalls> {
                        <pauserRegistryCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AllocationManagerCalls::pauserRegistry)
                    }
                    pauserRegistry
                },
                {
                    fn getAllocations(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AllocationManagerCalls> {
                        <getAllocationsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AllocationManagerCalls::getAllocations)
                    }
                    getAllocations
                },
                {
                    fn owner(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AllocationManagerCalls> {
                        <ownerCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AllocationManagerCalls::owner)
                    }
                    owner
                },
                {
                    fn getMaxMagnitudesAtBlock(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AllocationManagerCalls> {
                        <getMaxMagnitudesAtBlockCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AllocationManagerCalls::getMaxMagnitudesAtBlock)
                    }
                    getMaxMagnitudesAtBlock
                },
                {
                    fn modifyAllocations(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AllocationManagerCalls> {
                        <modifyAllocationsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AllocationManagerCalls::modifyAllocations)
                    }
                    modifyAllocations
                },
                {
                    fn getMaxMagnitude(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AllocationManagerCalls> {
                        <getMaxMagnitudeCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AllocationManagerCalls::getMaxMagnitude)
                    }
                    getMaxMagnitude
                },
                {
                    fn updateAVSMetadataURI(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AllocationManagerCalls> {
                        <updateAVSMetadataURICall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AllocationManagerCalls::updateAVSMetadataURI)
                    }
                    updateAVSMetadataURI
                },
                {
                    fn registerForOperatorSets(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AllocationManagerCalls> {
                        <registerForOperatorSetsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AllocationManagerCalls::registerForOperatorSets)
                    }
                    registerForOperatorSets
                },
                {
                    fn getMemberCount(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AllocationManagerCalls> {
                        <getMemberCountCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AllocationManagerCalls::getMemberCount)
                    }
                    getMemberCount
                },
                {
                    fn removeStrategiesFromOperatorSet(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AllocationManagerCalls> {
                        <removeStrategiesFromOperatorSetCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AllocationManagerCalls::removeStrategiesFromOperatorSet)
                    }
                    removeStrategiesFromOperatorSet
                },
                {
                    fn getAllocationDelay(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AllocationManagerCalls> {
                        <getAllocationDelayCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AllocationManagerCalls::getAllocationDelay)
                    }
                    getAllocationDelay
                },
                {
                    fn getOperatorSetCount(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AllocationManagerCalls> {
                        <getOperatorSetCountCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AllocationManagerCalls::getOperatorSetCount)
                    }
                    getOperatorSetCount
                },
                {
                    fn getAllocatedStrategies(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AllocationManagerCalls> {
                        <getAllocatedStrategiesCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AllocationManagerCalls::getAllocatedStrategies)
                    }
                    getAllocatedStrategies
                },
                {
                    fn initialize(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AllocationManagerCalls> {
                        <initializeCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AllocationManagerCalls::initialize)
                    }
                    initialize
                },
                {
                    fn setAVSRegistrar(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AllocationManagerCalls> {
                        <setAVSRegistrarCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AllocationManagerCalls::setAVSRegistrar)
                    }
                    setAVSRegistrar
                },
                {
                    fn delegation(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AllocationManagerCalls> {
                        <delegationCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AllocationManagerCalls::delegation)
                    }
                    delegation
                },
                {
                    fn transferOwnership(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AllocationManagerCalls> {
                        <transferOwnershipCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AllocationManagerCalls::transferOwnership)
                    }
                    transferOwnership
                },
                {
                    fn getEncumberedMagnitude(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AllocationManagerCalls> {
                        <getEncumberedMagnitudeCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AllocationManagerCalls::getEncumberedMagnitude)
                    }
                    getEncumberedMagnitude
                },
                {
                    fn unpause(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AllocationManagerCalls> {
                        <unpauseCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AllocationManagerCalls::unpause)
                    }
                    unpause
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
                Self::ALLOCATION_CONFIGURATION_DELAY(inner) => {
                    <ALLOCATION_CONFIGURATION_DELAYCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::DEALLOCATION_DELAY(inner) => {
                    <DEALLOCATION_DELAYCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::addStrategiesToOperatorSet(inner) => {
                    <addStrategiesToOperatorSetCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::clearDeallocationQueue(inner) => {
                    <clearDeallocationQueueCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::createOperatorSets(inner) => {
                    <createOperatorSetsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::delegation(inner) => {
                    <delegationCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::deregisterFromOperatorSets(inner) => {
                    <deregisterFromOperatorSetsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getAVSRegistrar(inner) => {
                    <getAVSRegistrarCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getAllocatableMagnitude(inner) => {
                    <getAllocatableMagnitudeCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getAllocatedSets(inner) => {
                    <getAllocatedSetsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getAllocatedStake(inner) => {
                    <getAllocatedStakeCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getAllocatedStrategies(inner) => {
                    <getAllocatedStrategiesCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getAllocation(inner) => {
                    <getAllocationCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getAllocationDelay(inner) => {
                    <getAllocationDelayCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getAllocations(inner) => {
                    <getAllocationsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getEncumberedMagnitude(inner) => {
                    <getEncumberedMagnitudeCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getMaxMagnitude(inner) => {
                    <getMaxMagnitudeCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getMaxMagnitudes_0(inner) => {
                    <getMaxMagnitudes_0Call as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getMaxMagnitudes_1(inner) => {
                    <getMaxMagnitudes_1Call as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getMaxMagnitudesAtBlock(inner) => {
                    <getMaxMagnitudesAtBlockCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getMemberCount(inner) => {
                    <getMemberCountCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getMembers(inner) => {
                    <getMembersCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::getMinimumSlashableStake(inner) => {
                    <getMinimumSlashableStakeCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getOperatorSetCount(inner) => {
                    <getOperatorSetCountCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getRegisteredSets(inner) => {
                    <getRegisteredSetsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getStrategiesInOperatorSet(inner) => {
                    <getStrategiesInOperatorSetCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getStrategyAllocations(inner) => {
                    <getStrategyAllocationsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::initialize(inner) => {
                    <initializeCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::isMemberOfOperatorSet(inner) => {
                    <isMemberOfOperatorSetCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::isOperatorSet(inner) => {
                    <isOperatorSetCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::isOperatorSlashable(inner) => {
                    <isOperatorSlashableCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::modifyAllocations(inner) => {
                    <modifyAllocationsCall as alloy_sol_types::SolCall>::abi_encoded_size(
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
                Self::permissionController(inner) => {
                    <permissionControllerCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::registerForOperatorSets(inner) => {
                    <registerForOperatorSetsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::removeStrategiesFromOperatorSet(inner) => {
                    <removeStrategiesFromOperatorSetCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::renounceOwnership(inner) => {
                    <renounceOwnershipCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::setAVSRegistrar(inner) => {
                    <setAVSRegistrarCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::setAllocationDelay(inner) => {
                    <setAllocationDelayCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::slashOperator(inner) => {
                    <slashOperatorCall as alloy_sol_types::SolCall>::abi_encoded_size(
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
                Self::updateAVSMetadataURI(inner) => {
                    <updateAVSMetadataURICall as alloy_sol_types::SolCall>::abi_encoded_size(
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
                Self::ALLOCATION_CONFIGURATION_DELAY(inner) => {
                    <ALLOCATION_CONFIGURATION_DELAYCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::DEALLOCATION_DELAY(inner) => {
                    <DEALLOCATION_DELAYCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::addStrategiesToOperatorSet(inner) => {
                    <addStrategiesToOperatorSetCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::clearDeallocationQueue(inner) => {
                    <clearDeallocationQueueCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::createOperatorSets(inner) => {
                    <createOperatorSetsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::delegation(inner) => {
                    <delegationCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::deregisterFromOperatorSets(inner) => {
                    <deregisterFromOperatorSetsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getAVSRegistrar(inner) => {
                    <getAVSRegistrarCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getAllocatableMagnitude(inner) => {
                    <getAllocatableMagnitudeCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getAllocatedSets(inner) => {
                    <getAllocatedSetsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getAllocatedStake(inner) => {
                    <getAllocatedStakeCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getAllocatedStrategies(inner) => {
                    <getAllocatedStrategiesCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getAllocation(inner) => {
                    <getAllocationCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getAllocationDelay(inner) => {
                    <getAllocationDelayCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getAllocations(inner) => {
                    <getAllocationsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getEncumberedMagnitude(inner) => {
                    <getEncumberedMagnitudeCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getMaxMagnitude(inner) => {
                    <getMaxMagnitudeCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getMaxMagnitudes_0(inner) => {
                    <getMaxMagnitudes_0Call as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getMaxMagnitudes_1(inner) => {
                    <getMaxMagnitudes_1Call as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getMaxMagnitudesAtBlock(inner) => {
                    <getMaxMagnitudesAtBlockCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getMemberCount(inner) => {
                    <getMemberCountCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getMembers(inner) => {
                    <getMembersCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getMinimumSlashableStake(inner) => {
                    <getMinimumSlashableStakeCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getOperatorSetCount(inner) => {
                    <getOperatorSetCountCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getRegisteredSets(inner) => {
                    <getRegisteredSetsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getStrategiesInOperatorSet(inner) => {
                    <getStrategiesInOperatorSetCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getStrategyAllocations(inner) => {
                    <getStrategyAllocationsCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::isMemberOfOperatorSet(inner) => {
                    <isMemberOfOperatorSetCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::isOperatorSet(inner) => {
                    <isOperatorSetCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::isOperatorSlashable(inner) => {
                    <isOperatorSlashableCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::modifyAllocations(inner) => {
                    <modifyAllocationsCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::permissionController(inner) => {
                    <permissionControllerCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::registerForOperatorSets(inner) => {
                    <registerForOperatorSetsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::removeStrategiesFromOperatorSet(inner) => {
                    <removeStrategiesFromOperatorSetCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::setAVSRegistrar(inner) => {
                    <setAVSRegistrarCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::setAllocationDelay(inner) => {
                    <setAllocationDelayCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::slashOperator(inner) => {
                    <slashOperatorCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::updateAVSMetadataURI(inner) => {
                    <updateAVSMetadataURICall as alloy_sol_types::SolCall>::abi_encode_raw(
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
    ///Container for all the [`AllocationManager`](self) custom errors.
    pub enum AllocationManagerErrors {
        #[allow(missing_docs)]
        AlreadyMemberOfSet(AlreadyMemberOfSet),
        #[allow(missing_docs)]
        CurrentlyPaused(CurrentlyPaused),
        #[allow(missing_docs)]
        Empty(Empty),
        #[allow(missing_docs)]
        InputAddressZero(InputAddressZero),
        #[allow(missing_docs)]
        InputArrayLengthMismatch(InputArrayLengthMismatch),
        #[allow(missing_docs)]
        InsufficientMagnitude(InsufficientMagnitude),
        #[allow(missing_docs)]
        InvalidAVSRegistrar(InvalidAVSRegistrar),
        #[allow(missing_docs)]
        InvalidCaller(InvalidCaller),
        #[allow(missing_docs)]
        InvalidNewPausedStatus(InvalidNewPausedStatus),
        #[allow(missing_docs)]
        InvalidOperator(InvalidOperator),
        #[allow(missing_docs)]
        InvalidOperatorSet(InvalidOperatorSet),
        #[allow(missing_docs)]
        InvalidPermissions(InvalidPermissions),
        #[allow(missing_docs)]
        InvalidShortString(InvalidShortString),
        #[allow(missing_docs)]
        InvalidSnapshotOrdering(InvalidSnapshotOrdering),
        #[allow(missing_docs)]
        InvalidWadToSlash(InvalidWadToSlash),
        #[allow(missing_docs)]
        ModificationAlreadyPending(ModificationAlreadyPending),
        #[allow(missing_docs)]
        NonexistentAVSMetadata(NonexistentAVSMetadata),
        #[allow(missing_docs)]
        NotMemberOfSet(NotMemberOfSet),
        #[allow(missing_docs)]
        OnlyPauser(OnlyPauser),
        #[allow(missing_docs)]
        OnlyUnpauser(OnlyUnpauser),
        #[allow(missing_docs)]
        OperatorNotSlashable(OperatorNotSlashable),
        #[allow(missing_docs)]
        OutOfBounds(OutOfBounds),
        #[allow(missing_docs)]
        SameMagnitude(SameMagnitude),
        #[allow(missing_docs)]
        StrategiesMustBeInAscendingOrder(StrategiesMustBeInAscendingOrder),
        #[allow(missing_docs)]
        StrategyAlreadyInOperatorSet(StrategyAlreadyInOperatorSet),
        #[allow(missing_docs)]
        StrategyNotInOperatorSet(StrategyNotInOperatorSet),
        #[allow(missing_docs)]
        StringTooLong(StringTooLong),
        #[allow(missing_docs)]
        UninitializedAllocationDelay(UninitializedAllocationDelay),
    }
    #[automatically_derived]
    impl AllocationManagerErrors {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [19u8, 83u8, 96u8, 49u8],
            [37u8, 19u8, 29u8, 79u8],
            [42u8, 55u8, 28u8, 126u8],
            [48u8, 90u8, 39u8, 169u8],
            [61u8, 178u8, 161u8, 42u8],
            [67u8, 113u8, 74u8, 253u8],
            [72u8, 245u8, 195u8, 237u8],
            [72u8, 247u8, 219u8, 185u8],
            [88u8, 92u8, 251u8, 47u8],
            [99u8, 120u8, 104u8, 78u8],
            [108u8, 155u8, 224u8, 191u8],
            [115u8, 99u8, 33u8, 118u8],
            [117u8, 223u8, 81u8, 220u8],
            [121u8, 72u8, 33u8, 255u8],
            [126u8, 197u8, 193u8, 84u8],
            [132u8, 10u8, 72u8, 213u8],
            [140u8, 12u8, 47u8, 38u8],
            [147u8, 45u8, 148u8, 247u8],
            [159u8, 28u8, 128u8, 83u8],
            [179u8, 81u8, 43u8, 12u8],
            [180u8, 18u8, 15u8, 20u8],
            [198u8, 29u8, 202u8, 93u8],
            [204u8, 234u8, 158u8, 111u8],
            [216u8, 216u8, 220u8, 78u8],
            [216u8, 252u8, 190u8, 48u8],
            [232u8, 88u8, 158u8, 8u8],
            [235u8, 191u8, 244u8, 151u8],
            [250u8, 85u8, 252u8, 129u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for AllocationManagerErrors {
        const NAME: &'static str = "AllocationManagerErrors";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 28usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::AlreadyMemberOfSet(_) => {
                    <AlreadyMemberOfSet as alloy_sol_types::SolError>::SELECTOR
                }
                Self::CurrentlyPaused(_) => {
                    <CurrentlyPaused as alloy_sol_types::SolError>::SELECTOR
                }
                Self::Empty(_) => <Empty as alloy_sol_types::SolError>::SELECTOR,
                Self::InputAddressZero(_) => {
                    <InputAddressZero as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InputArrayLengthMismatch(_) => {
                    <InputArrayLengthMismatch as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InsufficientMagnitude(_) => {
                    <InsufficientMagnitude as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidAVSRegistrar(_) => {
                    <InvalidAVSRegistrar as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidCaller(_) => {
                    <InvalidCaller as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidNewPausedStatus(_) => {
                    <InvalidNewPausedStatus as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidOperator(_) => {
                    <InvalidOperator as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidOperatorSet(_) => {
                    <InvalidOperatorSet as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidPermissions(_) => {
                    <InvalidPermissions as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidShortString(_) => {
                    <InvalidShortString as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidSnapshotOrdering(_) => {
                    <InvalidSnapshotOrdering as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidWadToSlash(_) => {
                    <InvalidWadToSlash as alloy_sol_types::SolError>::SELECTOR
                }
                Self::ModificationAlreadyPending(_) => {
                    <ModificationAlreadyPending as alloy_sol_types::SolError>::SELECTOR
                }
                Self::NonexistentAVSMetadata(_) => {
                    <NonexistentAVSMetadata as alloy_sol_types::SolError>::SELECTOR
                }
                Self::NotMemberOfSet(_) => {
                    <NotMemberOfSet as alloy_sol_types::SolError>::SELECTOR
                }
                Self::OnlyPauser(_) => {
                    <OnlyPauser as alloy_sol_types::SolError>::SELECTOR
                }
                Self::OnlyUnpauser(_) => {
                    <OnlyUnpauser as alloy_sol_types::SolError>::SELECTOR
                }
                Self::OperatorNotSlashable(_) => {
                    <OperatorNotSlashable as alloy_sol_types::SolError>::SELECTOR
                }
                Self::OutOfBounds(_) => {
                    <OutOfBounds as alloy_sol_types::SolError>::SELECTOR
                }
                Self::SameMagnitude(_) => {
                    <SameMagnitude as alloy_sol_types::SolError>::SELECTOR
                }
                Self::StrategiesMustBeInAscendingOrder(_) => {
                    <StrategiesMustBeInAscendingOrder as alloy_sol_types::SolError>::SELECTOR
                }
                Self::StrategyAlreadyInOperatorSet(_) => {
                    <StrategyAlreadyInOperatorSet as alloy_sol_types::SolError>::SELECTOR
                }
                Self::StrategyNotInOperatorSet(_) => {
                    <StrategyNotInOperatorSet as alloy_sol_types::SolError>::SELECTOR
                }
                Self::StringTooLong(_) => {
                    <StringTooLong as alloy_sol_types::SolError>::SELECTOR
                }
                Self::UninitializedAllocationDelay(_) => {
                    <UninitializedAllocationDelay as alloy_sol_types::SolError>::SELECTOR
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
            ) -> alloy_sol_types::Result<AllocationManagerErrors>] = &[
                {
                    fn InvalidWadToSlash(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AllocationManagerErrors> {
                        <InvalidWadToSlash as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AllocationManagerErrors::InvalidWadToSlash)
                    }
                    InvalidWadToSlash
                },
                {
                    fn NotMemberOfSet(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AllocationManagerErrors> {
                        <NotMemberOfSet as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AllocationManagerErrors::NotMemberOfSet)
                    }
                    NotMemberOfSet
                },
                {
                    fn InvalidSnapshotOrdering(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AllocationManagerErrors> {
                        <InvalidSnapshotOrdering as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AllocationManagerErrors::InvalidSnapshotOrdering)
                    }
                    InvalidSnapshotOrdering
                },
                {
                    fn StringTooLong(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AllocationManagerErrors> {
                        <StringTooLong as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AllocationManagerErrors::StringTooLong)
                    }
                    StringTooLong
                },
                {
                    fn Empty(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AllocationManagerErrors> {
                        <Empty as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AllocationManagerErrors::Empty)
                    }
                    Empty
                },
                {
                    fn InputArrayLengthMismatch(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AllocationManagerErrors> {
                        <InputArrayLengthMismatch as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AllocationManagerErrors::InputArrayLengthMismatch)
                    }
                    InputArrayLengthMismatch
                },
                {
                    fn InvalidCaller(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AllocationManagerErrors> {
                        <InvalidCaller as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AllocationManagerErrors::InvalidCaller)
                    }
                    InvalidCaller
                },
                {
                    fn NonexistentAVSMetadata(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AllocationManagerErrors> {
                        <NonexistentAVSMetadata as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AllocationManagerErrors::NonexistentAVSMetadata)
                    }
                    NonexistentAVSMetadata
                },
                {
                    fn StrategyAlreadyInOperatorSet(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AllocationManagerErrors> {
                        <StrategyAlreadyInOperatorSet as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AllocationManagerErrors::StrategyAlreadyInOperatorSet)
                    }
                    StrategyAlreadyInOperatorSet
                },
                {
                    fn StrategyNotInOperatorSet(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AllocationManagerErrors> {
                        <StrategyNotInOperatorSet as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AllocationManagerErrors::StrategyNotInOperatorSet)
                    }
                    StrategyNotInOperatorSet
                },
                {
                    fn InsufficientMagnitude(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AllocationManagerErrors> {
                        <InsufficientMagnitude as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AllocationManagerErrors::InsufficientMagnitude)
                    }
                    InsufficientMagnitude
                },
                {
                    fn InputAddressZero(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AllocationManagerErrors> {
                        <InputAddressZero as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AllocationManagerErrors::InputAddressZero)
                    }
                    InputAddressZero
                },
                {
                    fn OnlyPauser(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AllocationManagerErrors> {
                        <OnlyPauser as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AllocationManagerErrors::OnlyPauser)
                    }
                    OnlyPauser
                },
                {
                    fn OnlyUnpauser(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AllocationManagerErrors> {
                        <OnlyUnpauser as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AllocationManagerErrors::OnlyUnpauser)
                    }
                    OnlyUnpauser
                },
                {
                    fn InvalidOperatorSet(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AllocationManagerErrors> {
                        <InvalidOperatorSet as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AllocationManagerErrors::InvalidOperatorSet)
                    }
                    InvalidOperatorSet
                },
                {
                    fn CurrentlyPaused(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AllocationManagerErrors> {
                        <CurrentlyPaused as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AllocationManagerErrors::CurrentlyPaused)
                    }
                    CurrentlyPaused
                },
                {
                    fn SameMagnitude(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AllocationManagerErrors> {
                        <SameMagnitude as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AllocationManagerErrors::SameMagnitude)
                    }
                    SameMagnitude
                },
                {
                    fn InvalidPermissions(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AllocationManagerErrors> {
                        <InvalidPermissions as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AllocationManagerErrors::InvalidPermissions)
                    }
                    InvalidPermissions
                },
                {
                    fn StrategiesMustBeInAscendingOrder(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AllocationManagerErrors> {
                        <StrategiesMustBeInAscendingOrder as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                AllocationManagerErrors::StrategiesMustBeInAscendingOrder,
                            )
                    }
                    StrategiesMustBeInAscendingOrder
                },
                {
                    fn InvalidShortString(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AllocationManagerErrors> {
                        <InvalidShortString as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AllocationManagerErrors::InvalidShortString)
                    }
                    InvalidShortString
                },
                {
                    fn OutOfBounds(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AllocationManagerErrors> {
                        <OutOfBounds as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AllocationManagerErrors::OutOfBounds)
                    }
                    OutOfBounds
                },
                {
                    fn InvalidNewPausedStatus(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AllocationManagerErrors> {
                        <InvalidNewPausedStatus as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AllocationManagerErrors::InvalidNewPausedStatus)
                    }
                    InvalidNewPausedStatus
                },
                {
                    fn InvalidOperator(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AllocationManagerErrors> {
                        <InvalidOperator as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AllocationManagerErrors::InvalidOperator)
                    }
                    InvalidOperator
                },
                {
                    fn AlreadyMemberOfSet(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AllocationManagerErrors> {
                        <AlreadyMemberOfSet as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AllocationManagerErrors::AlreadyMemberOfSet)
                    }
                    AlreadyMemberOfSet
                },
                {
                    fn ModificationAlreadyPending(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AllocationManagerErrors> {
                        <ModificationAlreadyPending as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AllocationManagerErrors::ModificationAlreadyPending)
                    }
                    ModificationAlreadyPending
                },
                {
                    fn InvalidAVSRegistrar(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AllocationManagerErrors> {
                        <InvalidAVSRegistrar as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AllocationManagerErrors::InvalidAVSRegistrar)
                    }
                    InvalidAVSRegistrar
                },
                {
                    fn OperatorNotSlashable(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AllocationManagerErrors> {
                        <OperatorNotSlashable as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AllocationManagerErrors::OperatorNotSlashable)
                    }
                    OperatorNotSlashable
                },
                {
                    fn UninitializedAllocationDelay(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AllocationManagerErrors> {
                        <UninitializedAllocationDelay as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AllocationManagerErrors::UninitializedAllocationDelay)
                    }
                    UninitializedAllocationDelay
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
                Self::AlreadyMemberOfSet(inner) => {
                    <AlreadyMemberOfSet as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::CurrentlyPaused(inner) => {
                    <CurrentlyPaused as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::Empty(inner) => {
                    <Empty as alloy_sol_types::SolError>::abi_encoded_size(inner)
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
                Self::InsufficientMagnitude(inner) => {
                    <InsufficientMagnitude as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InvalidAVSRegistrar(inner) => {
                    <InvalidAVSRegistrar as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InvalidCaller(inner) => {
                    <InvalidCaller as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::InvalidNewPausedStatus(inner) => {
                    <InvalidNewPausedStatus as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InvalidOperator(inner) => {
                    <InvalidOperator as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InvalidOperatorSet(inner) => {
                    <InvalidOperatorSet as alloy_sol_types::SolError>::abi_encoded_size(
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
                Self::InvalidSnapshotOrdering(inner) => {
                    <InvalidSnapshotOrdering as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InvalidWadToSlash(inner) => {
                    <InvalidWadToSlash as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::ModificationAlreadyPending(inner) => {
                    <ModificationAlreadyPending as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::NonexistentAVSMetadata(inner) => {
                    <NonexistentAVSMetadata as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::NotMemberOfSet(inner) => {
                    <NotMemberOfSet as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::OnlyPauser(inner) => {
                    <OnlyPauser as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::OnlyUnpauser(inner) => {
                    <OnlyUnpauser as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::OperatorNotSlashable(inner) => {
                    <OperatorNotSlashable as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::OutOfBounds(inner) => {
                    <OutOfBounds as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::SameMagnitude(inner) => {
                    <SameMagnitude as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::StrategiesMustBeInAscendingOrder(inner) => {
                    <StrategiesMustBeInAscendingOrder as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::StrategyAlreadyInOperatorSet(inner) => {
                    <StrategyAlreadyInOperatorSet as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::StrategyNotInOperatorSet(inner) => {
                    <StrategyNotInOperatorSet as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::StringTooLong(inner) => {
                    <StringTooLong as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::UninitializedAllocationDelay(inner) => {
                    <UninitializedAllocationDelay as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
            }
        }
        #[inline]
        fn abi_encode_raw(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
            match self {
                Self::AlreadyMemberOfSet(inner) => {
                    <AlreadyMemberOfSet as alloy_sol_types::SolError>::abi_encode_raw(
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
                Self::Empty(inner) => {
                    <Empty as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
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
                Self::InsufficientMagnitude(inner) => {
                    <InsufficientMagnitude as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InvalidAVSRegistrar(inner) => {
                    <InvalidAVSRegistrar as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InvalidCaller(inner) => {
                    <InvalidCaller as alloy_sol_types::SolError>::abi_encode_raw(
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
                Self::InvalidOperator(inner) => {
                    <InvalidOperator as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InvalidOperatorSet(inner) => {
                    <InvalidOperatorSet as alloy_sol_types::SolError>::abi_encode_raw(
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
                Self::InvalidSnapshotOrdering(inner) => {
                    <InvalidSnapshotOrdering as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InvalidWadToSlash(inner) => {
                    <InvalidWadToSlash as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::ModificationAlreadyPending(inner) => {
                    <ModificationAlreadyPending as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::NonexistentAVSMetadata(inner) => {
                    <NonexistentAVSMetadata as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::NotMemberOfSet(inner) => {
                    <NotMemberOfSet as alloy_sol_types::SolError>::abi_encode_raw(
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
                Self::OperatorNotSlashable(inner) => {
                    <OperatorNotSlashable as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::OutOfBounds(inner) => {
                    <OutOfBounds as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::SameMagnitude(inner) => {
                    <SameMagnitude as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::StrategiesMustBeInAscendingOrder(inner) => {
                    <StrategiesMustBeInAscendingOrder as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::StrategyAlreadyInOperatorSet(inner) => {
                    <StrategyAlreadyInOperatorSet as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::StrategyNotInOperatorSet(inner) => {
                    <StrategyNotInOperatorSet as alloy_sol_types::SolError>::abi_encode_raw(
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
                Self::UninitializedAllocationDelay(inner) => {
                    <UninitializedAllocationDelay as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
            }
        }
    }
    ///Container for all the [`AllocationManager`](self) events.
    pub enum AllocationManagerEvents {
        #[allow(missing_docs)]
        AVSMetadataURIUpdated(AVSMetadataURIUpdated),
        #[allow(missing_docs)]
        AVSRegistrarSet(AVSRegistrarSet),
        #[allow(missing_docs)]
        AllocationDelaySet(AllocationDelaySet),
        #[allow(missing_docs)]
        AllocationUpdated(AllocationUpdated),
        #[allow(missing_docs)]
        EncumberedMagnitudeUpdated(EncumberedMagnitudeUpdated),
        #[allow(missing_docs)]
        Initialized(Initialized),
        #[allow(missing_docs)]
        MaxMagnitudeUpdated(MaxMagnitudeUpdated),
        #[allow(missing_docs)]
        OperatorAddedToOperatorSet(OperatorAddedToOperatorSet),
        #[allow(missing_docs)]
        OperatorRemovedFromOperatorSet(OperatorRemovedFromOperatorSet),
        #[allow(missing_docs)]
        OperatorSetCreated(OperatorSetCreated),
        #[allow(missing_docs)]
        OperatorSlashed(OperatorSlashed),
        #[allow(missing_docs)]
        OwnershipTransferred(OwnershipTransferred),
        #[allow(missing_docs)]
        Paused(Paused),
        #[allow(missing_docs)]
        StrategyAddedToOperatorSet(StrategyAddedToOperatorSet),
        #[allow(missing_docs)]
        StrategyRemovedFromOperatorSet(StrategyRemovedFromOperatorSet),
        #[allow(missing_docs)]
        Unpaused(Unpaused),
    }
    #[automatically_derived]
    impl AllocationManagerEvents {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 32usize]] = &[
            [
                20u8,
                135u8,
                175u8,
                84u8,
                24u8,
                196u8,
                126u8,
                229u8,
                234u8,
                69u8,
                239u8,
                74u8,
                147u8,
                57u8,
                134u8,
                104u8,
                18u8,
                8u8,
                144u8,
                119u8,
                74u8,
                158u8,
                19u8,
                72u8,
                126u8,
                97u8,
                233u8,
                220u8,
                59u8,
                175u8,
                118u8,
                221u8,
            ],
            [
                28u8,
                100u8,
                88u8,
                7u8,
                154u8,
                65u8,
                7u8,
                125u8,
                0u8,
                60u8,
                17u8,
                250u8,
                249u8,
                191u8,
                9u8,
                126u8,
                105u8,
                59u8,
                214u8,
                121u8,
                121u8,
                228u8,
                230u8,
                80u8,
                11u8,
                172u8,
                123u8,
                41u8,
                219u8,
                119u8,
                155u8,
                92u8,
            ],
            [
                42u8,
                233u8,
                69u8,
                196u8,
                12u8,
                68u8,
                220u8,
                14u8,
                194u8,
                99u8,
                249u8,
                86u8,
                9u8,
                195u8,
                253u8,
                198u8,
                149u8,
                46u8,
                10u8,
                239u8,
                162u8,
                45u8,
                99u8,
                116u8,
                228u8,
                79u8,
                44u8,
                153u8,
                122u8,
                206u8,
                223u8,
                133u8,
            ],
            [
                49u8,
                98u8,
                146u8,
                133u8,
                234u8,
                210u8,
                51u8,
                90u8,
                224u8,
                147u8,
                63u8,
                134u8,
                237u8,
                42u8,
                230u8,
                51u8,
                33u8,
                247u8,
                175u8,
                119u8,
                180u8,
                230u8,
                234u8,
                171u8,
                196u8,
                44u8,
                5u8,
                120u8,
                128u8,
                151u8,
                126u8,
                108u8,
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
                67u8,
                35u8,
                46u8,
                223u8,
                144u8,
                113u8,
                117u8,
                61u8,
                35u8,
                33u8,
                229u8,
                250u8,
                126u8,
                1u8,
                131u8,
                99u8,
                238u8,
                36u8,
                142u8,
                95u8,
                33u8,
                66u8,
                230u8,
                192u8,
                142u8,
                221u8,
                50u8,
                101u8,
                191u8,
                180u8,
                137u8,
                94u8,
            ],
            [
                78u8,
                133u8,
                117u8,
                29u8,
                99u8,
                49u8,
                80u8,
                108u8,
                108u8,
                98u8,
                51u8,
                95u8,
                32u8,
                126u8,
                179u8,
                31u8,
                18u8,
                166u8,
                30u8,
                87u8,
                15u8,
                52u8,
                245u8,
                193u8,
                118u8,
                64u8,
                48u8,
                135u8,
                133u8,
                198u8,
                212u8,
                219u8,
            ],
            [
                122u8,
                178u8,
                96u8,
                254u8,
                10u8,
                241u8,
                147u8,
                219u8,
                95u8,
                73u8,
                134u8,
                119u8,
                13u8,
                131u8,
                27u8,
                218u8,
                78u8,
                164u8,
                96u8,
                153u8,
                220u8,
                129u8,
                126u8,
                139u8,
                103u8,
                22u8,
                220u8,
                174u8,
                138u8,
                248u8,
                232u8,
                139u8,
            ],
            [
                123u8,
                75u8,
                7u8,
                61u8,
                128u8,
                220u8,
                172u8,
                85u8,
                161u8,
                17u8,
                119u8,
                216u8,
                69u8,
                154u8,
                217u8,
                246u8,
                100u8,
                206u8,
                235u8,
                145u8,
                247u8,
                31u8,
                39u8,
                22u8,
                123u8,
                177u8,
                79u8,
                129u8,
                82u8,
                167u8,
                238u8,
                238u8,
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
                128u8,
                150u8,
                154u8,
                210u8,
                148u8,
                40u8,
                214u8,
                121u8,
                126u8,
                231u8,
                170u8,
                208u8,
                132u8,
                249u8,
                228u8,
                164u8,
                42u8,
                130u8,
                252u8,
                80u8,
                109u8,
                205u8,
                44u8,
                163u8,
                182u8,
                251u8,
                67u8,
                31u8,
                133u8,
                204u8,
                235u8,
                229u8,
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
                168u8,
                156u8,
                29u8,
                194u8,
                67u8,
                216u8,
                144u8,
                138u8,
                150u8,
                221u8,
                132u8,
                148u8,
                75u8,
                204u8,
                151u8,
                214u8,
                188u8,
                106u8,
                192u8,
                13u8,
                215u8,
                142u8,
                32u8,
                98u8,
                21u8,
                118u8,
                190u8,
                106u8,
                60u8,
                148u8,
                55u8,
                19u8,
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
                172u8,
                249u8,
                9u8,
                95u8,
                235u8,
                58u8,
                55u8,
                12u8,
                156u8,
                246u8,
                146u8,
                66u8,
                28u8,
                105u8,
                239u8,
                50u8,
                13u8,
                77u8,
                181u8,
                198u8,
                110u8,
                106u8,
                125u8,
                41u8,
                199u8,
                105u8,
                78u8,
                176u8,
                35u8,
                100u8,
                252u8,
                85u8,
            ],
            [
                173u8,
                52u8,
                195u8,
                7u8,
                11u8,
                225u8,
                223u8,
                251u8,
                202u8,
                164u8,
                153u8,
                208u8,
                0u8,
                186u8,
                43u8,
                141u8,
                152u8,
                72u8,
                174u8,
                252u8,
                172u8,
                48u8,
                89u8,
                223u8,
                36u8,
                93u8,
                217u8,
                92u8,
                78u8,
                206u8,
                20u8,
                254u8,
            ],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolEventInterface for AllocationManagerEvents {
        const NAME: &'static str = "AllocationManagerEvents";
        const COUNT: usize = 16usize;
        fn decode_raw_log(
            topics: &[alloy_sol_types::Word],
            data: &[u8],
            validate: bool,
        ) -> alloy_sol_types::Result<Self> {
            match topics.first().copied() {
                Some(
                    <AVSMetadataURIUpdated as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <AVSMetadataURIUpdated as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::AVSMetadataURIUpdated)
                }
                Some(<AVSRegistrarSet as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <AVSRegistrarSet as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::AVSRegistrarSet)
                }
                Some(
                    <AllocationDelaySet as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <AllocationDelaySet as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::AllocationDelaySet)
                }
                Some(
                    <AllocationUpdated as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <AllocationUpdated as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::AllocationUpdated)
                }
                Some(
                    <EncumberedMagnitudeUpdated as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <EncumberedMagnitudeUpdated as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::EncumberedMagnitudeUpdated)
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
                    <MaxMagnitudeUpdated as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <MaxMagnitudeUpdated as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::MaxMagnitudeUpdated)
                }
                Some(
                    <OperatorAddedToOperatorSet as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <OperatorAddedToOperatorSet as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::OperatorAddedToOperatorSet)
                }
                Some(
                    <OperatorRemovedFromOperatorSet as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <OperatorRemovedFromOperatorSet as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::OperatorRemovedFromOperatorSet)
                }
                Some(
                    <OperatorSetCreated as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <OperatorSetCreated as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::OperatorSetCreated)
                }
                Some(<OperatorSlashed as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <OperatorSlashed as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::OperatorSlashed)
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
                    <StrategyAddedToOperatorSet as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <StrategyAddedToOperatorSet as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::StrategyAddedToOperatorSet)
                }
                Some(
                    <StrategyRemovedFromOperatorSet as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <StrategyRemovedFromOperatorSet as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::StrategyRemovedFromOperatorSet)
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
    impl alloy_sol_types::private::IntoLogData for AllocationManagerEvents {
        fn to_log_data(&self) -> alloy_sol_types::private::LogData {
            match self {
                Self::AVSMetadataURIUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::AVSRegistrarSet(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::AllocationDelaySet(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::AllocationUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::EncumberedMagnitudeUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::Initialized(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::MaxMagnitudeUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::OperatorAddedToOperatorSet(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::OperatorRemovedFromOperatorSet(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::OperatorSetCreated(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::OperatorSlashed(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::OwnershipTransferred(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::Paused(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::StrategyAddedToOperatorSet(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::StrategyRemovedFromOperatorSet(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::Unpaused(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
            }
        }
        fn into_log_data(self) -> alloy_sol_types::private::LogData {
            match self {
                Self::AVSMetadataURIUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::AVSRegistrarSet(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::AllocationDelaySet(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::AllocationUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::EncumberedMagnitudeUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::Initialized(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::MaxMagnitudeUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::OperatorAddedToOperatorSet(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::OperatorRemovedFromOperatorSet(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::OperatorSetCreated(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::OperatorSlashed(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::OwnershipTransferred(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::Paused(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::StrategyAddedToOperatorSet(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::StrategyRemovedFromOperatorSet(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::Unpaused(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
            }
        }
    }
    use alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`AllocationManager`](self) contract instance.

See the [wrapper's documentation](`AllocationManagerInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> AllocationManagerInstance<T, P, N> {
        AllocationManagerInstance::<T, P, N>::new(address, provider)
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
        _delegation: alloy::sol_types::private::Address,
        _pauserRegistry: alloy::sol_types::private::Address,
        _permissionController: alloy::sol_types::private::Address,
        _DEALLOCATION_DELAY: u32,
        _ALLOCATION_CONFIGURATION_DELAY: u32,
        _version: alloy::sol_types::private::String,
    ) -> impl ::core::future::Future<
        Output = alloy_contract::Result<AllocationManagerInstance<T, P, N>>,
    > {
        AllocationManagerInstance::<
            T,
            P,
            N,
        >::deploy(
            provider,
            _delegation,
            _pauserRegistry,
            _permissionController,
            _DEALLOCATION_DELAY,
            _ALLOCATION_CONFIGURATION_DELAY,
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
        _delegation: alloy::sol_types::private::Address,
        _pauserRegistry: alloy::sol_types::private::Address,
        _permissionController: alloy::sol_types::private::Address,
        _DEALLOCATION_DELAY: u32,
        _ALLOCATION_CONFIGURATION_DELAY: u32,
        _version: alloy::sol_types::private::String,
    ) -> alloy_contract::RawCallBuilder<T, P, N> {
        AllocationManagerInstance::<
            T,
            P,
            N,
        >::deploy_builder(
            provider,
            _delegation,
            _pauserRegistry,
            _permissionController,
            _DEALLOCATION_DELAY,
            _ALLOCATION_CONFIGURATION_DELAY,
            _version,
        )
    }
    /**A [`AllocationManager`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`AllocationManager`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct AllocationManagerInstance<T, P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for AllocationManagerInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("AllocationManagerInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > AllocationManagerInstance<T, P, N> {
        /**Creates a new wrapper around an on-chain [`AllocationManager`](self) contract instance.

See the [wrapper's documentation](`AllocationManagerInstance`) for more details.*/
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
            _delegation: alloy::sol_types::private::Address,
            _pauserRegistry: alloy::sol_types::private::Address,
            _permissionController: alloy::sol_types::private::Address,
            _DEALLOCATION_DELAY: u32,
            _ALLOCATION_CONFIGURATION_DELAY: u32,
            _version: alloy::sol_types::private::String,
        ) -> alloy_contract::Result<AllocationManagerInstance<T, P, N>> {
            let call_builder = Self::deploy_builder(
                provider,
                _delegation,
                _pauserRegistry,
                _permissionController,
                _DEALLOCATION_DELAY,
                _ALLOCATION_CONFIGURATION_DELAY,
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
            _delegation: alloy::sol_types::private::Address,
            _pauserRegistry: alloy::sol_types::private::Address,
            _permissionController: alloy::sol_types::private::Address,
            _DEALLOCATION_DELAY: u32,
            _ALLOCATION_CONFIGURATION_DELAY: u32,
            _version: alloy::sol_types::private::String,
        ) -> alloy_contract::RawCallBuilder<T, P, N> {
            alloy_contract::RawCallBuilder::new_raw_deploy(
                provider,
                [
                    &BYTECODE[..],
                    &alloy_sol_types::SolConstructor::abi_encode(
                        &constructorCall {
                            _delegation,
                            _pauserRegistry,
                            _permissionController,
                            _DEALLOCATION_DELAY,
                            _ALLOCATION_CONFIGURATION_DELAY,
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
    impl<T, P: ::core::clone::Clone, N> AllocationManagerInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> AllocationManagerInstance<T, P, N> {
            AllocationManagerInstance {
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
    > AllocationManagerInstance<T, P, N> {
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
        ///Creates a new call builder for the [`ALLOCATION_CONFIGURATION_DELAY`] function.
        pub fn ALLOCATION_CONFIGURATION_DELAY(
            &self,
        ) -> alloy_contract::SolCallBuilder<
            T,
            &P,
            ALLOCATION_CONFIGURATION_DELAYCall,
            N,
        > {
            self.call_builder(
                &ALLOCATION_CONFIGURATION_DELAYCall {
                },
            )
        }
        ///Creates a new call builder for the [`DEALLOCATION_DELAY`] function.
        pub fn DEALLOCATION_DELAY(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, DEALLOCATION_DELAYCall, N> {
            self.call_builder(&DEALLOCATION_DELAYCall {})
        }
        ///Creates a new call builder for the [`addStrategiesToOperatorSet`] function.
        pub fn addStrategiesToOperatorSet(
            &self,
            avs: alloy::sol_types::private::Address,
            operatorSetId: u32,
            strategies: alloy::sol_types::private::Vec<
                alloy::sol_types::private::Address,
            >,
        ) -> alloy_contract::SolCallBuilder<T, &P, addStrategiesToOperatorSetCall, N> {
            self.call_builder(
                &addStrategiesToOperatorSetCall {
                    avs,
                    operatorSetId,
                    strategies,
                },
            )
        }
        ///Creates a new call builder for the [`clearDeallocationQueue`] function.
        pub fn clearDeallocationQueue(
            &self,
            operator: alloy::sol_types::private::Address,
            strategies: alloy::sol_types::private::Vec<
                alloy::sol_types::private::Address,
            >,
            numToClear: alloy::sol_types::private::Vec<u16>,
        ) -> alloy_contract::SolCallBuilder<T, &P, clearDeallocationQueueCall, N> {
            self.call_builder(
                &clearDeallocationQueueCall {
                    operator,
                    strategies,
                    numToClear,
                },
            )
        }
        ///Creates a new call builder for the [`createOperatorSets`] function.
        pub fn createOperatorSets(
            &self,
            avs: alloy::sol_types::private::Address,
            params: alloy::sol_types::private::Vec<
                <IAllocationManagerTypes::CreateSetParams as alloy::sol_types::SolType>::RustType,
            >,
        ) -> alloy_contract::SolCallBuilder<T, &P, createOperatorSetsCall, N> {
            self.call_builder(
                &createOperatorSetsCall {
                    avs,
                    params,
                },
            )
        }
        ///Creates a new call builder for the [`delegation`] function.
        pub fn delegation(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, delegationCall, N> {
            self.call_builder(&delegationCall {})
        }
        ///Creates a new call builder for the [`deregisterFromOperatorSets`] function.
        pub fn deregisterFromOperatorSets(
            &self,
            params: <IAllocationManagerTypes::DeregisterParams as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<T, &P, deregisterFromOperatorSetsCall, N> {
            self.call_builder(
                &deregisterFromOperatorSetsCall {
                    params,
                },
            )
        }
        ///Creates a new call builder for the [`getAVSRegistrar`] function.
        pub fn getAVSRegistrar(
            &self,
            avs: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, getAVSRegistrarCall, N> {
            self.call_builder(&getAVSRegistrarCall { avs })
        }
        ///Creates a new call builder for the [`getAllocatableMagnitude`] function.
        pub fn getAllocatableMagnitude(
            &self,
            operator: alloy::sol_types::private::Address,
            strategy: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, getAllocatableMagnitudeCall, N> {
            self.call_builder(
                &getAllocatableMagnitudeCall {
                    operator,
                    strategy,
                },
            )
        }
        ///Creates a new call builder for the [`getAllocatedSets`] function.
        pub fn getAllocatedSets(
            &self,
            operator: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, getAllocatedSetsCall, N> {
            self.call_builder(&getAllocatedSetsCall { operator })
        }
        ///Creates a new call builder for the [`getAllocatedStake`] function.
        pub fn getAllocatedStake(
            &self,
            operatorSet: <OperatorSet as alloy::sol_types::SolType>::RustType,
            operators: alloy::sol_types::private::Vec<
                alloy::sol_types::private::Address,
            >,
            strategies: alloy::sol_types::private::Vec<
                alloy::sol_types::private::Address,
            >,
        ) -> alloy_contract::SolCallBuilder<T, &P, getAllocatedStakeCall, N> {
            self.call_builder(
                &getAllocatedStakeCall {
                    operatorSet,
                    operators,
                    strategies,
                },
            )
        }
        ///Creates a new call builder for the [`getAllocatedStrategies`] function.
        pub fn getAllocatedStrategies(
            &self,
            operator: alloy::sol_types::private::Address,
            operatorSet: <OperatorSet as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<T, &P, getAllocatedStrategiesCall, N> {
            self.call_builder(
                &getAllocatedStrategiesCall {
                    operator,
                    operatorSet,
                },
            )
        }
        ///Creates a new call builder for the [`getAllocation`] function.
        pub fn getAllocation(
            &self,
            operator: alloy::sol_types::private::Address,
            operatorSet: <OperatorSet as alloy::sol_types::SolType>::RustType,
            strategy: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, getAllocationCall, N> {
            self.call_builder(
                &getAllocationCall {
                    operator,
                    operatorSet,
                    strategy,
                },
            )
        }
        ///Creates a new call builder for the [`getAllocationDelay`] function.
        pub fn getAllocationDelay(
            &self,
            operator: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, getAllocationDelayCall, N> {
            self.call_builder(&getAllocationDelayCall { operator })
        }
        ///Creates a new call builder for the [`getAllocations`] function.
        pub fn getAllocations(
            &self,
            operators: alloy::sol_types::private::Vec<
                alloy::sol_types::private::Address,
            >,
            operatorSet: <OperatorSet as alloy::sol_types::SolType>::RustType,
            strategy: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, getAllocationsCall, N> {
            self.call_builder(
                &getAllocationsCall {
                    operators,
                    operatorSet,
                    strategy,
                },
            )
        }
        ///Creates a new call builder for the [`getEncumberedMagnitude`] function.
        pub fn getEncumberedMagnitude(
            &self,
            operator: alloy::sol_types::private::Address,
            strategy: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, getEncumberedMagnitudeCall, N> {
            self.call_builder(
                &getEncumberedMagnitudeCall {
                    operator,
                    strategy,
                },
            )
        }
        ///Creates a new call builder for the [`getMaxMagnitude`] function.
        pub fn getMaxMagnitude(
            &self,
            operator: alloy::sol_types::private::Address,
            strategy: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, getMaxMagnitudeCall, N> {
            self.call_builder(
                &getMaxMagnitudeCall {
                    operator,
                    strategy,
                },
            )
        }
        ///Creates a new call builder for the [`getMaxMagnitudes_0`] function.
        pub fn getMaxMagnitudes_0(
            &self,
            operators: alloy::sol_types::private::Vec<
                alloy::sol_types::private::Address,
            >,
            strategy: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, getMaxMagnitudes_0Call, N> {
            self.call_builder(
                &getMaxMagnitudes_0Call {
                    operators,
                    strategy,
                },
            )
        }
        ///Creates a new call builder for the [`getMaxMagnitudes_1`] function.
        pub fn getMaxMagnitudes_1(
            &self,
            operator: alloy::sol_types::private::Address,
            strategies: alloy::sol_types::private::Vec<
                alloy::sol_types::private::Address,
            >,
        ) -> alloy_contract::SolCallBuilder<T, &P, getMaxMagnitudes_1Call, N> {
            self.call_builder(
                &getMaxMagnitudes_1Call {
                    operator,
                    strategies,
                },
            )
        }
        ///Creates a new call builder for the [`getMaxMagnitudesAtBlock`] function.
        pub fn getMaxMagnitudesAtBlock(
            &self,
            operator: alloy::sol_types::private::Address,
            strategies: alloy::sol_types::private::Vec<
                alloy::sol_types::private::Address,
            >,
            blockNumber: u32,
        ) -> alloy_contract::SolCallBuilder<T, &P, getMaxMagnitudesAtBlockCall, N> {
            self.call_builder(
                &getMaxMagnitudesAtBlockCall {
                    operator,
                    strategies,
                    blockNumber,
                },
            )
        }
        ///Creates a new call builder for the [`getMemberCount`] function.
        pub fn getMemberCount(
            &self,
            operatorSet: <OperatorSet as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<T, &P, getMemberCountCall, N> {
            self.call_builder(&getMemberCountCall { operatorSet })
        }
        ///Creates a new call builder for the [`getMembers`] function.
        pub fn getMembers(
            &self,
            operatorSet: <OperatorSet as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<T, &P, getMembersCall, N> {
            self.call_builder(&getMembersCall { operatorSet })
        }
        ///Creates a new call builder for the [`getMinimumSlashableStake`] function.
        pub fn getMinimumSlashableStake(
            &self,
            operatorSet: <OperatorSet as alloy::sol_types::SolType>::RustType,
            operators: alloy::sol_types::private::Vec<
                alloy::sol_types::private::Address,
            >,
            strategies: alloy::sol_types::private::Vec<
                alloy::sol_types::private::Address,
            >,
            futureBlock: u32,
        ) -> alloy_contract::SolCallBuilder<T, &P, getMinimumSlashableStakeCall, N> {
            self.call_builder(
                &getMinimumSlashableStakeCall {
                    operatorSet,
                    operators,
                    strategies,
                    futureBlock,
                },
            )
        }
        ///Creates a new call builder for the [`getOperatorSetCount`] function.
        pub fn getOperatorSetCount(
            &self,
            avs: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, getOperatorSetCountCall, N> {
            self.call_builder(&getOperatorSetCountCall { avs })
        }
        ///Creates a new call builder for the [`getRegisteredSets`] function.
        pub fn getRegisteredSets(
            &self,
            operator: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, getRegisteredSetsCall, N> {
            self.call_builder(&getRegisteredSetsCall { operator })
        }
        ///Creates a new call builder for the [`getStrategiesInOperatorSet`] function.
        pub fn getStrategiesInOperatorSet(
            &self,
            operatorSet: <OperatorSet as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<T, &P, getStrategiesInOperatorSetCall, N> {
            self.call_builder(
                &getStrategiesInOperatorSetCall {
                    operatorSet,
                },
            )
        }
        ///Creates a new call builder for the [`getStrategyAllocations`] function.
        pub fn getStrategyAllocations(
            &self,
            operator: alloy::sol_types::private::Address,
            strategy: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, getStrategyAllocationsCall, N> {
            self.call_builder(
                &getStrategyAllocationsCall {
                    operator,
                    strategy,
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
        ///Creates a new call builder for the [`isMemberOfOperatorSet`] function.
        pub fn isMemberOfOperatorSet(
            &self,
            operator: alloy::sol_types::private::Address,
            operatorSet: <OperatorSet as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<T, &P, isMemberOfOperatorSetCall, N> {
            self.call_builder(
                &isMemberOfOperatorSetCall {
                    operator,
                    operatorSet,
                },
            )
        }
        ///Creates a new call builder for the [`isOperatorSet`] function.
        pub fn isOperatorSet(
            &self,
            operatorSet: <OperatorSet as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<T, &P, isOperatorSetCall, N> {
            self.call_builder(&isOperatorSetCall { operatorSet })
        }
        ///Creates a new call builder for the [`isOperatorSlashable`] function.
        pub fn isOperatorSlashable(
            &self,
            operator: alloy::sol_types::private::Address,
            operatorSet: <OperatorSet as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<T, &P, isOperatorSlashableCall, N> {
            self.call_builder(
                &isOperatorSlashableCall {
                    operator,
                    operatorSet,
                },
            )
        }
        ///Creates a new call builder for the [`modifyAllocations`] function.
        pub fn modifyAllocations(
            &self,
            operator: alloy::sol_types::private::Address,
            params: alloy::sol_types::private::Vec<
                <IAllocationManagerTypes::AllocateParams as alloy::sol_types::SolType>::RustType,
            >,
        ) -> alloy_contract::SolCallBuilder<T, &P, modifyAllocationsCall, N> {
            self.call_builder(
                &modifyAllocationsCall {
                    operator,
                    params,
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
        ///Creates a new call builder for the [`permissionController`] function.
        pub fn permissionController(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, permissionControllerCall, N> {
            self.call_builder(&permissionControllerCall {})
        }
        ///Creates a new call builder for the [`registerForOperatorSets`] function.
        pub fn registerForOperatorSets(
            &self,
            operator: alloy::sol_types::private::Address,
            params: <IAllocationManagerTypes::RegisterParams as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<T, &P, registerForOperatorSetsCall, N> {
            self.call_builder(
                &registerForOperatorSetsCall {
                    operator,
                    params,
                },
            )
        }
        ///Creates a new call builder for the [`removeStrategiesFromOperatorSet`] function.
        pub fn removeStrategiesFromOperatorSet(
            &self,
            avs: alloy::sol_types::private::Address,
            operatorSetId: u32,
            strategies: alloy::sol_types::private::Vec<
                alloy::sol_types::private::Address,
            >,
        ) -> alloy_contract::SolCallBuilder<
            T,
            &P,
            removeStrategiesFromOperatorSetCall,
            N,
        > {
            self.call_builder(
                &removeStrategiesFromOperatorSetCall {
                    avs,
                    operatorSetId,
                    strategies,
                },
            )
        }
        ///Creates a new call builder for the [`renounceOwnership`] function.
        pub fn renounceOwnership(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, renounceOwnershipCall, N> {
            self.call_builder(&renounceOwnershipCall {})
        }
        ///Creates a new call builder for the [`setAVSRegistrar`] function.
        pub fn setAVSRegistrar(
            &self,
            avs: alloy::sol_types::private::Address,
            registrar: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, setAVSRegistrarCall, N> {
            self.call_builder(
                &setAVSRegistrarCall {
                    avs,
                    registrar,
                },
            )
        }
        ///Creates a new call builder for the [`setAllocationDelay`] function.
        pub fn setAllocationDelay(
            &self,
            operator: alloy::sol_types::private::Address,
            delay: u32,
        ) -> alloy_contract::SolCallBuilder<T, &P, setAllocationDelayCall, N> {
            self.call_builder(
                &setAllocationDelayCall {
                    operator,
                    delay,
                },
            )
        }
        ///Creates a new call builder for the [`slashOperator`] function.
        pub fn slashOperator(
            &self,
            avs: alloy::sol_types::private::Address,
            params: <IAllocationManagerTypes::SlashingParams as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<T, &P, slashOperatorCall, N> {
            self.call_builder(&slashOperatorCall { avs, params })
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
        ///Creates a new call builder for the [`updateAVSMetadataURI`] function.
        pub fn updateAVSMetadataURI(
            &self,
            avs: alloy::sol_types::private::Address,
            metadataURI: alloy::sol_types::private::String,
        ) -> alloy_contract::SolCallBuilder<T, &P, updateAVSMetadataURICall, N> {
            self.call_builder(
                &updateAVSMetadataURICall {
                    avs,
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
    > AllocationManagerInstance<T, P, N> {
        /// Creates a new event filter using this contract instance's provider and address.
        ///
        /// Note that the type can be any event, not just those defined in this contract.
        /// Prefer using the other methods for building type-safe event filters.
        pub fn event_filter<E: alloy_sol_types::SolEvent>(
            &self,
        ) -> alloy_contract::Event<T, &P, E, N> {
            alloy_contract::Event::new_sol(&self.provider, &self.address)
        }
        ///Creates a new event filter for the [`AVSMetadataURIUpdated`] event.
        pub fn AVSMetadataURIUpdated_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, AVSMetadataURIUpdated, N> {
            self.event_filter::<AVSMetadataURIUpdated>()
        }
        ///Creates a new event filter for the [`AVSRegistrarSet`] event.
        pub fn AVSRegistrarSet_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, AVSRegistrarSet, N> {
            self.event_filter::<AVSRegistrarSet>()
        }
        ///Creates a new event filter for the [`AllocationDelaySet`] event.
        pub fn AllocationDelaySet_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, AllocationDelaySet, N> {
            self.event_filter::<AllocationDelaySet>()
        }
        ///Creates a new event filter for the [`AllocationUpdated`] event.
        pub fn AllocationUpdated_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, AllocationUpdated, N> {
            self.event_filter::<AllocationUpdated>()
        }
        ///Creates a new event filter for the [`EncumberedMagnitudeUpdated`] event.
        pub fn EncumberedMagnitudeUpdated_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, EncumberedMagnitudeUpdated, N> {
            self.event_filter::<EncumberedMagnitudeUpdated>()
        }
        ///Creates a new event filter for the [`Initialized`] event.
        pub fn Initialized_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, Initialized, N> {
            self.event_filter::<Initialized>()
        }
        ///Creates a new event filter for the [`MaxMagnitudeUpdated`] event.
        pub fn MaxMagnitudeUpdated_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, MaxMagnitudeUpdated, N> {
            self.event_filter::<MaxMagnitudeUpdated>()
        }
        ///Creates a new event filter for the [`OperatorAddedToOperatorSet`] event.
        pub fn OperatorAddedToOperatorSet_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, OperatorAddedToOperatorSet, N> {
            self.event_filter::<OperatorAddedToOperatorSet>()
        }
        ///Creates a new event filter for the [`OperatorRemovedFromOperatorSet`] event.
        pub fn OperatorRemovedFromOperatorSet_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, OperatorRemovedFromOperatorSet, N> {
            self.event_filter::<OperatorRemovedFromOperatorSet>()
        }
        ///Creates a new event filter for the [`OperatorSetCreated`] event.
        pub fn OperatorSetCreated_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, OperatorSetCreated, N> {
            self.event_filter::<OperatorSetCreated>()
        }
        ///Creates a new event filter for the [`OperatorSlashed`] event.
        pub fn OperatorSlashed_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, OperatorSlashed, N> {
            self.event_filter::<OperatorSlashed>()
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
        ///Creates a new event filter for the [`StrategyAddedToOperatorSet`] event.
        pub fn StrategyAddedToOperatorSet_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, StrategyAddedToOperatorSet, N> {
            self.event_filter::<StrategyAddedToOperatorSet>()
        }
        ///Creates a new event filter for the [`StrategyRemovedFromOperatorSet`] event.
        pub fn StrategyRemovedFromOperatorSet_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, StrategyRemovedFromOperatorSet, N> {
            self.event_filter::<StrategyRemovedFromOperatorSet>()
        }
        ///Creates a new event filter for the [`Unpaused`] event.
        pub fn Unpaused_filter(&self) -> alloy_contract::Event<T, &P, Unpaused, N> {
            self.event_filter::<Unpaused>()
        }
    }
}

