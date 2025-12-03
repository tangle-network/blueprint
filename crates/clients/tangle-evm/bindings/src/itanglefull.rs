///Module containing a contract's types and functions.
/**

```solidity
library Types {
    type AssetKind is uint8;
    type MembershipModel is uint8;
    type PricingModel is uint8;
    type ServiceStatus is uint8;
    struct Asset { AssetKind kind; address token; }
    struct AssetSecurityCommitment { Asset asset; uint16 exposureBps; }
    struct AssetSecurityRequirement { Asset asset; uint16 minExposureBps; uint16 maxExposureBps; }
    struct Blueprint { address owner; address manager; uint64 createdAt; uint32 operatorCount; MembershipModel membership; PricingModel pricing; bool active; }
    struct BlueprintConfig { MembershipModel membership; PricingModel pricing; uint32 minOperators; uint32 maxOperators; uint256 subscriptionRate; uint64 subscriptionInterval; uint256 eventRate; }
    struct JobCall { uint8 jobIndex; address caller; uint64 createdAt; uint32 resultCount; uint256 payment; bool completed; }
    struct OperatorRegistration { uint64 registeredAt; uint64 updatedAt; bool active; bool online; }
    struct PaymentSplit { uint16 developerBps; uint16 protocolBps; uint16 operatorBps; uint16 restakerBps; }
    struct QuoteDetails { uint64 blueprintId; uint64 ttlBlocks; uint256 totalCost; uint64 timestamp; uint64 expiry; AssetSecurityCommitment[] securityCommitments; }
    struct Service { uint64 blueprintId; address owner; uint64 createdAt; uint64 ttl; uint64 terminatedAt; uint64 lastPaymentAt; uint32 operatorCount; uint32 minOperators; uint32 maxOperators; MembershipModel membership; PricingModel pricing; ServiceStatus status; }
    struct ServiceOperator { uint16 exposureBps; uint64 joinedAt; uint64 leftAt; bool active; }
    struct ServiceRequest { uint64 blueprintId; address requester; uint64 createdAt; uint64 ttl; uint32 operatorCount; uint32 approvalCount; address paymentToken; uint256 paymentAmount; MembershipModel membership; uint32 minOperators; uint32 maxOperators; bool rejected; }
    struct SignedQuote { QuoteDetails details; bytes signature; address operator; }
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
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct AssetKind(u8);
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<AssetKind> for u8 {
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
        impl AssetKind {
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
        impl alloy_sol_types::SolType for AssetKind {
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
        impl alloy_sol_types::EventTopic for AssetKind {
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
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct MembershipModel(u8);
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<MembershipModel> for u8 {
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
        impl MembershipModel {
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
        impl alloy_sol_types::SolType for MembershipModel {
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
        impl alloy_sol_types::EventTopic for MembershipModel {
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
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct PricingModel(u8);
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<PricingModel> for u8 {
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
        impl PricingModel {
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
        impl alloy_sol_types::SolType for PricingModel {
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
        impl alloy_sol_types::EventTopic for PricingModel {
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
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ServiceStatus(u8);
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<ServiceStatus> for u8 {
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
        impl ServiceStatus {
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
        impl alloy_sol_types::SolType for ServiceStatus {
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
        impl alloy_sol_types::EventTopic for ServiceStatus {
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
struct Asset { AssetKind kind; address token; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct Asset {
        #[allow(missing_docs)]
        pub kind: <AssetKind as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub token: alloy::sol_types::private::Address,
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
        type UnderlyingSolTuple<'a> = (AssetKind, alloy::sol_types::sol_data::Address);
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            <AssetKind as alloy::sol_types::SolType>::RustType,
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
        impl ::core::convert::From<Asset> for UnderlyingRustTuple<'_> {
            fn from(value: Asset) -> Self {
                (value.kind, value.token)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for Asset {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    kind: tuple.0,
                    token: tuple.1,
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
                    <AssetKind as alloy_sol_types::SolType>::tokenize(&self.kind),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.token,
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
                alloy_sol_types::private::Cow::Borrowed(
                    "Asset(uint8 kind,address token)",
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
                    <AssetKind as alloy_sol_types::SolType>::eip712_data_word(&self.kind)
                        .0,
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.token,
                        )
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
                    + <AssetKind as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.kind,
                    )
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.token,
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
                <AssetKind as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.kind,
                    out,
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.token,
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
struct AssetSecurityCommitment { Asset asset; uint16 exposureBps; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct AssetSecurityCommitment {
        #[allow(missing_docs)]
        pub asset: <Asset as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub exposureBps: u16,
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
        type UnderlyingSolTuple<'a> = (Asset, alloy::sol_types::sol_data::Uint<16>);
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            <Asset as alloy::sol_types::SolType>::RustType,
            u16,
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
        impl ::core::convert::From<AssetSecurityCommitment> for UnderlyingRustTuple<'_> {
            fn from(value: AssetSecurityCommitment) -> Self {
                (value.asset, value.exposureBps)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for AssetSecurityCommitment {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    asset: tuple.0,
                    exposureBps: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for AssetSecurityCommitment {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for AssetSecurityCommitment {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <Asset as alloy_sol_types::SolType>::tokenize(&self.asset),
                    <alloy::sol_types::sol_data::Uint<
                        16,
                    > as alloy_sol_types::SolType>::tokenize(&self.exposureBps),
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
        impl alloy_sol_types::SolType for AssetSecurityCommitment {
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
        impl alloy_sol_types::SolStruct for AssetSecurityCommitment {
            const NAME: &'static str = "AssetSecurityCommitment";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "AssetSecurityCommitment(Asset asset,uint16 exposureBps)",
                )
            }
            #[inline]
            fn eip712_components() -> alloy_sol_types::private::Vec<
                alloy_sol_types::private::Cow<'static, str>,
            > {
                let mut components = alloy_sol_types::private::Vec::with_capacity(1);
                components
                    .push(<Asset as alloy_sol_types::SolStruct>::eip712_root_type());
                components
                    .extend(<Asset as alloy_sol_types::SolStruct>::eip712_components());
                components
            }
            #[inline]
            fn eip712_encode_data(&self) -> alloy_sol_types::private::Vec<u8> {
                [
                    <Asset as alloy_sol_types::SolType>::eip712_data_word(&self.asset).0,
                    <alloy::sol_types::sol_data::Uint<
                        16,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.exposureBps)
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for AssetSecurityCommitment {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <Asset as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.asset,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        16,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.exposureBps,
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
                <Asset as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.asset,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    16,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.exposureBps,
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
struct AssetSecurityRequirement { Asset asset; uint16 minExposureBps; uint16 maxExposureBps; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct AssetSecurityRequirement {
        #[allow(missing_docs)]
        pub asset: <Asset as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub minExposureBps: u16,
        #[allow(missing_docs)]
        pub maxExposureBps: u16,
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
            Asset,
            alloy::sol_types::sol_data::Uint<16>,
            alloy::sol_types::sol_data::Uint<16>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            <Asset as alloy::sol_types::SolType>::RustType,
            u16,
            u16,
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
        impl ::core::convert::From<AssetSecurityRequirement>
        for UnderlyingRustTuple<'_> {
            fn from(value: AssetSecurityRequirement) -> Self {
                (value.asset, value.minExposureBps, value.maxExposureBps)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for AssetSecurityRequirement {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    asset: tuple.0,
                    minExposureBps: tuple.1,
                    maxExposureBps: tuple.2,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for AssetSecurityRequirement {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for AssetSecurityRequirement {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <Asset as alloy_sol_types::SolType>::tokenize(&self.asset),
                    <alloy::sol_types::sol_data::Uint<
                        16,
                    > as alloy_sol_types::SolType>::tokenize(&self.minExposureBps),
                    <alloy::sol_types::sol_data::Uint<
                        16,
                    > as alloy_sol_types::SolType>::tokenize(&self.maxExposureBps),
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
        impl alloy_sol_types::SolType for AssetSecurityRequirement {
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
        impl alloy_sol_types::SolStruct for AssetSecurityRequirement {
            const NAME: &'static str = "AssetSecurityRequirement";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "AssetSecurityRequirement(Asset asset,uint16 minExposureBps,uint16 maxExposureBps)",
                )
            }
            #[inline]
            fn eip712_components() -> alloy_sol_types::private::Vec<
                alloy_sol_types::private::Cow<'static, str>,
            > {
                let mut components = alloy_sol_types::private::Vec::with_capacity(1);
                components
                    .push(<Asset as alloy_sol_types::SolStruct>::eip712_root_type());
                components
                    .extend(<Asset as alloy_sol_types::SolStruct>::eip712_components());
                components
            }
            #[inline]
            fn eip712_encode_data(&self) -> alloy_sol_types::private::Vec<u8> {
                [
                    <Asset as alloy_sol_types::SolType>::eip712_data_word(&self.asset).0,
                    <alloy::sol_types::sol_data::Uint<
                        16,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.minExposureBps,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        16,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.maxExposureBps,
                        )
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for AssetSecurityRequirement {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <Asset as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.asset,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        16,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.minExposureBps,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        16,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.maxExposureBps,
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
                <Asset as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.asset,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    16,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.minExposureBps,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    16,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.maxExposureBps,
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
struct Blueprint { address owner; address manager; uint64 createdAt; uint32 operatorCount; MembershipModel membership; PricingModel pricing; bool active; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct Blueprint {
        #[allow(missing_docs)]
        pub owner: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub manager: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub createdAt: u64,
        #[allow(missing_docs)]
        pub operatorCount: u32,
        #[allow(missing_docs)]
        pub membership: <MembershipModel as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub pricing: <PricingModel as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub active: bool,
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
            alloy::sol_types::sol_data::Uint<64>,
            alloy::sol_types::sol_data::Uint<32>,
            MembershipModel,
            PricingModel,
            alloy::sol_types::sol_data::Bool,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::Address,
            alloy::sol_types::private::Address,
            u64,
            u32,
            <MembershipModel as alloy::sol_types::SolType>::RustType,
            <PricingModel as alloy::sol_types::SolType>::RustType,
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
        impl ::core::convert::From<Blueprint> for UnderlyingRustTuple<'_> {
            fn from(value: Blueprint) -> Self {
                (
                    value.owner,
                    value.manager,
                    value.createdAt,
                    value.operatorCount,
                    value.membership,
                    value.pricing,
                    value.active,
                )
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for Blueprint {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    owner: tuple.0,
                    manager: tuple.1,
                    createdAt: tuple.2,
                    operatorCount: tuple.3,
                    membership: tuple.4,
                    pricing: tuple.5,
                    active: tuple.6,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for Blueprint {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for Blueprint {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.owner,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.manager,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::tokenize(&self.createdAt),
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.operatorCount),
                    <MembershipModel as alloy_sol_types::SolType>::tokenize(
                        &self.membership,
                    ),
                    <PricingModel as alloy_sol_types::SolType>::tokenize(&self.pricing),
                    <alloy::sol_types::sol_data::Bool as alloy_sol_types::SolType>::tokenize(
                        &self.active,
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
        impl alloy_sol_types::SolType for Blueprint {
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
        impl alloy_sol_types::SolStruct for Blueprint {
            const NAME: &'static str = "Blueprint";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "Blueprint(address owner,address manager,uint64 createdAt,uint32 operatorCount,uint8 membership,uint8 pricing,bool active)",
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
                            &self.owner,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.manager,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.createdAt)
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.operatorCount)
                        .0,
                    <MembershipModel as alloy_sol_types::SolType>::eip712_data_word(
                            &self.membership,
                        )
                        .0,
                    <PricingModel as alloy_sol_types::SolType>::eip712_data_word(
                            &self.pricing,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Bool as alloy_sol_types::SolType>::eip712_data_word(
                            &self.active,
                        )
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for Blueprint {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.owner,
                    )
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.manager,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.createdAt,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.operatorCount,
                    )
                    + <MembershipModel as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.membership,
                    )
                    + <PricingModel as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.pricing,
                    )
                    + <alloy::sol_types::sol_data::Bool as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.active,
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
                    &rust.owner,
                    out,
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.manager,
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
                    &rust.operatorCount,
                    out,
                );
                <MembershipModel as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.membership,
                    out,
                );
                <PricingModel as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.pricing,
                    out,
                );
                <alloy::sol_types::sol_data::Bool as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.active,
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
struct BlueprintConfig { MembershipModel membership; PricingModel pricing; uint32 minOperators; uint32 maxOperators; uint256 subscriptionRate; uint64 subscriptionInterval; uint256 eventRate; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct BlueprintConfig {
        #[allow(missing_docs)]
        pub membership: <MembershipModel as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub pricing: <PricingModel as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub minOperators: u32,
        #[allow(missing_docs)]
        pub maxOperators: u32,
        #[allow(missing_docs)]
        pub subscriptionRate: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub subscriptionInterval: u64,
        #[allow(missing_docs)]
        pub eventRate: alloy::sol_types::private::primitives::aliases::U256,
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
            MembershipModel,
            PricingModel,
            alloy::sol_types::sol_data::Uint<32>,
            alloy::sol_types::sol_data::Uint<32>,
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Uint<64>,
            alloy::sol_types::sol_data::Uint<256>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            <MembershipModel as alloy::sol_types::SolType>::RustType,
            <PricingModel as alloy::sol_types::SolType>::RustType,
            u32,
            u32,
            alloy::sol_types::private::primitives::aliases::U256,
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
        impl ::core::convert::From<BlueprintConfig> for UnderlyingRustTuple<'_> {
            fn from(value: BlueprintConfig) -> Self {
                (
                    value.membership,
                    value.pricing,
                    value.minOperators,
                    value.maxOperators,
                    value.subscriptionRate,
                    value.subscriptionInterval,
                    value.eventRate,
                )
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for BlueprintConfig {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    membership: tuple.0,
                    pricing: tuple.1,
                    minOperators: tuple.2,
                    maxOperators: tuple.3,
                    subscriptionRate: tuple.4,
                    subscriptionInterval: tuple.5,
                    eventRate: tuple.6,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for BlueprintConfig {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for BlueprintConfig {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <MembershipModel as alloy_sol_types::SolType>::tokenize(
                        &self.membership,
                    ),
                    <PricingModel as alloy_sol_types::SolType>::tokenize(&self.pricing),
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.minOperators),
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.maxOperators),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.subscriptionRate),
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::tokenize(&self.subscriptionInterval),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.eventRate),
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
        impl alloy_sol_types::SolType for BlueprintConfig {
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
        impl alloy_sol_types::SolStruct for BlueprintConfig {
            const NAME: &'static str = "BlueprintConfig";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "BlueprintConfig(uint8 membership,uint8 pricing,uint32 minOperators,uint32 maxOperators,uint256 subscriptionRate,uint64 subscriptionInterval,uint256 eventRate)",
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
                    <MembershipModel as alloy_sol_types::SolType>::eip712_data_word(
                            &self.membership,
                        )
                        .0,
                    <PricingModel as alloy_sol_types::SolType>::eip712_data_word(
                            &self.pricing,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.minOperators)
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.maxOperators)
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.subscriptionRate,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.subscriptionInterval,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.eventRate)
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for BlueprintConfig {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <MembershipModel as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.membership,
                    )
                    + <PricingModel as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.pricing,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.minOperators,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.maxOperators,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.subscriptionRate,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.subscriptionInterval,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.eventRate,
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
                <MembershipModel as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.membership,
                    out,
                );
                <PricingModel as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.pricing,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.minOperators,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.maxOperators,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.subscriptionRate,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    64,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.subscriptionInterval,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.eventRate,
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
    /**```solidity
struct OperatorRegistration { uint64 registeredAt; uint64 updatedAt; bool active; bool online; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct OperatorRegistration {
        #[allow(missing_docs)]
        pub registeredAt: u64,
        #[allow(missing_docs)]
        pub updatedAt: u64,
        #[allow(missing_docs)]
        pub active: bool,
        #[allow(missing_docs)]
        pub online: bool,
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
            alloy::sol_types::sol_data::Uint<64>,
            alloy::sol_types::sol_data::Bool,
            alloy::sol_types::sol_data::Bool,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (u64, u64, bool, bool);
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<OperatorRegistration> for UnderlyingRustTuple<'_> {
            fn from(value: OperatorRegistration) -> Self {
                (value.registeredAt, value.updatedAt, value.active, value.online)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for OperatorRegistration {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    registeredAt: tuple.0,
                    updatedAt: tuple.1,
                    active: tuple.2,
                    online: tuple.3,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for OperatorRegistration {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for OperatorRegistration {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::tokenize(&self.registeredAt),
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::tokenize(&self.updatedAt),
                    <alloy::sol_types::sol_data::Bool as alloy_sol_types::SolType>::tokenize(
                        &self.active,
                    ),
                    <alloy::sol_types::sol_data::Bool as alloy_sol_types::SolType>::tokenize(
                        &self.online,
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
        impl alloy_sol_types::SolType for OperatorRegistration {
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
        impl alloy_sol_types::SolStruct for OperatorRegistration {
            const NAME: &'static str = "OperatorRegistration";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "OperatorRegistration(uint64 registeredAt,uint64 updatedAt,bool active,bool online)",
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
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.registeredAt)
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.updatedAt)
                        .0,
                    <alloy::sol_types::sol_data::Bool as alloy_sol_types::SolType>::eip712_data_word(
                            &self.active,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Bool as alloy_sol_types::SolType>::eip712_data_word(
                            &self.online,
                        )
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for OperatorRegistration {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.registeredAt,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.updatedAt,
                    )
                    + <alloy::sol_types::sol_data::Bool as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.active,
                    )
                    + <alloy::sol_types::sol_data::Bool as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.online,
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
                    &rust.registeredAt,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    64,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.updatedAt,
                    out,
                );
                <alloy::sol_types::sol_data::Bool as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.active,
                    out,
                );
                <alloy::sol_types::sol_data::Bool as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.online,
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
struct PaymentSplit { uint16 developerBps; uint16 protocolBps; uint16 operatorBps; uint16 restakerBps; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct PaymentSplit {
        #[allow(missing_docs)]
        pub developerBps: u16,
        #[allow(missing_docs)]
        pub protocolBps: u16,
        #[allow(missing_docs)]
        pub operatorBps: u16,
        #[allow(missing_docs)]
        pub restakerBps: u16,
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
            alloy::sol_types::sol_data::Uint<16>,
            alloy::sol_types::sol_data::Uint<16>,
            alloy::sol_types::sol_data::Uint<16>,
            alloy::sol_types::sol_data::Uint<16>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (u16, u16, u16, u16);
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<PaymentSplit> for UnderlyingRustTuple<'_> {
            fn from(value: PaymentSplit) -> Self {
                (
                    value.developerBps,
                    value.protocolBps,
                    value.operatorBps,
                    value.restakerBps,
                )
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for PaymentSplit {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    developerBps: tuple.0,
                    protocolBps: tuple.1,
                    operatorBps: tuple.2,
                    restakerBps: tuple.3,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for PaymentSplit {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for PaymentSplit {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        16,
                    > as alloy_sol_types::SolType>::tokenize(&self.developerBps),
                    <alloy::sol_types::sol_data::Uint<
                        16,
                    > as alloy_sol_types::SolType>::tokenize(&self.protocolBps),
                    <alloy::sol_types::sol_data::Uint<
                        16,
                    > as alloy_sol_types::SolType>::tokenize(&self.operatorBps),
                    <alloy::sol_types::sol_data::Uint<
                        16,
                    > as alloy_sol_types::SolType>::tokenize(&self.restakerBps),
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
        impl alloy_sol_types::SolType for PaymentSplit {
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
        impl alloy_sol_types::SolStruct for PaymentSplit {
            const NAME: &'static str = "PaymentSplit";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "PaymentSplit(uint16 developerBps,uint16 protocolBps,uint16 operatorBps,uint16 restakerBps)",
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
                        16,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.developerBps)
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        16,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.protocolBps)
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        16,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.operatorBps)
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        16,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.restakerBps)
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for PaymentSplit {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Uint<
                        16,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.developerBps,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        16,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.protocolBps,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        16,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.operatorBps,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        16,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.restakerBps,
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
                    16,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.developerBps,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    16,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.protocolBps,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    16,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.operatorBps,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    16,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.restakerBps,
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
struct QuoteDetails { uint64 blueprintId; uint64 ttlBlocks; uint256 totalCost; uint64 timestamp; uint64 expiry; AssetSecurityCommitment[] securityCommitments; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct QuoteDetails {
        #[allow(missing_docs)]
        pub blueprintId: u64,
        #[allow(missing_docs)]
        pub ttlBlocks: u64,
        #[allow(missing_docs)]
        pub totalCost: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub timestamp: u64,
        #[allow(missing_docs)]
        pub expiry: u64,
        #[allow(missing_docs)]
        pub securityCommitments: alloy::sol_types::private::Vec<
            <AssetSecurityCommitment as alloy::sol_types::SolType>::RustType,
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
            alloy::sol_types::sol_data::Uint<64>,
            alloy::sol_types::sol_data::Uint<64>,
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Uint<64>,
            alloy::sol_types::sol_data::Uint<64>,
            alloy::sol_types::sol_data::Array<AssetSecurityCommitment>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            u64,
            u64,
            alloy::sol_types::private::primitives::aliases::U256,
            u64,
            u64,
            alloy::sol_types::private::Vec<
                <AssetSecurityCommitment as alloy::sol_types::SolType>::RustType,
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
        impl ::core::convert::From<QuoteDetails> for UnderlyingRustTuple<'_> {
            fn from(value: QuoteDetails) -> Self {
                (
                    value.blueprintId,
                    value.ttlBlocks,
                    value.totalCost,
                    value.timestamp,
                    value.expiry,
                    value.securityCommitments,
                )
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for QuoteDetails {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    blueprintId: tuple.0,
                    ttlBlocks: tuple.1,
                    totalCost: tuple.2,
                    timestamp: tuple.3,
                    expiry: tuple.4,
                    securityCommitments: tuple.5,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for QuoteDetails {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for QuoteDetails {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::tokenize(&self.blueprintId),
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::tokenize(&self.ttlBlocks),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.totalCost),
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::tokenize(&self.timestamp),
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::tokenize(&self.expiry),
                    <alloy::sol_types::sol_data::Array<
                        AssetSecurityCommitment,
                    > as alloy_sol_types::SolType>::tokenize(&self.securityCommitments),
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
        impl alloy_sol_types::SolType for QuoteDetails {
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
        impl alloy_sol_types::SolStruct for QuoteDetails {
            const NAME: &'static str = "QuoteDetails";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "QuoteDetails(uint64 blueprintId,uint64 ttlBlocks,uint256 totalCost,uint64 timestamp,uint64 expiry,AssetSecurityCommitment[] securityCommitments)",
                )
            }
            #[inline]
            fn eip712_components() -> alloy_sol_types::private::Vec<
                alloy_sol_types::private::Cow<'static, str>,
            > {
                let mut components = alloy_sol_types::private::Vec::with_capacity(1);
                components
                    .push(
                        <AssetSecurityCommitment as alloy_sol_types::SolStruct>::eip712_root_type(),
                    );
                components
                    .extend(
                        <AssetSecurityCommitment as alloy_sol_types::SolStruct>::eip712_components(),
                    );
                components
            }
            #[inline]
            fn eip712_encode_data(&self) -> alloy_sol_types::private::Vec<u8> {
                [
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.blueprintId)
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.ttlBlocks)
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.totalCost)
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.timestamp)
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.expiry)
                        .0,
                    <alloy::sol_types::sol_data::Array<
                        AssetSecurityCommitment,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.securityCommitments,
                        )
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for QuoteDetails {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.blueprintId,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.ttlBlocks,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.totalCost,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.timestamp,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.expiry,
                    )
                    + <alloy::sol_types::sol_data::Array<
                        AssetSecurityCommitment,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.securityCommitments,
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
                    &rust.blueprintId,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    64,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.ttlBlocks,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.totalCost,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    64,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.timestamp,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    64,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.expiry,
                    out,
                );
                <alloy::sol_types::sol_data::Array<
                    AssetSecurityCommitment,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.securityCommitments,
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
struct Service { uint64 blueprintId; address owner; uint64 createdAt; uint64 ttl; uint64 terminatedAt; uint64 lastPaymentAt; uint32 operatorCount; uint32 minOperators; uint32 maxOperators; MembershipModel membership; PricingModel pricing; ServiceStatus status; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct Service {
        #[allow(missing_docs)]
        pub blueprintId: u64,
        #[allow(missing_docs)]
        pub owner: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub createdAt: u64,
        #[allow(missing_docs)]
        pub ttl: u64,
        #[allow(missing_docs)]
        pub terminatedAt: u64,
        #[allow(missing_docs)]
        pub lastPaymentAt: u64,
        #[allow(missing_docs)]
        pub operatorCount: u32,
        #[allow(missing_docs)]
        pub minOperators: u32,
        #[allow(missing_docs)]
        pub maxOperators: u32,
        #[allow(missing_docs)]
        pub membership: <MembershipModel as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub pricing: <PricingModel as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub status: <ServiceStatus as alloy::sol_types::SolType>::RustType,
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
            alloy::sol_types::sol_data::Uint<64>,
            alloy::sol_types::sol_data::Uint<64>,
            alloy::sol_types::sol_data::Uint<64>,
            alloy::sol_types::sol_data::Uint<64>,
            alloy::sol_types::sol_data::Uint<32>,
            alloy::sol_types::sol_data::Uint<32>,
            alloy::sol_types::sol_data::Uint<32>,
            MembershipModel,
            PricingModel,
            ServiceStatus,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            u64,
            alloy::sol_types::private::Address,
            u64,
            u64,
            u64,
            u64,
            u32,
            u32,
            u32,
            <MembershipModel as alloy::sol_types::SolType>::RustType,
            <PricingModel as alloy::sol_types::SolType>::RustType,
            <ServiceStatus as alloy::sol_types::SolType>::RustType,
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
        impl ::core::convert::From<Service> for UnderlyingRustTuple<'_> {
            fn from(value: Service) -> Self {
                (
                    value.blueprintId,
                    value.owner,
                    value.createdAt,
                    value.ttl,
                    value.terminatedAt,
                    value.lastPaymentAt,
                    value.operatorCount,
                    value.minOperators,
                    value.maxOperators,
                    value.membership,
                    value.pricing,
                    value.status,
                )
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for Service {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    blueprintId: tuple.0,
                    owner: tuple.1,
                    createdAt: tuple.2,
                    ttl: tuple.3,
                    terminatedAt: tuple.4,
                    lastPaymentAt: tuple.5,
                    operatorCount: tuple.6,
                    minOperators: tuple.7,
                    maxOperators: tuple.8,
                    membership: tuple.9,
                    pricing: tuple.10,
                    status: tuple.11,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for Service {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for Service {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::tokenize(&self.blueprintId),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.owner,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::tokenize(&self.createdAt),
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::tokenize(&self.ttl),
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::tokenize(&self.terminatedAt),
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::tokenize(&self.lastPaymentAt),
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.operatorCount),
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.minOperators),
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.maxOperators),
                    <MembershipModel as alloy_sol_types::SolType>::tokenize(
                        &self.membership,
                    ),
                    <PricingModel as alloy_sol_types::SolType>::tokenize(&self.pricing),
                    <ServiceStatus as alloy_sol_types::SolType>::tokenize(&self.status),
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
        impl alloy_sol_types::SolType for Service {
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
        impl alloy_sol_types::SolStruct for Service {
            const NAME: &'static str = "Service";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "Service(uint64 blueprintId,address owner,uint64 createdAt,uint64 ttl,uint64 terminatedAt,uint64 lastPaymentAt,uint32 operatorCount,uint32 minOperators,uint32 maxOperators,uint8 membership,uint8 pricing,uint8 status)",
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
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.blueprintId)
                        .0,
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.owner,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.createdAt)
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.ttl)
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.terminatedAt)
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.lastPaymentAt)
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.operatorCount)
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.minOperators)
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.maxOperators)
                        .0,
                    <MembershipModel as alloy_sol_types::SolType>::eip712_data_word(
                            &self.membership,
                        )
                        .0,
                    <PricingModel as alloy_sol_types::SolType>::eip712_data_word(
                            &self.pricing,
                        )
                        .0,
                    <ServiceStatus as alloy_sol_types::SolType>::eip712_data_word(
                            &self.status,
                        )
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for Service {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.blueprintId,
                    )
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.owner,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.createdAt,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(&rust.ttl)
                    + <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.terminatedAt,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.lastPaymentAt,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.operatorCount,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.minOperators,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.maxOperators,
                    )
                    + <MembershipModel as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.membership,
                    )
                    + <PricingModel as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.pricing,
                    )
                    + <ServiceStatus as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.status,
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
                    &rust.blueprintId,
                    out,
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.owner,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    64,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.createdAt,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    64,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(&rust.ttl, out);
                <alloy::sol_types::sol_data::Uint<
                    64,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.terminatedAt,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    64,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.lastPaymentAt,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.operatorCount,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.minOperators,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.maxOperators,
                    out,
                );
                <MembershipModel as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.membership,
                    out,
                );
                <PricingModel as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.pricing,
                    out,
                );
                <ServiceStatus as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.status,
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
struct ServiceOperator { uint16 exposureBps; uint64 joinedAt; uint64 leftAt; bool active; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ServiceOperator {
        #[allow(missing_docs)]
        pub exposureBps: u16,
        #[allow(missing_docs)]
        pub joinedAt: u64,
        #[allow(missing_docs)]
        pub leftAt: u64,
        #[allow(missing_docs)]
        pub active: bool,
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
            alloy::sol_types::sol_data::Uint<16>,
            alloy::sol_types::sol_data::Uint<64>,
            alloy::sol_types::sol_data::Uint<64>,
            alloy::sol_types::sol_data::Bool,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (u16, u64, u64, bool);
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<ServiceOperator> for UnderlyingRustTuple<'_> {
            fn from(value: ServiceOperator) -> Self {
                (value.exposureBps, value.joinedAt, value.leftAt, value.active)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for ServiceOperator {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    exposureBps: tuple.0,
                    joinedAt: tuple.1,
                    leftAt: tuple.2,
                    active: tuple.3,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for ServiceOperator {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for ServiceOperator {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        16,
                    > as alloy_sol_types::SolType>::tokenize(&self.exposureBps),
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::tokenize(&self.joinedAt),
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::tokenize(&self.leftAt),
                    <alloy::sol_types::sol_data::Bool as alloy_sol_types::SolType>::tokenize(
                        &self.active,
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
        impl alloy_sol_types::SolType for ServiceOperator {
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
        impl alloy_sol_types::SolStruct for ServiceOperator {
            const NAME: &'static str = "ServiceOperator";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "ServiceOperator(uint16 exposureBps,uint64 joinedAt,uint64 leftAt,bool active)",
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
                        16,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.exposureBps)
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.joinedAt)
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.leftAt)
                        .0,
                    <alloy::sol_types::sol_data::Bool as alloy_sol_types::SolType>::eip712_data_word(
                            &self.active,
                        )
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for ServiceOperator {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Uint<
                        16,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.exposureBps,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.joinedAt,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.leftAt,
                    )
                    + <alloy::sol_types::sol_data::Bool as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.active,
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
                    16,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.exposureBps,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    64,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.joinedAt,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    64,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.leftAt,
                    out,
                );
                <alloy::sol_types::sol_data::Bool as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.active,
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
struct ServiceRequest { uint64 blueprintId; address requester; uint64 createdAt; uint64 ttl; uint32 operatorCount; uint32 approvalCount; address paymentToken; uint256 paymentAmount; MembershipModel membership; uint32 minOperators; uint32 maxOperators; bool rejected; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ServiceRequest {
        #[allow(missing_docs)]
        pub blueprintId: u64,
        #[allow(missing_docs)]
        pub requester: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub createdAt: u64,
        #[allow(missing_docs)]
        pub ttl: u64,
        #[allow(missing_docs)]
        pub operatorCount: u32,
        #[allow(missing_docs)]
        pub approvalCount: u32,
        #[allow(missing_docs)]
        pub paymentToken: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub paymentAmount: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub membership: <MembershipModel as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub minOperators: u32,
        #[allow(missing_docs)]
        pub maxOperators: u32,
        #[allow(missing_docs)]
        pub rejected: bool,
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
            alloy::sol_types::sol_data::Uint<64>,
            alloy::sol_types::sol_data::Uint<64>,
            alloy::sol_types::sol_data::Uint<32>,
            alloy::sol_types::sol_data::Uint<32>,
            alloy::sol_types::sol_data::Address,
            alloy::sol_types::sol_data::Uint<256>,
            MembershipModel,
            alloy::sol_types::sol_data::Uint<32>,
            alloy::sol_types::sol_data::Uint<32>,
            alloy::sol_types::sol_data::Bool,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            u64,
            alloy::sol_types::private::Address,
            u64,
            u64,
            u32,
            u32,
            alloy::sol_types::private::Address,
            alloy::sol_types::private::primitives::aliases::U256,
            <MembershipModel as alloy::sol_types::SolType>::RustType,
            u32,
            u32,
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
        impl ::core::convert::From<ServiceRequest> for UnderlyingRustTuple<'_> {
            fn from(value: ServiceRequest) -> Self {
                (
                    value.blueprintId,
                    value.requester,
                    value.createdAt,
                    value.ttl,
                    value.operatorCount,
                    value.approvalCount,
                    value.paymentToken,
                    value.paymentAmount,
                    value.membership,
                    value.minOperators,
                    value.maxOperators,
                    value.rejected,
                )
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for ServiceRequest {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    blueprintId: tuple.0,
                    requester: tuple.1,
                    createdAt: tuple.2,
                    ttl: tuple.3,
                    operatorCount: tuple.4,
                    approvalCount: tuple.5,
                    paymentToken: tuple.6,
                    paymentAmount: tuple.7,
                    membership: tuple.8,
                    minOperators: tuple.9,
                    maxOperators: tuple.10,
                    rejected: tuple.11,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for ServiceRequest {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for ServiceRequest {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::tokenize(&self.blueprintId),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.requester,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::tokenize(&self.createdAt),
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::tokenize(&self.ttl),
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.operatorCount),
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.approvalCount),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.paymentToken,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.paymentAmount),
                    <MembershipModel as alloy_sol_types::SolType>::tokenize(
                        &self.membership,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.minOperators),
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.maxOperators),
                    <alloy::sol_types::sol_data::Bool as alloy_sol_types::SolType>::tokenize(
                        &self.rejected,
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
        impl alloy_sol_types::SolType for ServiceRequest {
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
        impl alloy_sol_types::SolStruct for ServiceRequest {
            const NAME: &'static str = "ServiceRequest";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "ServiceRequest(uint64 blueprintId,address requester,uint64 createdAt,uint64 ttl,uint32 operatorCount,uint32 approvalCount,address paymentToken,uint256 paymentAmount,uint8 membership,uint32 minOperators,uint32 maxOperators,bool rejected)",
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
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.blueprintId)
                        .0,
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.requester,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.createdAt)
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.ttl)
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.operatorCount)
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.approvalCount)
                        .0,
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.paymentToken,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.paymentAmount)
                        .0,
                    <MembershipModel as alloy_sol_types::SolType>::eip712_data_word(
                            &self.membership,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.minOperators)
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.maxOperators)
                        .0,
                    <alloy::sol_types::sol_data::Bool as alloy_sol_types::SolType>::eip712_data_word(
                            &self.rejected,
                        )
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for ServiceRequest {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.blueprintId,
                    )
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.requester,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.createdAt,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(&rust.ttl)
                    + <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.operatorCount,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.approvalCount,
                    )
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.paymentToken,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.paymentAmount,
                    )
                    + <MembershipModel as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.membership,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.minOperators,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.maxOperators,
                    )
                    + <alloy::sol_types::sol_data::Bool as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.rejected,
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
                    &rust.blueprintId,
                    out,
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.requester,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    64,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.createdAt,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    64,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(&rust.ttl, out);
                <alloy::sol_types::sol_data::Uint<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.operatorCount,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.approvalCount,
                    out,
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.paymentToken,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.paymentAmount,
                    out,
                );
                <MembershipModel as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.membership,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.minOperators,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.maxOperators,
                    out,
                );
                <alloy::sol_types::sol_data::Bool as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.rejected,
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
struct SignedQuote { QuoteDetails details; bytes signature; address operator; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct SignedQuote {
        #[allow(missing_docs)]
        pub details: <QuoteDetails as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub signature: alloy::sol_types::private::Bytes,
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
            QuoteDetails,
            alloy::sol_types::sol_data::Bytes,
            alloy::sol_types::sol_data::Address,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            <QuoteDetails as alloy::sol_types::SolType>::RustType,
            alloy::sol_types::private::Bytes,
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
        impl ::core::convert::From<SignedQuote> for UnderlyingRustTuple<'_> {
            fn from(value: SignedQuote) -> Self {
                (value.details, value.signature, value.operator)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for SignedQuote {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    details: tuple.0,
                    signature: tuple.1,
                    operator: tuple.2,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for SignedQuote {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for SignedQuote {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <QuoteDetails as alloy_sol_types::SolType>::tokenize(&self.details),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.signature,
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
        impl alloy_sol_types::SolType for SignedQuote {
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
        impl alloy_sol_types::SolStruct for SignedQuote {
            const NAME: &'static str = "SignedQuote";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "SignedQuote(QuoteDetails details,bytes signature,address operator)",
                )
            }
            #[inline]
            fn eip712_components() -> alloy_sol_types::private::Vec<
                alloy_sol_types::private::Cow<'static, str>,
            > {
                let mut components = alloy_sol_types::private::Vec::with_capacity(1);
                components
                    .push(
                        <QuoteDetails as alloy_sol_types::SolStruct>::eip712_root_type(),
                    );
                components
                    .extend(
                        <QuoteDetails as alloy_sol_types::SolStruct>::eip712_components(),
                    );
                components
            }
            #[inline]
            fn eip712_encode_data(&self) -> alloy_sol_types::private::Vec<u8> {
                [
                    <QuoteDetails as alloy_sol_types::SolType>::eip712_data_word(
                            &self.details,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::eip712_data_word(
                            &self.signature,
                        )
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
        impl alloy_sol_types::EventTopic for SignedQuote {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <QuoteDetails as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.details,
                    )
                    + <alloy::sol_types::sol_data::Bytes as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.signature,
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
                out.reserve(
                    <Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust),
                );
                <QuoteDetails as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.details,
                    out,
                );
                <alloy::sol_types::sol_data::Bytes as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.signature,
                    out,
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.operator,
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
    type AssetKind is uint8;
    type MembershipModel is uint8;
    type PricingModel is uint8;
    type ServiceStatus is uint8;
    struct Asset {
        AssetKind kind;
        address token;
    }
    struct AssetSecurityCommitment {
        Asset asset;
        uint16 exposureBps;
    }
    struct AssetSecurityRequirement {
        Asset asset;
        uint16 minExposureBps;
        uint16 maxExposureBps;
    }
    struct Blueprint {
        address owner;
        address manager;
        uint64 createdAt;
        uint32 operatorCount;
        MembershipModel membership;
        PricingModel pricing;
        bool active;
    }
    struct BlueprintConfig {
        MembershipModel membership;
        PricingModel pricing;
        uint32 minOperators;
        uint32 maxOperators;
        uint256 subscriptionRate;
        uint64 subscriptionInterval;
        uint256 eventRate;
    }
    struct JobCall {
        uint8 jobIndex;
        address caller;
        uint64 createdAt;
        uint32 resultCount;
        uint256 payment;
        bool completed;
    }
    struct OperatorRegistration {
        uint64 registeredAt;
        uint64 updatedAt;
        bool active;
        bool online;
    }
    struct PaymentSplit {
        uint16 developerBps;
        uint16 protocolBps;
        uint16 operatorBps;
        uint16 restakerBps;
    }
    struct QuoteDetails {
        uint64 blueprintId;
        uint64 ttlBlocks;
        uint256 totalCost;
        uint64 timestamp;
        uint64 expiry;
        AssetSecurityCommitment[] securityCommitments;
    }
    struct Service {
        uint64 blueprintId;
        address owner;
        uint64 createdAt;
        uint64 ttl;
        uint64 terminatedAt;
        uint64 lastPaymentAt;
        uint32 operatorCount;
        uint32 minOperators;
        uint32 maxOperators;
        MembershipModel membership;
        PricingModel pricing;
        ServiceStatus status;
    }
    struct ServiceOperator {
        uint16 exposureBps;
        uint64 joinedAt;
        uint64 leftAt;
        bool active;
    }
    struct ServiceRequest {
        uint64 blueprintId;
        address requester;
        uint64 createdAt;
        uint64 ttl;
        uint32 operatorCount;
        uint32 approvalCount;
        address paymentToken;
        uint256 paymentAmount;
        MembershipModel membership;
        uint32 minOperators;
        uint32 maxOperators;
        bool rejected;
    }
    struct SignedQuote {
        QuoteDetails details;
        bytes signature;
        address operator;
    }
}

interface ITangleFull {
    event BlueprintCreated(uint64 indexed blueprintId, address indexed owner, address manager, string metadataUri);
    event BlueprintDeactivated(uint64 indexed blueprintId);
    event BlueprintTransferred(uint64 indexed blueprintId, address indexed from, address indexed to);
    event BlueprintUpdated(uint64 indexed blueprintId, string metadataUri);
    event JobCompleted(uint64 indexed serviceId, uint64 indexed callId, uint32 resultCount);
    event JobResultSubmitted(uint64 indexed serviceId, uint64 indexed callId, address indexed operator, bytes result);
    event JobSubmitted(uint64 indexed serviceId, uint64 indexed callId, uint8 indexed jobIndex, address caller, bytes inputs);
    event OperatorJoinedService(uint64 indexed serviceId, address indexed operator, uint16 exposureBps);
    event OperatorLeftService(uint64 indexed serviceId, address indexed operator);
    event OperatorOnlineStatusChanged(uint64 indexed blueprintId, address indexed operator, bool online);
    event OperatorPreferencesUpdated(uint64 indexed blueprintId, address indexed operator, bytes preferences);
    event OperatorRegistered(uint64 indexed blueprintId, address indexed operator, bytes preferences);
    event OperatorUnregistered(uint64 indexed blueprintId, address indexed operator);
    event RewardsClaimed(address indexed account, uint256 amount);
    event RewardsDistributed(uint64 indexed serviceId, uint256 developerAmount, uint256 protocolAmount, uint256 operatorAmount, uint256 restakerAmount);
    event ServiceActivated(uint64 indexed serviceId, uint64 indexed requestId, uint64 indexed blueprintId, address owner, address[] operators);
    event ServiceApproved(uint64 indexed requestId, address indexed operator);
    event ServiceCreatedFromQuotes(uint64 indexed serviceId, uint64 indexed blueprintId, address indexed owner, address[] operators);
    event ServiceRejected(uint64 indexed requestId, address indexed operator);
    event ServiceRequested(uint64 indexed requestId, uint64 indexed blueprintId, address indexed requester, address[] operators, bytes config, uint64 ttl, address paymentToken, uint256 paymentAmount);
    event ServiceRequestedWithSecurity(uint64 indexed requestId, uint64 indexed blueprintId, address indexed requester, address[] operators, Types.AssetSecurityRequirement[] securityRequirements);
    event ServiceTerminated(uint64 indexed serviceId, address indexed owner);
    event SlashExecuted(uint64 indexed serviceId, address indexed operator, uint256 amount);
    event SlashProposed(uint64 indexed serviceId, address indexed operator, uint256 amount, bytes32 evidence);
    event SubscriptionBilled(uint64 indexed serviceId, uint256 amount, uint64 billedAt);

    function addPermittedCaller(uint64 serviceId, address caller) external;
    function approveService(uint64 requestId, uint8 restakingPercent) external;
    function approveServiceWithCommitments(uint64 requestId, Types.AssetSecurityCommitment[] memory commitments) external;
    function billSubscription(uint64 serviceId) external;
    function blueprintCount() external view returns (uint64);
    function blueprintOperatorCount(uint64 blueprintId) external view returns (uint256);
    function claimRewards() external;
    function createBlueprint(string memory metadataUri, address manager) external returns (uint64 blueprintId);
    function createBlueprintWithConfig(string memory metadataUri, address manager, Types.BlueprintConfig memory config) external returns (uint64 blueprintId);
    function createServiceFromQuotes(uint64 blueprintId, Types.SignedQuote[] memory quotes, bytes memory config, address[] memory permittedCallers, uint64 ttl) external payable returns (uint64 serviceId);
    function deactivateBlueprint(uint64 blueprintId) external;
    function getBlueprint(uint64 blueprintId) external view returns (Types.Blueprint memory);
    function getBlueprintConfig(uint64 blueprintId) external view returns (Types.BlueprintConfig memory);
    function getJobCall(uint64 serviceId, uint64 callId) external view returns (Types.JobCall memory);
    function getOperatorRegistration(uint64 blueprintId, address operator) external view returns (Types.OperatorRegistration memory);
    function getService(uint64 serviceId) external view returns (Types.Service memory);
    function getServiceOperator(uint64 serviceId, address operator) external view returns (Types.ServiceOperator memory);
    function getServiceOperators(uint64 serviceId) external view returns (address[] memory);
    function getServiceRequest(uint64 requestId) external view returns (Types.ServiceRequest memory);
    function getServiceTotalExposure(uint64 serviceId) external view returns (uint256);
    function isOperatorRegistered(uint64 blueprintId, address operator) external view returns (bool);
    function isPermittedCaller(uint64 serviceId, address caller) external view returns (bool);
    function isServiceActive(uint64 serviceId) external view returns (bool);
    function isServiceOperator(uint64 serviceId, address operator) external view returns (bool);
    function joinService(uint64 serviceId, uint16 exposureBps) external;
    function leaveService(uint64 serviceId) external;
    function pause() external;
    function pendingRewards(address account) external view returns (uint256);
    function proposeSlash(uint64 serviceId, address operator, uint256 amount, bytes32 evidence) external;
    function registerOperator(uint64 blueprintId, bytes memory preferences) external;
    function rejectService(uint64 requestId) external;
    function removePermittedCaller(uint64 serviceId, address caller) external;
    function requestService(uint64 blueprintId, address[] memory operators, bytes memory config, address[] memory permittedCallers, uint64 ttl, address paymentToken, uint256 paymentAmount) external payable returns (uint64 requestId);
    function requestServiceWithExposure(uint64 blueprintId, address[] memory operators, uint16[] memory exposureBps, bytes memory config, address[] memory permittedCallers, uint64 ttl, address paymentToken, uint256 paymentAmount) external payable returns (uint64 requestId);
    function requestServiceWithSecurity(uint64 blueprintId, address[] memory operators, Types.AssetSecurityRequirement[] memory securityRequirements, bytes memory config, address[] memory permittedCallers, uint64 ttl, address paymentToken, uint256 paymentAmount) external payable returns (uint64 requestId);
    function serviceCount() external view returns (uint64);
    function setOperatorOnline(uint64 blueprintId, bool online) external;
    function setPaymentSplit(Types.PaymentSplit memory split) external;
    function setRestaking(address restaking) external;
    function setTreasury(address treasury) external;
    function submitAggregatedResult(uint64 serviceId, uint64 callId, bytes memory output, uint256 signerBitmap, uint256[2] memory aggregatedSignature, uint256[4] memory aggregatedPubkey) external;
    function submitJob(uint64 serviceId, uint8 jobIndex, bytes memory inputs) external payable returns (uint64 callId);
    function submitJobs(uint64 serviceId, uint8[] memory jobIndices, bytes[] memory inputs) external payable returns (uint64[] memory callIds);
    function submitResult(uint64 serviceId, uint64 callId, bytes memory result) external;
    function submitResults(uint64 serviceId, uint64[] memory callIds, bytes[] memory results) external;
    function terminateService(uint64 serviceId) external;
    function transferBlueprint(uint64 blueprintId, address newOwner) external;
    function unpause() external;
    function unregisterOperator(uint64 blueprintId) external;
    function updateBlueprint(uint64 blueprintId, string memory metadataUri) external;
    function updateOperatorPreferences(uint64 blueprintId, bytes memory preferences) external;
}
```

...which was generated by the following JSON ABI:
```json
[
  {
    "type": "function",
    "name": "addPermittedCaller",
    "inputs": [
      {
        "name": "serviceId",
        "type": "uint64",
        "internalType": "uint64"
      },
      {
        "name": "caller",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "approveService",
    "inputs": [
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
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "approveServiceWithCommitments",
    "inputs": [
      {
        "name": "requestId",
        "type": "uint64",
        "internalType": "uint64"
      },
      {
        "name": "commitments",
        "type": "tuple[]",
        "internalType": "struct Types.AssetSecurityCommitment[]",
        "components": [
          {
            "name": "asset",
            "type": "tuple",
            "internalType": "struct Types.Asset",
            "components": [
              {
                "name": "kind",
                "type": "uint8",
                "internalType": "enum Types.AssetKind"
              },
              {
                "name": "token",
                "type": "address",
                "internalType": "address"
              }
            ]
          },
          {
            "name": "exposureBps",
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
    "name": "billSubscription",
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
    "name": "blueprintCount",
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
    "name": "blueprintOperatorCount",
    "inputs": [
      {
        "name": "blueprintId",
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
    "name": "claimRewards",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "createBlueprint",
    "inputs": [
      {
        "name": "metadataUri",
        "type": "string",
        "internalType": "string"
      },
      {
        "name": "manager",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [
      {
        "name": "blueprintId",
        "type": "uint64",
        "internalType": "uint64"
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "createBlueprintWithConfig",
    "inputs": [
      {
        "name": "metadataUri",
        "type": "string",
        "internalType": "string"
      },
      {
        "name": "manager",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "config",
        "type": "tuple",
        "internalType": "struct Types.BlueprintConfig",
        "components": [
          {
            "name": "membership",
            "type": "uint8",
            "internalType": "enum Types.MembershipModel"
          },
          {
            "name": "pricing",
            "type": "uint8",
            "internalType": "enum Types.PricingModel"
          },
          {
            "name": "minOperators",
            "type": "uint32",
            "internalType": "uint32"
          },
          {
            "name": "maxOperators",
            "type": "uint32",
            "internalType": "uint32"
          },
          {
            "name": "subscriptionRate",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "subscriptionInterval",
            "type": "uint64",
            "internalType": "uint64"
          },
          {
            "name": "eventRate",
            "type": "uint256",
            "internalType": "uint256"
          }
        ]
      }
    ],
    "outputs": [
      {
        "name": "blueprintId",
        "type": "uint64",
        "internalType": "uint64"
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "createServiceFromQuotes",
    "inputs": [
      {
        "name": "blueprintId",
        "type": "uint64",
        "internalType": "uint64"
      },
      {
        "name": "quotes",
        "type": "tuple[]",
        "internalType": "struct Types.SignedQuote[]",
        "components": [
          {
            "name": "details",
            "type": "tuple",
            "internalType": "struct Types.QuoteDetails",
            "components": [
              {
                "name": "blueprintId",
                "type": "uint64",
                "internalType": "uint64"
              },
              {
                "name": "ttlBlocks",
                "type": "uint64",
                "internalType": "uint64"
              },
              {
                "name": "totalCost",
                "type": "uint256",
                "internalType": "uint256"
              },
              {
                "name": "timestamp",
                "type": "uint64",
                "internalType": "uint64"
              },
              {
                "name": "expiry",
                "type": "uint64",
                "internalType": "uint64"
              },
              {
                "name": "securityCommitments",
                "type": "tuple[]",
                "internalType": "struct Types.AssetSecurityCommitment[]",
                "components": [
                  {
                    "name": "asset",
                    "type": "tuple",
                    "internalType": "struct Types.Asset",
                    "components": [
                      {
                        "name": "kind",
                        "type": "uint8",
                        "internalType": "enum Types.AssetKind"
                      },
                      {
                        "name": "token",
                        "type": "address",
                        "internalType": "address"
                      }
                    ]
                  },
                  {
                    "name": "exposureBps",
                    "type": "uint16",
                    "internalType": "uint16"
                  }
                ]
              }
            ]
          },
          {
            "name": "signature",
            "type": "bytes",
            "internalType": "bytes"
          },
          {
            "name": "operator",
            "type": "address",
            "internalType": "address"
          }
        ]
      },
      {
        "name": "config",
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
      }
    ],
    "outputs": [
      {
        "name": "serviceId",
        "type": "uint64",
        "internalType": "uint64"
      }
    ],
    "stateMutability": "payable"
  },
  {
    "type": "function",
    "name": "deactivateBlueprint",
    "inputs": [
      {
        "name": "blueprintId",
        "type": "uint64",
        "internalType": "uint64"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "getBlueprint",
    "inputs": [
      {
        "name": "blueprintId",
        "type": "uint64",
        "internalType": "uint64"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "tuple",
        "internalType": "struct Types.Blueprint",
        "components": [
          {
            "name": "owner",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "manager",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "createdAt",
            "type": "uint64",
            "internalType": "uint64"
          },
          {
            "name": "operatorCount",
            "type": "uint32",
            "internalType": "uint32"
          },
          {
            "name": "membership",
            "type": "uint8",
            "internalType": "enum Types.MembershipModel"
          },
          {
            "name": "pricing",
            "type": "uint8",
            "internalType": "enum Types.PricingModel"
          },
          {
            "name": "active",
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
    "name": "getBlueprintConfig",
    "inputs": [
      {
        "name": "blueprintId",
        "type": "uint64",
        "internalType": "uint64"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "tuple",
        "internalType": "struct Types.BlueprintConfig",
        "components": [
          {
            "name": "membership",
            "type": "uint8",
            "internalType": "enum Types.MembershipModel"
          },
          {
            "name": "pricing",
            "type": "uint8",
            "internalType": "enum Types.PricingModel"
          },
          {
            "name": "minOperators",
            "type": "uint32",
            "internalType": "uint32"
          },
          {
            "name": "maxOperators",
            "type": "uint32",
            "internalType": "uint32"
          },
          {
            "name": "subscriptionRate",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "subscriptionInterval",
            "type": "uint64",
            "internalType": "uint64"
          },
          {
            "name": "eventRate",
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
    "name": "getOperatorRegistration",
    "inputs": [
      {
        "name": "blueprintId",
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
        "internalType": "struct Types.OperatorRegistration",
        "components": [
          {
            "name": "registeredAt",
            "type": "uint64",
            "internalType": "uint64"
          },
          {
            "name": "updatedAt",
            "type": "uint64",
            "internalType": "uint64"
          },
          {
            "name": "active",
            "type": "bool",
            "internalType": "bool"
          },
          {
            "name": "online",
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
    "name": "getService",
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
        "internalType": "struct Types.Service",
        "components": [
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
            "name": "createdAt",
            "type": "uint64",
            "internalType": "uint64"
          },
          {
            "name": "ttl",
            "type": "uint64",
            "internalType": "uint64"
          },
          {
            "name": "terminatedAt",
            "type": "uint64",
            "internalType": "uint64"
          },
          {
            "name": "lastPaymentAt",
            "type": "uint64",
            "internalType": "uint64"
          },
          {
            "name": "operatorCount",
            "type": "uint32",
            "internalType": "uint32"
          },
          {
            "name": "minOperators",
            "type": "uint32",
            "internalType": "uint32"
          },
          {
            "name": "maxOperators",
            "type": "uint32",
            "internalType": "uint32"
          },
          {
            "name": "membership",
            "type": "uint8",
            "internalType": "enum Types.MembershipModel"
          },
          {
            "name": "pricing",
            "type": "uint8",
            "internalType": "enum Types.PricingModel"
          },
          {
            "name": "status",
            "type": "uint8",
            "internalType": "enum Types.ServiceStatus"
          }
        ]
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getServiceOperator",
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
        "internalType": "struct Types.ServiceOperator",
        "components": [
          {
            "name": "exposureBps",
            "type": "uint16",
            "internalType": "uint16"
          },
          {
            "name": "joinedAt",
            "type": "uint64",
            "internalType": "uint64"
          },
          {
            "name": "leftAt",
            "type": "uint64",
            "internalType": "uint64"
          },
          {
            "name": "active",
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
    "name": "getServiceOperators",
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
    "name": "getServiceRequest",
    "inputs": [
      {
        "name": "requestId",
        "type": "uint64",
        "internalType": "uint64"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "tuple",
        "internalType": "struct Types.ServiceRequest",
        "components": [
          {
            "name": "blueprintId",
            "type": "uint64",
            "internalType": "uint64"
          },
          {
            "name": "requester",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "createdAt",
            "type": "uint64",
            "internalType": "uint64"
          },
          {
            "name": "ttl",
            "type": "uint64",
            "internalType": "uint64"
          },
          {
            "name": "operatorCount",
            "type": "uint32",
            "internalType": "uint32"
          },
          {
            "name": "approvalCount",
            "type": "uint32",
            "internalType": "uint32"
          },
          {
            "name": "paymentToken",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "paymentAmount",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "membership",
            "type": "uint8",
            "internalType": "enum Types.MembershipModel"
          },
          {
            "name": "minOperators",
            "type": "uint32",
            "internalType": "uint32"
          },
          {
            "name": "maxOperators",
            "type": "uint32",
            "internalType": "uint32"
          },
          {
            "name": "rejected",
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
    "name": "getServiceTotalExposure",
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
    "name": "isOperatorRegistered",
    "inputs": [
      {
        "name": "blueprintId",
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
    "name": "isPermittedCaller",
    "inputs": [
      {
        "name": "serviceId",
        "type": "uint64",
        "internalType": "uint64"
      },
      {
        "name": "caller",
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
    "name": "isServiceActive",
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
        "type": "bool",
        "internalType": "bool"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "isServiceOperator",
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
    "name": "joinService",
    "inputs": [
      {
        "name": "serviceId",
        "type": "uint64",
        "internalType": "uint64"
      },
      {
        "name": "exposureBps",
        "type": "uint16",
        "internalType": "uint16"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "leaveService",
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
    "name": "pause",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "pendingRewards",
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
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "proposeSlash",
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
        "name": "amount",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "evidence",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "registerOperator",
    "inputs": [
      {
        "name": "blueprintId",
        "type": "uint64",
        "internalType": "uint64"
      },
      {
        "name": "preferences",
        "type": "bytes",
        "internalType": "bytes"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "rejectService",
    "inputs": [
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
    "name": "removePermittedCaller",
    "inputs": [
      {
        "name": "serviceId",
        "type": "uint64",
        "internalType": "uint64"
      },
      {
        "name": "caller",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "requestService",
    "inputs": [
      {
        "name": "blueprintId",
        "type": "uint64",
        "internalType": "uint64"
      },
      {
        "name": "operators",
        "type": "address[]",
        "internalType": "address[]"
      },
      {
        "name": "config",
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
        "name": "paymentToken",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "paymentAmount",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [
      {
        "name": "requestId",
        "type": "uint64",
        "internalType": "uint64"
      }
    ],
    "stateMutability": "payable"
  },
  {
    "type": "function",
    "name": "requestServiceWithExposure",
    "inputs": [
      {
        "name": "blueprintId",
        "type": "uint64",
        "internalType": "uint64"
      },
      {
        "name": "operators",
        "type": "address[]",
        "internalType": "address[]"
      },
      {
        "name": "exposureBps",
        "type": "uint16[]",
        "internalType": "uint16[]"
      },
      {
        "name": "config",
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
        "name": "paymentToken",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "paymentAmount",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [
      {
        "name": "requestId",
        "type": "uint64",
        "internalType": "uint64"
      }
    ],
    "stateMutability": "payable"
  },
  {
    "type": "function",
    "name": "requestServiceWithSecurity",
    "inputs": [
      {
        "name": "blueprintId",
        "type": "uint64",
        "internalType": "uint64"
      },
      {
        "name": "operators",
        "type": "address[]",
        "internalType": "address[]"
      },
      {
        "name": "securityRequirements",
        "type": "tuple[]",
        "internalType": "struct Types.AssetSecurityRequirement[]",
        "components": [
          {
            "name": "asset",
            "type": "tuple",
            "internalType": "struct Types.Asset",
            "components": [
              {
                "name": "kind",
                "type": "uint8",
                "internalType": "enum Types.AssetKind"
              },
              {
                "name": "token",
                "type": "address",
                "internalType": "address"
              }
            ]
          },
          {
            "name": "minExposureBps",
            "type": "uint16",
            "internalType": "uint16"
          },
          {
            "name": "maxExposureBps",
            "type": "uint16",
            "internalType": "uint16"
          }
        ]
      },
      {
        "name": "config",
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
        "name": "paymentToken",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "paymentAmount",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [
      {
        "name": "requestId",
        "type": "uint64",
        "internalType": "uint64"
      }
    ],
    "stateMutability": "payable"
  },
  {
    "type": "function",
    "name": "serviceCount",
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
    "name": "setOperatorOnline",
    "inputs": [
      {
        "name": "blueprintId",
        "type": "uint64",
        "internalType": "uint64"
      },
      {
        "name": "online",
        "type": "bool",
        "internalType": "bool"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "setPaymentSplit",
    "inputs": [
      {
        "name": "split",
        "type": "tuple",
        "internalType": "struct Types.PaymentSplit",
        "components": [
          {
            "name": "developerBps",
            "type": "uint16",
            "internalType": "uint16"
          },
          {
            "name": "protocolBps",
            "type": "uint16",
            "internalType": "uint16"
          },
          {
            "name": "operatorBps",
            "type": "uint16",
            "internalType": "uint16"
          },
          {
            "name": "restakerBps",
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
    "name": "setRestaking",
    "inputs": [
      {
        "name": "restaking",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "setTreasury",
    "inputs": [
      {
        "name": "treasury",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
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
    "type": "function",
    "name": "terminateService",
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
    "name": "transferBlueprint",
    "inputs": [
      {
        "name": "blueprintId",
        "type": "uint64",
        "internalType": "uint64"
      },
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
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "unregisterOperator",
    "inputs": [
      {
        "name": "blueprintId",
        "type": "uint64",
        "internalType": "uint64"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "updateBlueprint",
    "inputs": [
      {
        "name": "blueprintId",
        "type": "uint64",
        "internalType": "uint64"
      },
      {
        "name": "metadataUri",
        "type": "string",
        "internalType": "string"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "updateOperatorPreferences",
    "inputs": [
      {
        "name": "blueprintId",
        "type": "uint64",
        "internalType": "uint64"
      },
      {
        "name": "preferences",
        "type": "bytes",
        "internalType": "bytes"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "event",
    "name": "BlueprintCreated",
    "inputs": [
      {
        "name": "blueprintId",
        "type": "uint64",
        "indexed": true,
        "internalType": "uint64"
      },
      {
        "name": "owner",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "manager",
        "type": "address",
        "indexed": false,
        "internalType": "address"
      },
      {
        "name": "metadataUri",
        "type": "string",
        "indexed": false,
        "internalType": "string"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "BlueprintDeactivated",
    "inputs": [
      {
        "name": "blueprintId",
        "type": "uint64",
        "indexed": true,
        "internalType": "uint64"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "BlueprintTransferred",
    "inputs": [
      {
        "name": "blueprintId",
        "type": "uint64",
        "indexed": true,
        "internalType": "uint64"
      },
      {
        "name": "from",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "to",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "BlueprintUpdated",
    "inputs": [
      {
        "name": "blueprintId",
        "type": "uint64",
        "indexed": true,
        "internalType": "uint64"
      },
      {
        "name": "metadataUri",
        "type": "string",
        "indexed": false,
        "internalType": "string"
      }
    ],
    "anonymous": false
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
  },
  {
    "type": "event",
    "name": "OperatorJoinedService",
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
        "name": "exposureBps",
        "type": "uint16",
        "indexed": false,
        "internalType": "uint16"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "OperatorLeftService",
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
    "name": "OperatorOnlineStatusChanged",
    "inputs": [
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
        "name": "online",
        "type": "bool",
        "indexed": false,
        "internalType": "bool"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "OperatorPreferencesUpdated",
    "inputs": [
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
        "name": "preferences",
        "type": "bytes",
        "indexed": false,
        "internalType": "bytes"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "OperatorRegistered",
    "inputs": [
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
        "name": "preferences",
        "type": "bytes",
        "indexed": false,
        "internalType": "bytes"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "OperatorUnregistered",
    "inputs": [
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
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "RewardsClaimed",
    "inputs": [
      {
        "name": "account",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "amount",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "RewardsDistributed",
    "inputs": [
      {
        "name": "serviceId",
        "type": "uint64",
        "indexed": true,
        "internalType": "uint64"
      },
      {
        "name": "developerAmount",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "protocolAmount",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "operatorAmount",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "restakerAmount",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "ServiceActivated",
    "inputs": [
      {
        "name": "serviceId",
        "type": "uint64",
        "indexed": true,
        "internalType": "uint64"
      },
      {
        "name": "requestId",
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
        "name": "owner",
        "type": "address",
        "indexed": false,
        "internalType": "address"
      },
      {
        "name": "operators",
        "type": "address[]",
        "indexed": false,
        "internalType": "address[]"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "ServiceApproved",
    "inputs": [
      {
        "name": "requestId",
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
    "name": "ServiceCreatedFromQuotes",
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
        "name": "owner",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "operators",
        "type": "address[]",
        "indexed": false,
        "internalType": "address[]"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "ServiceRejected",
    "inputs": [
      {
        "name": "requestId",
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
    "name": "ServiceRequested",
    "inputs": [
      {
        "name": "requestId",
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
        "name": "requester",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "operators",
        "type": "address[]",
        "indexed": false,
        "internalType": "address[]"
      },
      {
        "name": "config",
        "type": "bytes",
        "indexed": false,
        "internalType": "bytes"
      },
      {
        "name": "ttl",
        "type": "uint64",
        "indexed": false,
        "internalType": "uint64"
      },
      {
        "name": "paymentToken",
        "type": "address",
        "indexed": false,
        "internalType": "address"
      },
      {
        "name": "paymentAmount",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "ServiceRequestedWithSecurity",
    "inputs": [
      {
        "name": "requestId",
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
        "name": "requester",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "operators",
        "type": "address[]",
        "indexed": false,
        "internalType": "address[]"
      },
      {
        "name": "securityRequirements",
        "type": "tuple[]",
        "indexed": false,
        "internalType": "struct Types.AssetSecurityRequirement[]",
        "components": [
          {
            "name": "asset",
            "type": "tuple",
            "internalType": "struct Types.Asset",
            "components": [
              {
                "name": "kind",
                "type": "uint8",
                "internalType": "enum Types.AssetKind"
              },
              {
                "name": "token",
                "type": "address",
                "internalType": "address"
              }
            ]
          },
          {
            "name": "minExposureBps",
            "type": "uint16",
            "internalType": "uint16"
          },
          {
            "name": "maxExposureBps",
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
    "name": "ServiceTerminated",
    "inputs": [
      {
        "name": "serviceId",
        "type": "uint64",
        "indexed": true,
        "internalType": "uint64"
      },
      {
        "name": "owner",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "SlashExecuted",
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
        "name": "amount",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "SlashProposed",
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
        "name": "amount",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "evidence",
        "type": "bytes32",
        "indexed": false,
        "internalType": "bytes32"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "SubscriptionBilled",
    "inputs": [
      {
        "name": "serviceId",
        "type": "uint64",
        "indexed": true,
        "internalType": "uint64"
      },
      {
        "name": "amount",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "billedAt",
        "type": "uint64",
        "indexed": false,
        "internalType": "uint64"
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
pub mod ITangleFull {
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
    /**Event with signature `BlueprintCreated(uint64,address,address,string)` and selector `0x333da2452b770d8297cd9630b93a2a9190c82483327a16472f621efc11ace010`.
```solidity
event BlueprintCreated(uint64 indexed blueprintId, address indexed owner, address manager, string metadataUri);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct BlueprintCreated {
        #[allow(missing_docs)]
        pub blueprintId: u64,
        #[allow(missing_docs)]
        pub owner: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub manager: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub metadataUri: alloy::sol_types::private::String,
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
        impl alloy_sol_types::SolEvent for BlueprintCreated {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::String,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<64>,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "BlueprintCreated(uint64,address,address,string)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                51u8,
                61u8,
                162u8,
                69u8,
                43u8,
                119u8,
                13u8,
                130u8,
                151u8,
                205u8,
                150u8,
                48u8,
                185u8,
                58u8,
                42u8,
                145u8,
                144u8,
                200u8,
                36u8,
                131u8,
                50u8,
                122u8,
                22u8,
                71u8,
                47u8,
                98u8,
                30u8,
                252u8,
                17u8,
                172u8,
                224u8,
                16u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    blueprintId: topics.1,
                    owner: topics.2,
                    manager: data.0,
                    metadataUri: data.1,
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
                        &self.manager,
                    ),
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        &self.metadataUri,
                    ),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (
                    Self::SIGNATURE_HASH.into(),
                    self.blueprintId.clone(),
                    self.owner.clone(),
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
                > as alloy_sol_types::EventTopic>::encode_topic(&self.blueprintId);
                out[2usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.owner,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for BlueprintCreated {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&BlueprintCreated> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &BlueprintCreated) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `BlueprintDeactivated(uint64)` and selector `0xe14286f3ad49ada6d0911adda8ef90616999045bde2a33e391a7b5ae6589e789`.
```solidity
event BlueprintDeactivated(uint64 indexed blueprintId);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct BlueprintDeactivated {
        #[allow(missing_docs)]
        pub blueprintId: u64,
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
        impl alloy_sol_types::SolEvent for BlueprintDeactivated {
            type DataTuple<'a> = ();
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<64>,
            );
            const SIGNATURE: &'static str = "BlueprintDeactivated(uint64)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                225u8,
                66u8,
                134u8,
                243u8,
                173u8,
                73u8,
                173u8,
                166u8,
                208u8,
                145u8,
                26u8,
                221u8,
                168u8,
                239u8,
                144u8,
                97u8,
                105u8,
                153u8,
                4u8,
                91u8,
                222u8,
                42u8,
                51u8,
                227u8,
                145u8,
                167u8,
                181u8,
                174u8,
                101u8,
                137u8,
                231u8,
                137u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { blueprintId: topics.1 }
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
                (Self::SIGNATURE_HASH.into(), self.blueprintId.clone())
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
                > as alloy_sol_types::EventTopic>::encode_topic(&self.blueprintId);
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for BlueprintDeactivated {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&BlueprintDeactivated> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &BlueprintDeactivated) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `BlueprintTransferred(uint64,address,address)` and selector `0xd2b958a1b0b978ceded4986b4f49c8b25fbd6e812cb8925e13efc731ea4aa24d`.
```solidity
event BlueprintTransferred(uint64 indexed blueprintId, address indexed from, address indexed to);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct BlueprintTransferred {
        #[allow(missing_docs)]
        pub blueprintId: u64,
        #[allow(missing_docs)]
        pub from: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub to: alloy::sol_types::private::Address,
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
        impl alloy_sol_types::SolEvent for BlueprintTransferred {
            type DataTuple<'a> = ();
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<64>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "BlueprintTransferred(uint64,address,address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                210u8,
                185u8,
                88u8,
                161u8,
                176u8,
                185u8,
                120u8,
                206u8,
                222u8,
                212u8,
                152u8,
                107u8,
                79u8,
                73u8,
                200u8,
                178u8,
                95u8,
                189u8,
                110u8,
                129u8,
                44u8,
                184u8,
                146u8,
                94u8,
                19u8,
                239u8,
                199u8,
                49u8,
                234u8,
                74u8,
                162u8,
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
                    blueprintId: topics.1,
                    from: topics.2,
                    to: topics.3,
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
                    self.blueprintId.clone(),
                    self.from.clone(),
                    self.to.clone(),
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
                > as alloy_sol_types::EventTopic>::encode_topic(&self.blueprintId);
                out[2usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.from,
                );
                out[3usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.to,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for BlueprintTransferred {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&BlueprintTransferred> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &BlueprintTransferred) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `BlueprintUpdated(uint64,string)` and selector `0x30fc45e05a33fd9068cf6ae1a8f3db2b9d15372dd6494eccc8cd5d305fdf38b3`.
```solidity
event BlueprintUpdated(uint64 indexed blueprintId, string metadataUri);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct BlueprintUpdated {
        #[allow(missing_docs)]
        pub blueprintId: u64,
        #[allow(missing_docs)]
        pub metadataUri: alloy::sol_types::private::String,
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
        impl alloy_sol_types::SolEvent for BlueprintUpdated {
            type DataTuple<'a> = (alloy::sol_types::sol_data::String,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<64>,
            );
            const SIGNATURE: &'static str = "BlueprintUpdated(uint64,string)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                48u8,
                252u8,
                69u8,
                224u8,
                90u8,
                51u8,
                253u8,
                144u8,
                104u8,
                207u8,
                106u8,
                225u8,
                168u8,
                243u8,
                219u8,
                43u8,
                157u8,
                21u8,
                55u8,
                45u8,
                214u8,
                73u8,
                78u8,
                204u8,
                200u8,
                205u8,
                93u8,
                48u8,
                95u8,
                223u8,
                56u8,
                179u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    blueprintId: topics.1,
                    metadataUri: data.0,
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
                        &self.metadataUri,
                    ),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.blueprintId.clone())
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
                > as alloy_sol_types::EventTopic>::encode_topic(&self.blueprintId);
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for BlueprintUpdated {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&BlueprintUpdated> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &BlueprintUpdated) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
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
    /**Event with signature `OperatorJoinedService(uint64,address,uint16)` and selector `0x2457918796078440068385e73e422539c84f2f9045dd442e123d4284258ae24a`.
```solidity
event OperatorJoinedService(uint64 indexed serviceId, address indexed operator, uint16 exposureBps);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct OperatorJoinedService {
        #[allow(missing_docs)]
        pub serviceId: u64,
        #[allow(missing_docs)]
        pub operator: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub exposureBps: u16,
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
        impl alloy_sol_types::SolEvent for OperatorJoinedService {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Uint<16>,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<64>,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "OperatorJoinedService(uint64,address,uint16)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                36u8,
                87u8,
                145u8,
                135u8,
                150u8,
                7u8,
                132u8,
                64u8,
                6u8,
                131u8,
                133u8,
                231u8,
                62u8,
                66u8,
                37u8,
                57u8,
                200u8,
                79u8,
                47u8,
                144u8,
                69u8,
                221u8,
                68u8,
                46u8,
                18u8,
                61u8,
                66u8,
                132u8,
                37u8,
                138u8,
                226u8,
                74u8,
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
                    exposureBps: data.0,
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
                        16,
                    > as alloy_sol_types::SolType>::tokenize(&self.exposureBps),
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
        impl alloy_sol_types::private::IntoLogData for OperatorJoinedService {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&OperatorJoinedService> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &OperatorJoinedService) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `OperatorLeftService(uint64,address)` and selector `0x8ce53e4b6002ff090e641ddecdaf7edd2813a98e19139fa7d1c79fccc3c4dead`.
```solidity
event OperatorLeftService(uint64 indexed serviceId, address indexed operator);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct OperatorLeftService {
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
        impl alloy_sol_types::SolEvent for OperatorLeftService {
            type DataTuple<'a> = ();
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<64>,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "OperatorLeftService(uint64,address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                140u8,
                229u8,
                62u8,
                75u8,
                96u8,
                2u8,
                255u8,
                9u8,
                14u8,
                100u8,
                29u8,
                222u8,
                205u8,
                175u8,
                126u8,
                221u8,
                40u8,
                19u8,
                169u8,
                142u8,
                25u8,
                19u8,
                159u8,
                167u8,
                209u8,
                199u8,
                159u8,
                204u8,
                195u8,
                196u8,
                222u8,
                173u8,
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
        impl alloy_sol_types::private::IntoLogData for OperatorLeftService {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&OperatorLeftService> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &OperatorLeftService) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `OperatorOnlineStatusChanged(uint64,address,bool)` and selector `0xd40c06eeb20d219d2e33a6df52a06b3ed1fa0ddc220478ac98e00464e483495b`.
```solidity
event OperatorOnlineStatusChanged(uint64 indexed blueprintId, address indexed operator, bool online);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct OperatorOnlineStatusChanged {
        #[allow(missing_docs)]
        pub blueprintId: u64,
        #[allow(missing_docs)]
        pub operator: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub online: bool,
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
        impl alloy_sol_types::SolEvent for OperatorOnlineStatusChanged {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<64>,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "OperatorOnlineStatusChanged(uint64,address,bool)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                212u8,
                12u8,
                6u8,
                238u8,
                178u8,
                13u8,
                33u8,
                157u8,
                46u8,
                51u8,
                166u8,
                223u8,
                82u8,
                160u8,
                107u8,
                62u8,
                209u8,
                250u8,
                13u8,
                220u8,
                34u8,
                4u8,
                120u8,
                172u8,
                152u8,
                224u8,
                4u8,
                100u8,
                228u8,
                131u8,
                73u8,
                91u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    blueprintId: topics.1,
                    operator: topics.2,
                    online: data.0,
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
                    <alloy::sol_types::sol_data::Bool as alloy_sol_types::SolType>::tokenize(
                        &self.online,
                    ),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (
                    Self::SIGNATURE_HASH.into(),
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
                > as alloy_sol_types::EventTopic>::encode_topic(&self.blueprintId);
                out[2usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.operator,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for OperatorOnlineStatusChanged {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&OperatorOnlineStatusChanged> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(
                this: &OperatorOnlineStatusChanged,
            ) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `OperatorPreferencesUpdated(uint64,address,bytes)` and selector `0x8547e94bd95ca7d3c7a3be9b66a29e6f36151be54a2563e35098438861a4c871`.
```solidity
event OperatorPreferencesUpdated(uint64 indexed blueprintId, address indexed operator, bytes preferences);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct OperatorPreferencesUpdated {
        #[allow(missing_docs)]
        pub blueprintId: u64,
        #[allow(missing_docs)]
        pub operator: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub preferences: alloy::sol_types::private::Bytes,
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
        impl alloy_sol_types::SolEvent for OperatorPreferencesUpdated {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Bytes,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<64>,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "OperatorPreferencesUpdated(uint64,address,bytes)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                133u8,
                71u8,
                233u8,
                75u8,
                217u8,
                92u8,
                167u8,
                211u8,
                199u8,
                163u8,
                190u8,
                155u8,
                102u8,
                162u8,
                158u8,
                111u8,
                54u8,
                21u8,
                27u8,
                229u8,
                74u8,
                37u8,
                99u8,
                227u8,
                80u8,
                152u8,
                67u8,
                136u8,
                97u8,
                164u8,
                200u8,
                113u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    blueprintId: topics.1,
                    operator: topics.2,
                    preferences: data.0,
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
                        &self.preferences,
                    ),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (
                    Self::SIGNATURE_HASH.into(),
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
                > as alloy_sol_types::EventTopic>::encode_topic(&self.blueprintId);
                out[2usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.operator,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for OperatorPreferencesUpdated {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&OperatorPreferencesUpdated> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(
                this: &OperatorPreferencesUpdated,
            ) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `OperatorRegistered(uint64,address,bytes)` and selector `0xa9cf99c4c75bf0874576e6ee70b00b920e9c36551e4d6c17694ecaa684624d2b`.
```solidity
event OperatorRegistered(uint64 indexed blueprintId, address indexed operator, bytes preferences);
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
        pub blueprintId: u64,
        #[allow(missing_docs)]
        pub operator: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub preferences: alloy::sol_types::private::Bytes,
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
            type DataTuple<'a> = (alloy::sol_types::sol_data::Bytes,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<64>,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "OperatorRegistered(uint64,address,bytes)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                169u8,
                207u8,
                153u8,
                196u8,
                199u8,
                91u8,
                240u8,
                135u8,
                69u8,
                118u8,
                230u8,
                238u8,
                112u8,
                176u8,
                11u8,
                146u8,
                14u8,
                156u8,
                54u8,
                85u8,
                30u8,
                77u8,
                108u8,
                23u8,
                105u8,
                78u8,
                202u8,
                166u8,
                132u8,
                98u8,
                77u8,
                43u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    blueprintId: topics.1,
                    operator: topics.2,
                    preferences: data.0,
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
                        &self.preferences,
                    ),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (
                    Self::SIGNATURE_HASH.into(),
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
                > as alloy_sol_types::EventTopic>::encode_topic(&self.blueprintId);
                out[2usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
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
    /**Event with signature `OperatorUnregistered(uint64,address)` and selector `0x88e46e95bcdf1020c0caf9f3dd5fde69c41d2151dbbca0a77c03c4e28b4d3ec1`.
```solidity
event OperatorUnregistered(uint64 indexed blueprintId, address indexed operator);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct OperatorUnregistered {
        #[allow(missing_docs)]
        pub blueprintId: u64,
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
        impl alloy_sol_types::SolEvent for OperatorUnregistered {
            type DataTuple<'a> = ();
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<64>,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "OperatorUnregistered(uint64,address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                136u8,
                228u8,
                110u8,
                149u8,
                188u8,
                223u8,
                16u8,
                32u8,
                192u8,
                202u8,
                249u8,
                243u8,
                221u8,
                95u8,
                222u8,
                105u8,
                196u8,
                29u8,
                33u8,
                81u8,
                219u8,
                188u8,
                160u8,
                167u8,
                124u8,
                3u8,
                196u8,
                226u8,
                139u8,
                77u8,
                62u8,
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
                    blueprintId: topics.1,
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
                > as alloy_sol_types::EventTopic>::encode_topic(&self.blueprintId);
                out[2usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.operator,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for OperatorUnregistered {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&OperatorUnregistered> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &OperatorUnregistered) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `RewardsClaimed(address,uint256)` and selector `0xfc30cddea38e2bf4d6ea7d3f9ed3b6ad7f176419f4963bd81318067a4aee73fe`.
```solidity
event RewardsClaimed(address indexed account, uint256 amount);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct RewardsClaimed {
        #[allow(missing_docs)]
        pub account: alloy::sol_types::private::Address,
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
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for RewardsClaimed {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "RewardsClaimed(address,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                252u8,
                48u8,
                205u8,
                222u8,
                163u8,
                142u8,
                43u8,
                244u8,
                214u8,
                234u8,
                125u8,
                63u8,
                158u8,
                211u8,
                182u8,
                173u8,
                127u8,
                23u8,
                100u8,
                25u8,
                244u8,
                150u8,
                59u8,
                216u8,
                19u8,
                24u8,
                6u8,
                122u8,
                74u8,
                238u8,
                115u8,
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
                    account: topics.1,
                    amount: data.0,
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
                    > as alloy_sol_types::SolType>::tokenize(&self.amount),
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
        impl alloy_sol_types::private::IntoLogData for RewardsClaimed {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&RewardsClaimed> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &RewardsClaimed) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `RewardsDistributed(uint64,uint256,uint256,uint256,uint256)` and selector `0xefae2b43c59cbab6c92bbbe3f44a6e73cc9ccbbf94065cdc388297a541c9976e`.
```solidity
event RewardsDistributed(uint64 indexed serviceId, uint256 developerAmount, uint256 protocolAmount, uint256 operatorAmount, uint256 restakerAmount);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct RewardsDistributed {
        #[allow(missing_docs)]
        pub serviceId: u64,
        #[allow(missing_docs)]
        pub developerAmount: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub protocolAmount: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub operatorAmount: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub restakerAmount: alloy::sol_types::private::primitives::aliases::U256,
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
        impl alloy_sol_types::SolEvent for RewardsDistributed {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<64>,
            );
            const SIGNATURE: &'static str = "RewardsDistributed(uint64,uint256,uint256,uint256,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                239u8,
                174u8,
                43u8,
                67u8,
                197u8,
                156u8,
                186u8,
                182u8,
                201u8,
                43u8,
                187u8,
                227u8,
                244u8,
                74u8,
                110u8,
                115u8,
                204u8,
                156u8,
                203u8,
                191u8,
                148u8,
                6u8,
                92u8,
                220u8,
                56u8,
                130u8,
                151u8,
                165u8,
                65u8,
                201u8,
                151u8,
                110u8,
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
                    developerAmount: data.0,
                    protocolAmount: data.1,
                    operatorAmount: data.2,
                    restakerAmount: data.3,
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
                    > as alloy_sol_types::SolType>::tokenize(&self.developerAmount),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.protocolAmount),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.operatorAmount),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.restakerAmount),
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
        impl alloy_sol_types::private::IntoLogData for RewardsDistributed {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&RewardsDistributed> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &RewardsDistributed) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `ServiceActivated(uint64,uint64,uint64,address,address[])` and selector `0x93003b4812f9ce52c058cbded90d70bf175b4f77d5998eaf9f263c72fdcc87ca`.
```solidity
event ServiceActivated(uint64 indexed serviceId, uint64 indexed requestId, uint64 indexed blueprintId, address owner, address[] operators);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct ServiceActivated {
        #[allow(missing_docs)]
        pub serviceId: u64,
        #[allow(missing_docs)]
        pub requestId: u64,
        #[allow(missing_docs)]
        pub blueprintId: u64,
        #[allow(missing_docs)]
        pub owner: alloy::sol_types::private::Address,
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
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for ServiceActivated {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<64>,
                alloy::sol_types::sol_data::Uint<64>,
                alloy::sol_types::sol_data::Uint<64>,
            );
            const SIGNATURE: &'static str = "ServiceActivated(uint64,uint64,uint64,address,address[])";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                147u8,
                0u8,
                59u8,
                72u8,
                18u8,
                249u8,
                206u8,
                82u8,
                192u8,
                88u8,
                203u8,
                222u8,
                217u8,
                13u8,
                112u8,
                191u8,
                23u8,
                91u8,
                79u8,
                119u8,
                213u8,
                153u8,
                142u8,
                175u8,
                159u8,
                38u8,
                60u8,
                114u8,
                253u8,
                204u8,
                135u8,
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
                    serviceId: topics.1,
                    requestId: topics.2,
                    blueprintId: topics.3,
                    owner: data.0,
                    operators: data.1,
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
                        &self.owner,
                    ),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Address,
                    > as alloy_sol_types::SolType>::tokenize(&self.operators),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (
                    Self::SIGNATURE_HASH.into(),
                    self.serviceId.clone(),
                    self.requestId.clone(),
                    self.blueprintId.clone(),
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
                > as alloy_sol_types::EventTopic>::encode_topic(&self.requestId);
                out[3usize] = <alloy::sol_types::sol_data::Uint<
                    64,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.blueprintId);
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for ServiceActivated {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&ServiceActivated> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &ServiceActivated) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `ServiceApproved(uint64,address)` and selector `0xceda1a44ead4da3d3abaaa941afa6d9530fd84dd8e99385c9183d986dcab980f`.
```solidity
event ServiceApproved(uint64 indexed requestId, address indexed operator);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct ServiceApproved {
        #[allow(missing_docs)]
        pub requestId: u64,
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
        impl alloy_sol_types::SolEvent for ServiceApproved {
            type DataTuple<'a> = ();
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<64>,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "ServiceApproved(uint64,address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                206u8,
                218u8,
                26u8,
                68u8,
                234u8,
                212u8,
                218u8,
                61u8,
                58u8,
                186u8,
                170u8,
                148u8,
                26u8,
                250u8,
                109u8,
                149u8,
                48u8,
                253u8,
                132u8,
                221u8,
                142u8,
                153u8,
                56u8,
                92u8,
                145u8,
                131u8,
                217u8,
                134u8,
                220u8,
                171u8,
                152u8,
                15u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    requestId: topics.1,
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
                    self.requestId.clone(),
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
                > as alloy_sol_types::EventTopic>::encode_topic(&self.requestId);
                out[2usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.operator,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for ServiceApproved {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&ServiceApproved> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &ServiceApproved) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `ServiceCreatedFromQuotes(uint64,uint64,address,address[])` and selector `0x0fb6ea41be7e4b4d030ff394e280973a01fb8e2ca9da7c900ae309669a177519`.
```solidity
event ServiceCreatedFromQuotes(uint64 indexed serviceId, uint64 indexed blueprintId, address indexed owner, address[] operators);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct ServiceCreatedFromQuotes {
        #[allow(missing_docs)]
        pub serviceId: u64,
        #[allow(missing_docs)]
        pub blueprintId: u64,
        #[allow(missing_docs)]
        pub owner: alloy::sol_types::private::Address,
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
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for ServiceCreatedFromQuotes {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
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
            const SIGNATURE: &'static str = "ServiceCreatedFromQuotes(uint64,uint64,address,address[])";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                15u8,
                182u8,
                234u8,
                65u8,
                190u8,
                126u8,
                75u8,
                77u8,
                3u8,
                15u8,
                243u8,
                148u8,
                226u8,
                128u8,
                151u8,
                58u8,
                1u8,
                251u8,
                142u8,
                44u8,
                169u8,
                218u8,
                124u8,
                144u8,
                10u8,
                227u8,
                9u8,
                102u8,
                154u8,
                23u8,
                117u8,
                25u8,
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
                    owner: topics.3,
                    operators: data.0,
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
                        alloy::sol_types::sol_data::Address,
                    > as alloy_sol_types::SolType>::tokenize(&self.operators),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (
                    Self::SIGNATURE_HASH.into(),
                    self.serviceId.clone(),
                    self.blueprintId.clone(),
                    self.owner.clone(),
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
                    &self.owner,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for ServiceCreatedFromQuotes {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&ServiceCreatedFromQuotes> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(
                this: &ServiceCreatedFromQuotes,
            ) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `ServiceRejected(uint64,address)` and selector `0xc63050b963ec6e2cf36df90ecacaa43212c0e74b8c3a58a0f8cce3bd5a2a7675`.
```solidity
event ServiceRejected(uint64 indexed requestId, address indexed operator);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct ServiceRejected {
        #[allow(missing_docs)]
        pub requestId: u64,
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
        impl alloy_sol_types::SolEvent for ServiceRejected {
            type DataTuple<'a> = ();
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<64>,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "ServiceRejected(uint64,address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                198u8,
                48u8,
                80u8,
                185u8,
                99u8,
                236u8,
                110u8,
                44u8,
                243u8,
                109u8,
                249u8,
                14u8,
                202u8,
                202u8,
                164u8,
                50u8,
                18u8,
                192u8,
                231u8,
                75u8,
                140u8,
                58u8,
                88u8,
                160u8,
                248u8,
                204u8,
                227u8,
                189u8,
                90u8,
                42u8,
                118u8,
                117u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    requestId: topics.1,
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
                    self.requestId.clone(),
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
                > as alloy_sol_types::EventTopic>::encode_topic(&self.requestId);
                out[2usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.operator,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for ServiceRejected {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&ServiceRejected> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &ServiceRejected) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `ServiceRequested(uint64,uint64,address,address[],bytes,uint64,address,uint256)` and selector `0xf0d5943a10a17d85e69b14bab3db69ceff3f919bb7fcf79aafeffc39ba1a7eaf`.
```solidity
event ServiceRequested(uint64 indexed requestId, uint64 indexed blueprintId, address indexed requester, address[] operators, bytes config, uint64 ttl, address paymentToken, uint256 paymentAmount);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct ServiceRequested {
        #[allow(missing_docs)]
        pub requestId: u64,
        #[allow(missing_docs)]
        pub blueprintId: u64,
        #[allow(missing_docs)]
        pub requester: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub operators: alloy::sol_types::private::Vec<
            alloy::sol_types::private::Address,
        >,
        #[allow(missing_docs)]
        pub config: alloy::sol_types::private::Bytes,
        #[allow(missing_docs)]
        pub ttl: u64,
        #[allow(missing_docs)]
        pub paymentToken: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub paymentAmount: alloy::sol_types::private::primitives::aliases::U256,
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
        impl alloy_sol_types::SolEvent for ServiceRequested {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
                alloy::sol_types::sol_data::Bytes,
                alloy::sol_types::sol_data::Uint<64>,
                alloy::sol_types::sol_data::Address,
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
            const SIGNATURE: &'static str = "ServiceRequested(uint64,uint64,address,address[],bytes,uint64,address,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                240u8,
                213u8,
                148u8,
                58u8,
                16u8,
                161u8,
                125u8,
                133u8,
                230u8,
                155u8,
                20u8,
                186u8,
                179u8,
                219u8,
                105u8,
                206u8,
                255u8,
                63u8,
                145u8,
                155u8,
                183u8,
                252u8,
                247u8,
                154u8,
                175u8,
                239u8,
                252u8,
                57u8,
                186u8,
                26u8,
                126u8,
                175u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    requestId: topics.1,
                    blueprintId: topics.2,
                    requester: topics.3,
                    operators: data.0,
                    config: data.1,
                    ttl: data.2,
                    paymentToken: data.3,
                    paymentAmount: data.4,
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
                        alloy::sol_types::sol_data::Address,
                    > as alloy_sol_types::SolType>::tokenize(&self.operators),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.config,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::tokenize(&self.ttl),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.paymentToken,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.paymentAmount),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (
                    Self::SIGNATURE_HASH.into(),
                    self.requestId.clone(),
                    self.blueprintId.clone(),
                    self.requester.clone(),
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
                > as alloy_sol_types::EventTopic>::encode_topic(&self.requestId);
                out[2usize] = <alloy::sol_types::sol_data::Uint<
                    64,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.blueprintId);
                out[3usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.requester,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for ServiceRequested {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&ServiceRequested> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &ServiceRequested) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `ServiceRequestedWithSecurity(uint64,uint64,address,address[],((uint8,address),uint16,uint16)[])` and selector `0xe9b9d941d31762ebeb7a00008b4cf95e9a19255cb6695c9d35215c914a7e8486`.
```solidity
event ServiceRequestedWithSecurity(uint64 indexed requestId, uint64 indexed blueprintId, address indexed requester, address[] operators, Types.AssetSecurityRequirement[] securityRequirements);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct ServiceRequestedWithSecurity {
        #[allow(missing_docs)]
        pub requestId: u64,
        #[allow(missing_docs)]
        pub blueprintId: u64,
        #[allow(missing_docs)]
        pub requester: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub operators: alloy::sol_types::private::Vec<
            alloy::sol_types::private::Address,
        >,
        #[allow(missing_docs)]
        pub securityRequirements: alloy::sol_types::private::Vec<
            <Types::AssetSecurityRequirement as alloy::sol_types::SolType>::RustType,
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
        impl alloy_sol_types::SolEvent for ServiceRequestedWithSecurity {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
                alloy::sol_types::sol_data::Array<Types::AssetSecurityRequirement>,
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
            const SIGNATURE: &'static str = "ServiceRequestedWithSecurity(uint64,uint64,address,address[],((uint8,address),uint16,uint16)[])";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                233u8,
                185u8,
                217u8,
                65u8,
                211u8,
                23u8,
                98u8,
                235u8,
                235u8,
                122u8,
                0u8,
                0u8,
                139u8,
                76u8,
                249u8,
                94u8,
                154u8,
                25u8,
                37u8,
                92u8,
                182u8,
                105u8,
                92u8,
                157u8,
                53u8,
                33u8,
                92u8,
                145u8,
                74u8,
                126u8,
                132u8,
                134u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    requestId: topics.1,
                    blueprintId: topics.2,
                    requester: topics.3,
                    operators: data.0,
                    securityRequirements: data.1,
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
                        alloy::sol_types::sol_data::Address,
                    > as alloy_sol_types::SolType>::tokenize(&self.operators),
                    <alloy::sol_types::sol_data::Array<
                        Types::AssetSecurityRequirement,
                    > as alloy_sol_types::SolType>::tokenize(&self.securityRequirements),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (
                    Self::SIGNATURE_HASH.into(),
                    self.requestId.clone(),
                    self.blueprintId.clone(),
                    self.requester.clone(),
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
                > as alloy_sol_types::EventTopic>::encode_topic(&self.requestId);
                out[2usize] = <alloy::sol_types::sol_data::Uint<
                    64,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.blueprintId);
                out[3usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.requester,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for ServiceRequestedWithSecurity {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&ServiceRequestedWithSecurity> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(
                this: &ServiceRequestedWithSecurity,
            ) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `ServiceTerminated(uint64,address)` and selector `0x946bc863ee08c6c14af4e70163bd1597c71122f2f0589abce60a78e01e8cfb74`.
```solidity
event ServiceTerminated(uint64 indexed serviceId, address indexed owner);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct ServiceTerminated {
        #[allow(missing_docs)]
        pub serviceId: u64,
        #[allow(missing_docs)]
        pub owner: alloy::sol_types::private::Address,
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
        impl alloy_sol_types::SolEvent for ServiceTerminated {
            type DataTuple<'a> = ();
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<64>,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "ServiceTerminated(uint64,address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                148u8,
                107u8,
                200u8,
                99u8,
                238u8,
                8u8,
                198u8,
                193u8,
                74u8,
                244u8,
                231u8,
                1u8,
                99u8,
                189u8,
                21u8,
                151u8,
                199u8,
                17u8,
                34u8,
                242u8,
                240u8,
                88u8,
                154u8,
                188u8,
                230u8,
                10u8,
                120u8,
                224u8,
                30u8,
                140u8,
                251u8,
                116u8,
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
                    owner: topics.2,
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
                (Self::SIGNATURE_HASH.into(), self.serviceId.clone(), self.owner.clone())
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
                    &self.owner,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for ServiceTerminated {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&ServiceTerminated> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &ServiceTerminated) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `SlashExecuted(uint64,address,uint256)` and selector `0x26c5c426df452d68e3b3d27c7b069a62cd268ad9e1cd55c9c9f239320890de23`.
```solidity
event SlashExecuted(uint64 indexed serviceId, address indexed operator, uint256 amount);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct SlashExecuted {
        #[allow(missing_docs)]
        pub serviceId: u64,
        #[allow(missing_docs)]
        pub operator: alloy::sol_types::private::Address,
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
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for SlashExecuted {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<64>,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "SlashExecuted(uint64,address,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                38u8,
                197u8,
                196u8,
                38u8,
                223u8,
                69u8,
                45u8,
                104u8,
                227u8,
                179u8,
                210u8,
                124u8,
                123u8,
                6u8,
                154u8,
                98u8,
                205u8,
                38u8,
                138u8,
                217u8,
                225u8,
                205u8,
                85u8,
                201u8,
                201u8,
                242u8,
                57u8,
                50u8,
                8u8,
                144u8,
                222u8,
                35u8,
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
                    amount: data.0,
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
                    > as alloy_sol_types::SolType>::tokenize(&self.amount),
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
        impl alloy_sol_types::private::IntoLogData for SlashExecuted {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&SlashExecuted> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &SlashExecuted) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `SlashProposed(uint64,address,uint256,bytes32)` and selector `0xf4ad7d6e9772d592d2a6c9449d8cc40b67b9f0e4b81c3c76979c799b3beb34c2`.
```solidity
event SlashProposed(uint64 indexed serviceId, address indexed operator, uint256 amount, bytes32 evidence);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct SlashProposed {
        #[allow(missing_docs)]
        pub serviceId: u64,
        #[allow(missing_docs)]
        pub operator: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub amount: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub evidence: alloy::sol_types::private::FixedBytes<32>,
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
        impl alloy_sol_types::SolEvent for SlashProposed {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<64>,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "SlashProposed(uint64,address,uint256,bytes32)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                244u8,
                173u8,
                125u8,
                110u8,
                151u8,
                114u8,
                213u8,
                146u8,
                210u8,
                166u8,
                201u8,
                68u8,
                157u8,
                140u8,
                196u8,
                11u8,
                103u8,
                185u8,
                240u8,
                228u8,
                184u8,
                28u8,
                60u8,
                118u8,
                151u8,
                156u8,
                121u8,
                155u8,
                59u8,
                235u8,
                52u8,
                194u8,
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
                    amount: data.0,
                    evidence: data.1,
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
                    > as alloy_sol_types::SolType>::tokenize(&self.amount),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.evidence),
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
        impl alloy_sol_types::private::IntoLogData for SlashProposed {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&SlashProposed> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &SlashProposed) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `SubscriptionBilled(uint64,uint256,uint64)` and selector `0x24dc485bef04b4d790d97ac10381d8d9e66c56dd6fba9931b703ca169511778a`.
```solidity
event SubscriptionBilled(uint64 indexed serviceId, uint256 amount, uint64 billedAt);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct SubscriptionBilled {
        #[allow(missing_docs)]
        pub serviceId: u64,
        #[allow(missing_docs)]
        pub amount: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub billedAt: u64,
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
        impl alloy_sol_types::SolEvent for SubscriptionBilled {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<64>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<64>,
            );
            const SIGNATURE: &'static str = "SubscriptionBilled(uint64,uint256,uint64)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                36u8,
                220u8,
                72u8,
                91u8,
                239u8,
                4u8,
                180u8,
                215u8,
                144u8,
                217u8,
                122u8,
                193u8,
                3u8,
                129u8,
                216u8,
                217u8,
                230u8,
                108u8,
                86u8,
                221u8,
                111u8,
                186u8,
                153u8,
                49u8,
                183u8,
                3u8,
                202u8,
                22u8,
                149u8,
                17u8,
                119u8,
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
                    serviceId: topics.1,
                    amount: data.0,
                    billedAt: data.1,
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
                    > as alloy_sol_types::SolType>::tokenize(&self.amount),
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::tokenize(&self.billedAt),
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
        impl alloy_sol_types::private::IntoLogData for SubscriptionBilled {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&SubscriptionBilled> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &SubscriptionBilled) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Function with signature `addPermittedCaller(uint64,address)` and selector `0x815d6a26`.
```solidity
function addPermittedCaller(uint64 serviceId, address caller) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct addPermittedCallerCall {
        #[allow(missing_docs)]
        pub serviceId: u64,
        #[allow(missing_docs)]
        pub caller: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`addPermittedCaller(uint64,address)`](addPermittedCallerCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct addPermittedCallerReturn {}
    #[allow(
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
            impl ::core::convert::From<addPermittedCallerCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: addPermittedCallerCall) -> Self {
                    (value.serviceId, value.caller)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for addPermittedCallerCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        serviceId: tuple.0,
                        caller: tuple.1,
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
            impl ::core::convert::From<addPermittedCallerReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: addPermittedCallerReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for addPermittedCallerReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for addPermittedCallerCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<64>,
                alloy::sol_types::sol_data::Address,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = addPermittedCallerReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "addPermittedCaller(uint64,address)";
            const SELECTOR: [u8; 4] = [129u8, 93u8, 106u8, 38u8];
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
                        &self.caller,
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
    /**Function with signature `approveService(uint64,uint8)` and selector `0x614823b2`.
```solidity
function approveService(uint64 requestId, uint8 restakingPercent) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct approveServiceCall {
        #[allow(missing_docs)]
        pub requestId: u64,
        #[allow(missing_docs)]
        pub restakingPercent: u8,
    }
    ///Container type for the return parameters of the [`approveService(uint64,uint8)`](approveServiceCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct approveServiceReturn {}
    #[allow(
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
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (u64, u8);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<approveServiceCall> for UnderlyingRustTuple<'_> {
                fn from(value: approveServiceCall) -> Self {
                    (value.requestId, value.restakingPercent)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for approveServiceCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        requestId: tuple.0,
                        restakingPercent: tuple.1,
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
            impl ::core::convert::From<approveServiceReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: approveServiceReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for approveServiceReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for approveServiceCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<64>,
                alloy::sol_types::sol_data::Uint<8>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = approveServiceReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "approveService(uint64,uint8)";
            const SELECTOR: [u8; 4] = [97u8, 72u8, 35u8, 178u8];
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
    /**Function with signature `approveServiceWithCommitments(uint64,((uint8,address),uint16)[])` and selector `0x23d7b3e1`.
```solidity
function approveServiceWithCommitments(uint64 requestId, Types.AssetSecurityCommitment[] memory commitments) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct approveServiceWithCommitmentsCall {
        #[allow(missing_docs)]
        pub requestId: u64,
        #[allow(missing_docs)]
        pub commitments: alloy::sol_types::private::Vec<
            <Types::AssetSecurityCommitment as alloy::sol_types::SolType>::RustType,
        >,
    }
    ///Container type for the return parameters of the [`approveServiceWithCommitments(uint64,((uint8,address),uint16)[])`](approveServiceWithCommitmentsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct approveServiceWithCommitmentsReturn {}
    #[allow(
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
                alloy::sol_types::sol_data::Array<Types::AssetSecurityCommitment>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                u64,
                alloy::sol_types::private::Vec<
                    <Types::AssetSecurityCommitment as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<approveServiceWithCommitmentsCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: approveServiceWithCommitmentsCall) -> Self {
                    (value.requestId, value.commitments)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for approveServiceWithCommitmentsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        requestId: tuple.0,
                        commitments: tuple.1,
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
            impl ::core::convert::From<approveServiceWithCommitmentsReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: approveServiceWithCommitmentsReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for approveServiceWithCommitmentsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for approveServiceWithCommitmentsCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<64>,
                alloy::sol_types::sol_data::Array<Types::AssetSecurityCommitment>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = approveServiceWithCommitmentsReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "approveServiceWithCommitments(uint64,((uint8,address),uint16)[])";
            const SELECTOR: [u8; 4] = [35u8, 215u8, 179u8, 225u8];
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
                    <alloy::sol_types::sol_data::Array<
                        Types::AssetSecurityCommitment,
                    > as alloy_sol_types::SolType>::tokenize(&self.commitments),
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
    /**Function with signature `billSubscription(uint64)` and selector `0xcf3806c6`.
```solidity
function billSubscription(uint64 serviceId) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct billSubscriptionCall {
        #[allow(missing_docs)]
        pub serviceId: u64,
    }
    ///Container type for the return parameters of the [`billSubscription(uint64)`](billSubscriptionCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct billSubscriptionReturn {}
    #[allow(
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
            impl ::core::convert::From<billSubscriptionCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: billSubscriptionCall) -> Self {
                    (value.serviceId,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for billSubscriptionCall {
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
            impl ::core::convert::From<billSubscriptionReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: billSubscriptionReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for billSubscriptionReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for billSubscriptionCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<64>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = billSubscriptionReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "billSubscription(uint64)";
            const SELECTOR: [u8; 4] = [207u8, 56u8, 6u8, 198u8];
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
    /**Function with signature `blueprintCount()` and selector `0xc602d4fa`.
```solidity
function blueprintCount() external view returns (uint64);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct blueprintCountCall {}
    ///Container type for the return parameters of the [`blueprintCount()`](blueprintCountCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct blueprintCountReturn {
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
            impl ::core::convert::From<blueprintCountCall> for UnderlyingRustTuple<'_> {
                fn from(value: blueprintCountCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for blueprintCountCall {
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
            impl ::core::convert::From<blueprintCountReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: blueprintCountReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for blueprintCountReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for blueprintCountCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = blueprintCountReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<64>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "blueprintCount()";
            const SELECTOR: [u8; 4] = [198u8, 2u8, 212u8, 250u8];
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
    /**Function with signature `blueprintOperatorCount(uint64)` and selector `0x8d3f65be`.
```solidity
function blueprintOperatorCount(uint64 blueprintId) external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct blueprintOperatorCountCall {
        #[allow(missing_docs)]
        pub blueprintId: u64,
    }
    ///Container type for the return parameters of the [`blueprintOperatorCount(uint64)`](blueprintOperatorCountCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct blueprintOperatorCountReturn {
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
            impl ::core::convert::From<blueprintOperatorCountCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: blueprintOperatorCountCall) -> Self {
                    (value.blueprintId,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for blueprintOperatorCountCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { blueprintId: tuple.0 }
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
            impl ::core::convert::From<blueprintOperatorCountReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: blueprintOperatorCountReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for blueprintOperatorCountReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for blueprintOperatorCountCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<64>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = blueprintOperatorCountReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "blueprintOperatorCount(uint64)";
            const SELECTOR: [u8; 4] = [141u8, 63u8, 101u8, 190u8];
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
    /**Function with signature `claimRewards()` and selector `0x372500ab`.
```solidity
function claimRewards() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct claimRewardsCall {}
    ///Container type for the return parameters of the [`claimRewards()`](claimRewardsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct claimRewardsReturn {}
    #[allow(
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
            impl ::core::convert::From<claimRewardsCall> for UnderlyingRustTuple<'_> {
                fn from(value: claimRewardsCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for claimRewardsCall {
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
            impl ::core::convert::From<claimRewardsReturn> for UnderlyingRustTuple<'_> {
                fn from(value: claimRewardsReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for claimRewardsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for claimRewardsCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = claimRewardsReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "claimRewards()";
            const SELECTOR: [u8; 4] = [55u8, 37u8, 0u8, 171u8];
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
    /**Function with signature `createBlueprint(string,address)` and selector `0xe72146a9`.
```solidity
function createBlueprint(string memory metadataUri, address manager) external returns (uint64 blueprintId);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct createBlueprintCall {
        #[allow(missing_docs)]
        pub metadataUri: alloy::sol_types::private::String,
        #[allow(missing_docs)]
        pub manager: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`createBlueprint(string,address)`](createBlueprintCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct createBlueprintReturn {
        #[allow(missing_docs)]
        pub blueprintId: u64,
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
                alloy::sol_types::sol_data::String,
                alloy::sol_types::sol_data::Address,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::String,
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
            impl ::core::convert::From<createBlueprintCall> for UnderlyingRustTuple<'_> {
                fn from(value: createBlueprintCall) -> Self {
                    (value.metadataUri, value.manager)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for createBlueprintCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        metadataUri: tuple.0,
                        manager: tuple.1,
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
            impl ::core::convert::From<createBlueprintReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: createBlueprintReturn) -> Self {
                    (value.blueprintId,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for createBlueprintReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { blueprintId: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for createBlueprintCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::String,
                alloy::sol_types::sol_data::Address,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = createBlueprintReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<64>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "createBlueprint(string,address)";
            const SELECTOR: [u8; 4] = [231u8, 33u8, 70u8, 169u8];
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
                        &self.metadataUri,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.manager,
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
    /**Function with signature `createBlueprintWithConfig(string,address,(uint8,uint8,uint32,uint32,uint256,uint64,uint256))` and selector `0x78448637`.
```solidity
function createBlueprintWithConfig(string memory metadataUri, address manager, Types.BlueprintConfig memory config) external returns (uint64 blueprintId);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct createBlueprintWithConfigCall {
        #[allow(missing_docs)]
        pub metadataUri: alloy::sol_types::private::String,
        #[allow(missing_docs)]
        pub manager: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub config: <Types::BlueprintConfig as alloy::sol_types::SolType>::RustType,
    }
    ///Container type for the return parameters of the [`createBlueprintWithConfig(string,address,(uint8,uint8,uint32,uint32,uint256,uint64,uint256))`](createBlueprintWithConfigCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct createBlueprintWithConfigReturn {
        #[allow(missing_docs)]
        pub blueprintId: u64,
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
                alloy::sol_types::sol_data::String,
                alloy::sol_types::sol_data::Address,
                Types::BlueprintConfig,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::String,
                alloy::sol_types::private::Address,
                <Types::BlueprintConfig as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<createBlueprintWithConfigCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: createBlueprintWithConfigCall) -> Self {
                    (value.metadataUri, value.manager, value.config)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for createBlueprintWithConfigCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        metadataUri: tuple.0,
                        manager: tuple.1,
                        config: tuple.2,
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
            impl ::core::convert::From<createBlueprintWithConfigReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: createBlueprintWithConfigReturn) -> Self {
                    (value.blueprintId,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for createBlueprintWithConfigReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { blueprintId: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for createBlueprintWithConfigCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::String,
                alloy::sol_types::sol_data::Address,
                Types::BlueprintConfig,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = createBlueprintWithConfigReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<64>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "createBlueprintWithConfig(string,address,(uint8,uint8,uint32,uint32,uint256,uint64,uint256))";
            const SELECTOR: [u8; 4] = [120u8, 68u8, 134u8, 55u8];
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
                        &self.metadataUri,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.manager,
                    ),
                    <Types::BlueprintConfig as alloy_sol_types::SolType>::tokenize(
                        &self.config,
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
    /**Function with signature `createServiceFromQuotes(uint64,((uint64,uint64,uint256,uint64,uint64,((uint8,address),uint16)[]),bytes,address)[],bytes,address[],uint64)` and selector `0xf0411af3`.
```solidity
function createServiceFromQuotes(uint64 blueprintId, Types.SignedQuote[] memory quotes, bytes memory config, address[] memory permittedCallers, uint64 ttl) external payable returns (uint64 serviceId);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct createServiceFromQuotesCall {
        #[allow(missing_docs)]
        pub blueprintId: u64,
        #[allow(missing_docs)]
        pub quotes: alloy::sol_types::private::Vec<
            <Types::SignedQuote as alloy::sol_types::SolType>::RustType,
        >,
        #[allow(missing_docs)]
        pub config: alloy::sol_types::private::Bytes,
        #[allow(missing_docs)]
        pub permittedCallers: alloy::sol_types::private::Vec<
            alloy::sol_types::private::Address,
        >,
        #[allow(missing_docs)]
        pub ttl: u64,
    }
    ///Container type for the return parameters of the [`createServiceFromQuotes(uint64,((uint64,uint64,uint256,uint64,uint64,((uint8,address),uint16)[]),bytes,address)[],bytes,address[],uint64)`](createServiceFromQuotesCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct createServiceFromQuotesReturn {
        #[allow(missing_docs)]
        pub serviceId: u64,
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
                alloy::sol_types::sol_data::Array<Types::SignedQuote>,
                alloy::sol_types::sol_data::Bytes,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
                alloy::sol_types::sol_data::Uint<64>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                u64,
                alloy::sol_types::private::Vec<
                    <Types::SignedQuote as alloy::sol_types::SolType>::RustType,
                >,
                alloy::sol_types::private::Bytes,
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
            impl ::core::convert::From<createServiceFromQuotesCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: createServiceFromQuotesCall) -> Self {
                    (
                        value.blueprintId,
                        value.quotes,
                        value.config,
                        value.permittedCallers,
                        value.ttl,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for createServiceFromQuotesCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        blueprintId: tuple.0,
                        quotes: tuple.1,
                        config: tuple.2,
                        permittedCallers: tuple.3,
                        ttl: tuple.4,
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
            impl ::core::convert::From<createServiceFromQuotesReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: createServiceFromQuotesReturn) -> Self {
                    (value.serviceId,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for createServiceFromQuotesReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { serviceId: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for createServiceFromQuotesCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<64>,
                alloy::sol_types::sol_data::Array<Types::SignedQuote>,
                alloy::sol_types::sol_data::Bytes,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
                alloy::sol_types::sol_data::Uint<64>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = createServiceFromQuotesReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<64>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "createServiceFromQuotes(uint64,((uint64,uint64,uint256,uint64,uint64,((uint8,address),uint16)[]),bytes,address)[],bytes,address[],uint64)";
            const SELECTOR: [u8; 4] = [240u8, 65u8, 26u8, 243u8];
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
                    <alloy::sol_types::sol_data::Array<
                        Types::SignedQuote,
                    > as alloy_sol_types::SolType>::tokenize(&self.quotes),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.config,
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
    /**Function with signature `deactivateBlueprint(uint64)` and selector `0x449bb849`.
```solidity
function deactivateBlueprint(uint64 blueprintId) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct deactivateBlueprintCall {
        #[allow(missing_docs)]
        pub blueprintId: u64,
    }
    ///Container type for the return parameters of the [`deactivateBlueprint(uint64)`](deactivateBlueprintCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct deactivateBlueprintReturn {}
    #[allow(
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
            impl ::core::convert::From<deactivateBlueprintCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: deactivateBlueprintCall) -> Self {
                    (value.blueprintId,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for deactivateBlueprintCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { blueprintId: tuple.0 }
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
            impl ::core::convert::From<deactivateBlueprintReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: deactivateBlueprintReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for deactivateBlueprintReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for deactivateBlueprintCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<64>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = deactivateBlueprintReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "deactivateBlueprint(uint64)";
            const SELECTOR: [u8; 4] = [68u8, 155u8, 184u8, 73u8];
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
    /**Function with signature `getBlueprint(uint64)` and selector `0xb7696dbb`.
```solidity
function getBlueprint(uint64 blueprintId) external view returns (Types.Blueprint memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getBlueprintCall {
        #[allow(missing_docs)]
        pub blueprintId: u64,
    }
    ///Container type for the return parameters of the [`getBlueprint(uint64)`](getBlueprintCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getBlueprintReturn {
        #[allow(missing_docs)]
        pub _0: <Types::Blueprint as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<getBlueprintCall> for UnderlyingRustTuple<'_> {
                fn from(value: getBlueprintCall) -> Self {
                    (value.blueprintId,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getBlueprintCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { blueprintId: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (Types::Blueprint,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <Types::Blueprint as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<getBlueprintReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getBlueprintReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getBlueprintReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getBlueprintCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<64>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getBlueprintReturn;
            type ReturnTuple<'a> = (Types::Blueprint,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getBlueprint(uint64)";
            const SELECTOR: [u8; 4] = [183u8, 105u8, 109u8, 187u8];
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
    /**Function with signature `getBlueprintConfig(uint64)` and selector `0x563a89f9`.
```solidity
function getBlueprintConfig(uint64 blueprintId) external view returns (Types.BlueprintConfig memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getBlueprintConfigCall {
        #[allow(missing_docs)]
        pub blueprintId: u64,
    }
    ///Container type for the return parameters of the [`getBlueprintConfig(uint64)`](getBlueprintConfigCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getBlueprintConfigReturn {
        #[allow(missing_docs)]
        pub _0: <Types::BlueprintConfig as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<getBlueprintConfigCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getBlueprintConfigCall) -> Self {
                    (value.blueprintId,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getBlueprintConfigCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { blueprintId: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (Types::BlueprintConfig,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <Types::BlueprintConfig as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<getBlueprintConfigReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getBlueprintConfigReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getBlueprintConfigReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getBlueprintConfigCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<64>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getBlueprintConfigReturn;
            type ReturnTuple<'a> = (Types::BlueprintConfig,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getBlueprintConfig(uint64)";
            const SELECTOR: [u8; 4] = [86u8, 58u8, 137u8, 249u8];
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
    /**Function with signature `getOperatorRegistration(uint64,address)` and selector `0x6bda42f3`.
```solidity
function getOperatorRegistration(uint64 blueprintId, address operator) external view returns (Types.OperatorRegistration memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getOperatorRegistrationCall {
        #[allow(missing_docs)]
        pub blueprintId: u64,
        #[allow(missing_docs)]
        pub operator: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`getOperatorRegistration(uint64,address)`](getOperatorRegistrationCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getOperatorRegistrationReturn {
        #[allow(missing_docs)]
        pub _0: <Types::OperatorRegistration as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<getOperatorRegistrationCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getOperatorRegistrationCall) -> Self {
                    (value.blueprintId, value.operator)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getOperatorRegistrationCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        blueprintId: tuple.0,
                        operator: tuple.1,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (Types::OperatorRegistration,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <Types::OperatorRegistration as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<getOperatorRegistrationReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getOperatorRegistrationReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getOperatorRegistrationReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getOperatorRegistrationCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<64>,
                alloy::sol_types::sol_data::Address,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getOperatorRegistrationReturn;
            type ReturnTuple<'a> = (Types::OperatorRegistration,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getOperatorRegistration(uint64,address)";
            const SELECTOR: [u8; 4] = [107u8, 218u8, 66u8, 243u8];
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
    /**Function with signature `getService(uint64)` and selector `0x3dc0d5fe`.
```solidity
function getService(uint64 serviceId) external view returns (Types.Service memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getServiceCall {
        #[allow(missing_docs)]
        pub serviceId: u64,
    }
    ///Container type for the return parameters of the [`getService(uint64)`](getServiceCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getServiceReturn {
        #[allow(missing_docs)]
        pub _0: <Types::Service as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<getServiceCall> for UnderlyingRustTuple<'_> {
                fn from(value: getServiceCall) -> Self {
                    (value.serviceId,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getServiceCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { serviceId: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (Types::Service,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <Types::Service as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<getServiceReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getServiceReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getServiceReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getServiceCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<64>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getServiceReturn;
            type ReturnTuple<'a> = (Types::Service,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getService(uint64)";
            const SELECTOR: [u8; 4] = [61u8, 192u8, 213u8, 254u8];
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
    /**Function with signature `getServiceOperator(uint64,address)` and selector `0x842524bb`.
```solidity
function getServiceOperator(uint64 serviceId, address operator) external view returns (Types.ServiceOperator memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getServiceOperatorCall {
        #[allow(missing_docs)]
        pub serviceId: u64,
        #[allow(missing_docs)]
        pub operator: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`getServiceOperator(uint64,address)`](getServiceOperatorCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getServiceOperatorReturn {
        #[allow(missing_docs)]
        pub _0: <Types::ServiceOperator as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<getServiceOperatorCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getServiceOperatorCall) -> Self {
                    (value.serviceId, value.operator)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getServiceOperatorCall {
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
            type UnderlyingSolTuple<'a> = (Types::ServiceOperator,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <Types::ServiceOperator as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<getServiceOperatorReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getServiceOperatorReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getServiceOperatorReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getServiceOperatorCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<64>,
                alloy::sol_types::sol_data::Address,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getServiceOperatorReturn;
            type ReturnTuple<'a> = (Types::ServiceOperator,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getServiceOperator(uint64,address)";
            const SELECTOR: [u8; 4] = [132u8, 37u8, 36u8, 187u8];
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
    /**Function with signature `getServiceOperators(uint64)` and selector `0xbb207f1f`.
```solidity
function getServiceOperators(uint64 serviceId) external view returns (address[] memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getServiceOperatorsCall {
        #[allow(missing_docs)]
        pub serviceId: u64,
    }
    ///Container type for the return parameters of the [`getServiceOperators(uint64)`](getServiceOperatorsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getServiceOperatorsReturn {
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
            impl ::core::convert::From<getServiceOperatorsCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getServiceOperatorsCall) -> Self {
                    (value.serviceId,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getServiceOperatorsCall {
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
            impl ::core::convert::From<getServiceOperatorsReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getServiceOperatorsReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getServiceOperatorsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getServiceOperatorsCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<64>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getServiceOperatorsReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getServiceOperators(uint64)";
            const SELECTOR: [u8; 4] = [187u8, 32u8, 127u8, 31u8];
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
    /**Function with signature `getServiceRequest(uint64)` and selector `0x5f9b4dfa`.
```solidity
function getServiceRequest(uint64 requestId) external view returns (Types.ServiceRequest memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getServiceRequestCall {
        #[allow(missing_docs)]
        pub requestId: u64,
    }
    ///Container type for the return parameters of the [`getServiceRequest(uint64)`](getServiceRequestCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getServiceRequestReturn {
        #[allow(missing_docs)]
        pub _0: <Types::ServiceRequest as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<getServiceRequestCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getServiceRequestCall) -> Self {
                    (value.requestId,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getServiceRequestCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { requestId: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (Types::ServiceRequest,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <Types::ServiceRequest as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<getServiceRequestReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getServiceRequestReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getServiceRequestReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getServiceRequestCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<64>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getServiceRequestReturn;
            type ReturnTuple<'a> = (Types::ServiceRequest,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getServiceRequest(uint64)";
            const SELECTOR: [u8; 4] = [95u8, 155u8, 77u8, 250u8];
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
    /**Function with signature `getServiceTotalExposure(uint64)` and selector `0x3e354514`.
```solidity
function getServiceTotalExposure(uint64 serviceId) external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getServiceTotalExposureCall {
        #[allow(missing_docs)]
        pub serviceId: u64,
    }
    ///Container type for the return parameters of the [`getServiceTotalExposure(uint64)`](getServiceTotalExposureCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getServiceTotalExposureReturn {
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
            impl ::core::convert::From<getServiceTotalExposureCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getServiceTotalExposureCall) -> Self {
                    (value.serviceId,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getServiceTotalExposureCall {
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
            impl ::core::convert::From<getServiceTotalExposureReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getServiceTotalExposureReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getServiceTotalExposureReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getServiceTotalExposureCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<64>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getServiceTotalExposureReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getServiceTotalExposure(uint64)";
            const SELECTOR: [u8; 4] = [62u8, 53u8, 69u8, 20u8];
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
    /**Function with signature `isOperatorRegistered(uint64,address)` and selector `0x64462595`.
```solidity
function isOperatorRegistered(uint64 blueprintId, address operator) external view returns (bool);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isOperatorRegisteredCall {
        #[allow(missing_docs)]
        pub blueprintId: u64,
        #[allow(missing_docs)]
        pub operator: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`isOperatorRegistered(uint64,address)`](isOperatorRegisteredCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isOperatorRegisteredReturn {
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
            impl ::core::convert::From<isOperatorRegisteredCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: isOperatorRegisteredCall) -> Self {
                    (value.blueprintId, value.operator)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for isOperatorRegisteredCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        blueprintId: tuple.0,
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
            impl ::core::convert::From<isOperatorRegisteredReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: isOperatorRegisteredReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for isOperatorRegisteredReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for isOperatorRegisteredCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<64>,
                alloy::sol_types::sol_data::Address,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = isOperatorRegisteredReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "isOperatorRegistered(uint64,address)";
            const SELECTOR: [u8; 4] = [100u8, 70u8, 37u8, 149u8];
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
    /**Function with signature `isPermittedCaller(uint64,address)` and selector `0x2ec2bd03`.
```solidity
function isPermittedCaller(uint64 serviceId, address caller) external view returns (bool);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isPermittedCallerCall {
        #[allow(missing_docs)]
        pub serviceId: u64,
        #[allow(missing_docs)]
        pub caller: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`isPermittedCaller(uint64,address)`](isPermittedCallerCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isPermittedCallerReturn {
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
            impl ::core::convert::From<isPermittedCallerCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: isPermittedCallerCall) -> Self {
                    (value.serviceId, value.caller)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for isPermittedCallerCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        serviceId: tuple.0,
                        caller: tuple.1,
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
            impl ::core::convert::From<isPermittedCallerReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: isPermittedCallerReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for isPermittedCallerReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for isPermittedCallerCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<64>,
                alloy::sol_types::sol_data::Address,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = isPermittedCallerReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "isPermittedCaller(uint64,address)";
            const SELECTOR: [u8; 4] = [46u8, 194u8, 189u8, 3u8];
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
                        &self.caller,
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
    /**Function with signature `isServiceActive(uint64)` and selector `0x2f46279f`.
```solidity
function isServiceActive(uint64 serviceId) external view returns (bool);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isServiceActiveCall {
        #[allow(missing_docs)]
        pub serviceId: u64,
    }
    ///Container type for the return parameters of the [`isServiceActive(uint64)`](isServiceActiveCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isServiceActiveReturn {
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
            impl ::core::convert::From<isServiceActiveCall> for UnderlyingRustTuple<'_> {
                fn from(value: isServiceActiveCall) -> Self {
                    (value.serviceId,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for isServiceActiveCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { serviceId: tuple.0 }
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
            impl ::core::convert::From<isServiceActiveReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: isServiceActiveReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for isServiceActiveReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for isServiceActiveCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<64>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = isServiceActiveReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "isServiceActive(uint64)";
            const SELECTOR: [u8; 4] = [47u8, 70u8, 39u8, 159u8];
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
    /**Function with signature `isServiceOperator(uint64,address)` and selector `0xa595b21e`.
```solidity
function isServiceOperator(uint64 serviceId, address operator) external view returns (bool);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isServiceOperatorCall {
        #[allow(missing_docs)]
        pub serviceId: u64,
        #[allow(missing_docs)]
        pub operator: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`isServiceOperator(uint64,address)`](isServiceOperatorCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isServiceOperatorReturn {
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
            impl ::core::convert::From<isServiceOperatorCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: isServiceOperatorCall) -> Self {
                    (value.serviceId, value.operator)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for isServiceOperatorCall {
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
            impl ::core::convert::From<isServiceOperatorReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: isServiceOperatorReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for isServiceOperatorReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for isServiceOperatorCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<64>,
                alloy::sol_types::sol_data::Address,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = isServiceOperatorReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "isServiceOperator(uint64,address)";
            const SELECTOR: [u8; 4] = [165u8, 149u8, 178u8, 30u8];
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
    /**Function with signature `joinService(uint64,uint16)` and selector `0x2b9da71a`.
```solidity
function joinService(uint64 serviceId, uint16 exposureBps) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct joinServiceCall {
        #[allow(missing_docs)]
        pub serviceId: u64,
        #[allow(missing_docs)]
        pub exposureBps: u16,
    }
    ///Container type for the return parameters of the [`joinService(uint64,uint16)`](joinServiceCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct joinServiceReturn {}
    #[allow(
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
                alloy::sol_types::sol_data::Uint<16>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (u64, u16);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<joinServiceCall> for UnderlyingRustTuple<'_> {
                fn from(value: joinServiceCall) -> Self {
                    (value.serviceId, value.exposureBps)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for joinServiceCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        serviceId: tuple.0,
                        exposureBps: tuple.1,
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
            impl ::core::convert::From<joinServiceReturn> for UnderlyingRustTuple<'_> {
                fn from(value: joinServiceReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for joinServiceReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for joinServiceCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<64>,
                alloy::sol_types::sol_data::Uint<16>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = joinServiceReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "joinService(uint64,uint16)";
            const SELECTOR: [u8; 4] = [43u8, 157u8, 167u8, 26u8];
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
                        16,
                    > as alloy_sol_types::SolType>::tokenize(&self.exposureBps),
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
    /**Function with signature `leaveService(uint64)` and selector `0x6778afbc`.
```solidity
function leaveService(uint64 serviceId) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct leaveServiceCall {
        #[allow(missing_docs)]
        pub serviceId: u64,
    }
    ///Container type for the return parameters of the [`leaveService(uint64)`](leaveServiceCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct leaveServiceReturn {}
    #[allow(
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
            impl ::core::convert::From<leaveServiceCall> for UnderlyingRustTuple<'_> {
                fn from(value: leaveServiceCall) -> Self {
                    (value.serviceId,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for leaveServiceCall {
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
            impl ::core::convert::From<leaveServiceReturn> for UnderlyingRustTuple<'_> {
                fn from(value: leaveServiceReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for leaveServiceReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for leaveServiceCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<64>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = leaveServiceReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "leaveService(uint64)";
            const SELECTOR: [u8; 4] = [103u8, 120u8, 175u8, 188u8];
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
    /**Function with signature `pause()` and selector `0x8456cb59`.
```solidity
function pause() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct pauseCall {}
    ///Container type for the return parameters of the [`pause()`](pauseCall) function.
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
            impl ::core::convert::From<pauseCall> for UnderlyingRustTuple<'_> {
                fn from(value: pauseCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for pauseCall {
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
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = pauseReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "pause()";
            const SELECTOR: [u8; 4] = [132u8, 86u8, 203u8, 89u8];
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
    /**Function with signature `pendingRewards(address)` and selector `0x31d7a262`.
```solidity
function pendingRewards(address account) external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct pendingRewardsCall {
        #[allow(missing_docs)]
        pub account: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`pendingRewards(address)`](pendingRewardsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct pendingRewardsReturn {
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
            impl ::core::convert::From<pendingRewardsCall> for UnderlyingRustTuple<'_> {
                fn from(value: pendingRewardsCall) -> Self {
                    (value.account,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for pendingRewardsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { account: tuple.0 }
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
            impl ::core::convert::From<pendingRewardsReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: pendingRewardsReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for pendingRewardsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for pendingRewardsCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = pendingRewardsReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "pendingRewards(address)";
            const SELECTOR: [u8; 4] = [49u8, 215u8, 162u8, 98u8];
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
    /**Function with signature `proposeSlash(uint64,address,uint256,bytes32)` and selector `0x30ae289c`.
```solidity
function proposeSlash(uint64 serviceId, address operator, uint256 amount, bytes32 evidence) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct proposeSlashCall {
        #[allow(missing_docs)]
        pub serviceId: u64,
        #[allow(missing_docs)]
        pub operator: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub amount: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub evidence: alloy::sol_types::private::FixedBytes<32>,
    }
    ///Container type for the return parameters of the [`proposeSlash(uint64,address,uint256,bytes32)`](proposeSlashCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct proposeSlashReturn {}
    #[allow(
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
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                u64,
                alloy::sol_types::private::Address,
                alloy::sol_types::private::primitives::aliases::U256,
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
            impl ::core::convert::From<proposeSlashCall> for UnderlyingRustTuple<'_> {
                fn from(value: proposeSlashCall) -> Self {
                    (value.serviceId, value.operator, value.amount, value.evidence)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for proposeSlashCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        serviceId: tuple.0,
                        operator: tuple.1,
                        amount: tuple.2,
                        evidence: tuple.3,
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
            impl ::core::convert::From<proposeSlashReturn> for UnderlyingRustTuple<'_> {
                fn from(value: proposeSlashReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for proposeSlashReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for proposeSlashCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<64>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = proposeSlashReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "proposeSlash(uint64,address,uint256,bytes32)";
            const SELECTOR: [u8; 4] = [48u8, 174u8, 40u8, 156u8];
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
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.amount),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.evidence),
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
    /**Function with signature `registerOperator(uint64,bytes)` and selector `0xcc3d19c3`.
```solidity
function registerOperator(uint64 blueprintId, bytes memory preferences) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct registerOperatorCall {
        #[allow(missing_docs)]
        pub blueprintId: u64,
        #[allow(missing_docs)]
        pub preferences: alloy::sol_types::private::Bytes,
    }
    ///Container type for the return parameters of the [`registerOperator(uint64,bytes)`](registerOperatorCall) function.
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
                alloy::sol_types::sol_data::Uint<64>,
                alloy::sol_types::sol_data::Bytes,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (u64, alloy::sol_types::private::Bytes);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<registerOperatorCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: registerOperatorCall) -> Self {
                    (value.blueprintId, value.preferences)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for registerOperatorCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        blueprintId: tuple.0,
                        preferences: tuple.1,
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
            impl ::core::convert::From<registerOperatorReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: registerOperatorReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for registerOperatorReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for registerOperatorCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<64>,
                alloy::sol_types::sol_data::Bytes,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = registerOperatorReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "registerOperator(uint64,bytes)";
            const SELECTOR: [u8; 4] = [204u8, 61u8, 25u8, 195u8];
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
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.preferences,
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
    /**Function with signature `rejectService(uint64)` and selector `0x6d8b88c3`.
```solidity
function rejectService(uint64 requestId) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct rejectServiceCall {
        #[allow(missing_docs)]
        pub requestId: u64,
    }
    ///Container type for the return parameters of the [`rejectService(uint64)`](rejectServiceCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct rejectServiceReturn {}
    #[allow(
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
            impl ::core::convert::From<rejectServiceCall> for UnderlyingRustTuple<'_> {
                fn from(value: rejectServiceCall) -> Self {
                    (value.requestId,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for rejectServiceCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { requestId: tuple.0 }
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
            impl ::core::convert::From<rejectServiceReturn> for UnderlyingRustTuple<'_> {
                fn from(value: rejectServiceReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for rejectServiceReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for rejectServiceCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<64>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = rejectServiceReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "rejectService(uint64)";
            const SELECTOR: [u8; 4] = [109u8, 139u8, 136u8, 195u8];
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
    /**Function with signature `removePermittedCaller(uint64,address)` and selector `0x5f359924`.
```solidity
function removePermittedCaller(uint64 serviceId, address caller) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct removePermittedCallerCall {
        #[allow(missing_docs)]
        pub serviceId: u64,
        #[allow(missing_docs)]
        pub caller: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`removePermittedCaller(uint64,address)`](removePermittedCallerCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct removePermittedCallerReturn {}
    #[allow(
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
            impl ::core::convert::From<removePermittedCallerCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: removePermittedCallerCall) -> Self {
                    (value.serviceId, value.caller)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for removePermittedCallerCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        serviceId: tuple.0,
                        caller: tuple.1,
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
            impl ::core::convert::From<removePermittedCallerReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: removePermittedCallerReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for removePermittedCallerReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for removePermittedCallerCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<64>,
                alloy::sol_types::sol_data::Address,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = removePermittedCallerReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "removePermittedCaller(uint64,address)";
            const SELECTOR: [u8; 4] = [95u8, 53u8, 153u8, 36u8];
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
                        &self.caller,
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
    /**Function with signature `requestService(uint64,address[],bytes,address[],uint64,address,uint256)` and selector `0xa37b9286`.
```solidity
function requestService(uint64 blueprintId, address[] memory operators, bytes memory config, address[] memory permittedCallers, uint64 ttl, address paymentToken, uint256 paymentAmount) external payable returns (uint64 requestId);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct requestServiceCall {
        #[allow(missing_docs)]
        pub blueprintId: u64,
        #[allow(missing_docs)]
        pub operators: alloy::sol_types::private::Vec<
            alloy::sol_types::private::Address,
        >,
        #[allow(missing_docs)]
        pub config: alloy::sol_types::private::Bytes,
        #[allow(missing_docs)]
        pub permittedCallers: alloy::sol_types::private::Vec<
            alloy::sol_types::private::Address,
        >,
        #[allow(missing_docs)]
        pub ttl: u64,
        #[allow(missing_docs)]
        pub paymentToken: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub paymentAmount: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`requestService(uint64,address[],bytes,address[],uint64,address,uint256)`](requestServiceCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct requestServiceReturn {
        #[allow(missing_docs)]
        pub requestId: u64,
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
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
                alloy::sol_types::sol_data::Bytes,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
                alloy::sol_types::sol_data::Uint<64>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                u64,
                alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
                alloy::sol_types::private::Bytes,
                alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
                u64,
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
            impl ::core::convert::From<requestServiceCall> for UnderlyingRustTuple<'_> {
                fn from(value: requestServiceCall) -> Self {
                    (
                        value.blueprintId,
                        value.operators,
                        value.config,
                        value.permittedCallers,
                        value.ttl,
                        value.paymentToken,
                        value.paymentAmount,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for requestServiceCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        blueprintId: tuple.0,
                        operators: tuple.1,
                        config: tuple.2,
                        permittedCallers: tuple.3,
                        ttl: tuple.4,
                        paymentToken: tuple.5,
                        paymentAmount: tuple.6,
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
            impl ::core::convert::From<requestServiceReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: requestServiceReturn) -> Self {
                    (value.requestId,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for requestServiceReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { requestId: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for requestServiceCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<64>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
                alloy::sol_types::sol_data::Bytes,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
                alloy::sol_types::sol_data::Uint<64>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = requestServiceReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<64>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "requestService(uint64,address[],bytes,address[],uint64,address,uint256)";
            const SELECTOR: [u8; 4] = [163u8, 123u8, 146u8, 134u8];
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
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Address,
                    > as alloy_sol_types::SolType>::tokenize(&self.operators),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.config,
                    ),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Address,
                    > as alloy_sol_types::SolType>::tokenize(&self.permittedCallers),
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::tokenize(&self.ttl),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.paymentToken,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.paymentAmount),
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
    /**Function with signature `requestServiceWithExposure(uint64,address[],uint16[],bytes,address[],uint64,address,uint256)` and selector `0x108a7d63`.
```solidity
function requestServiceWithExposure(uint64 blueprintId, address[] memory operators, uint16[] memory exposureBps, bytes memory config, address[] memory permittedCallers, uint64 ttl, address paymentToken, uint256 paymentAmount) external payable returns (uint64 requestId);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct requestServiceWithExposureCall {
        #[allow(missing_docs)]
        pub blueprintId: u64,
        #[allow(missing_docs)]
        pub operators: alloy::sol_types::private::Vec<
            alloy::sol_types::private::Address,
        >,
        #[allow(missing_docs)]
        pub exposureBps: alloy::sol_types::private::Vec<u16>,
        #[allow(missing_docs)]
        pub config: alloy::sol_types::private::Bytes,
        #[allow(missing_docs)]
        pub permittedCallers: alloy::sol_types::private::Vec<
            alloy::sol_types::private::Address,
        >,
        #[allow(missing_docs)]
        pub ttl: u64,
        #[allow(missing_docs)]
        pub paymentToken: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub paymentAmount: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`requestServiceWithExposure(uint64,address[],uint16[],bytes,address[],uint64,address,uint256)`](requestServiceWithExposureCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct requestServiceWithExposureReturn {
        #[allow(missing_docs)]
        pub requestId: u64,
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
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<16>>,
                alloy::sol_types::sol_data::Bytes,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
                alloy::sol_types::sol_data::Uint<64>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                u64,
                alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
                alloy::sol_types::private::Vec<u16>,
                alloy::sol_types::private::Bytes,
                alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
                u64,
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
            impl ::core::convert::From<requestServiceWithExposureCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: requestServiceWithExposureCall) -> Self {
                    (
                        value.blueprintId,
                        value.operators,
                        value.exposureBps,
                        value.config,
                        value.permittedCallers,
                        value.ttl,
                        value.paymentToken,
                        value.paymentAmount,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for requestServiceWithExposureCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        blueprintId: tuple.0,
                        operators: tuple.1,
                        exposureBps: tuple.2,
                        config: tuple.3,
                        permittedCallers: tuple.4,
                        ttl: tuple.5,
                        paymentToken: tuple.6,
                        paymentAmount: tuple.7,
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
            impl ::core::convert::From<requestServiceWithExposureReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: requestServiceWithExposureReturn) -> Self {
                    (value.requestId,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for requestServiceWithExposureReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { requestId: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for requestServiceWithExposureCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<64>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<16>>,
                alloy::sol_types::sol_data::Bytes,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
                alloy::sol_types::sol_data::Uint<64>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = requestServiceWithExposureReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<64>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "requestServiceWithExposure(uint64,address[],uint16[],bytes,address[],uint64,address,uint256)";
            const SELECTOR: [u8; 4] = [16u8, 138u8, 125u8, 99u8];
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
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Address,
                    > as alloy_sol_types::SolType>::tokenize(&self.operators),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<16>,
                    > as alloy_sol_types::SolType>::tokenize(&self.exposureBps),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.config,
                    ),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Address,
                    > as alloy_sol_types::SolType>::tokenize(&self.permittedCallers),
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::tokenize(&self.ttl),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.paymentToken,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.paymentAmount),
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
    /**Function with signature `requestServiceWithSecurity(uint64,address[],((uint8,address),uint16,uint16)[],bytes,address[],uint64,address,uint256)` and selector `0xec9f0fdd`.
```solidity
function requestServiceWithSecurity(uint64 blueprintId, address[] memory operators, Types.AssetSecurityRequirement[] memory securityRequirements, bytes memory config, address[] memory permittedCallers, uint64 ttl, address paymentToken, uint256 paymentAmount) external payable returns (uint64 requestId);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct requestServiceWithSecurityCall {
        #[allow(missing_docs)]
        pub blueprintId: u64,
        #[allow(missing_docs)]
        pub operators: alloy::sol_types::private::Vec<
            alloy::sol_types::private::Address,
        >,
        #[allow(missing_docs)]
        pub securityRequirements: alloy::sol_types::private::Vec<
            <Types::AssetSecurityRequirement as alloy::sol_types::SolType>::RustType,
        >,
        #[allow(missing_docs)]
        pub config: alloy::sol_types::private::Bytes,
        #[allow(missing_docs)]
        pub permittedCallers: alloy::sol_types::private::Vec<
            alloy::sol_types::private::Address,
        >,
        #[allow(missing_docs)]
        pub ttl: u64,
        #[allow(missing_docs)]
        pub paymentToken: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub paymentAmount: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`requestServiceWithSecurity(uint64,address[],((uint8,address),uint16,uint16)[],bytes,address[],uint64,address,uint256)`](requestServiceWithSecurityCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct requestServiceWithSecurityReturn {
        #[allow(missing_docs)]
        pub requestId: u64,
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
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
                alloy::sol_types::sol_data::Array<Types::AssetSecurityRequirement>,
                alloy::sol_types::sol_data::Bytes,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
                alloy::sol_types::sol_data::Uint<64>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                u64,
                alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
                alloy::sol_types::private::Vec<
                    <Types::AssetSecurityRequirement as alloy::sol_types::SolType>::RustType,
                >,
                alloy::sol_types::private::Bytes,
                alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
                u64,
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
            impl ::core::convert::From<requestServiceWithSecurityCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: requestServiceWithSecurityCall) -> Self {
                    (
                        value.blueprintId,
                        value.operators,
                        value.securityRequirements,
                        value.config,
                        value.permittedCallers,
                        value.ttl,
                        value.paymentToken,
                        value.paymentAmount,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for requestServiceWithSecurityCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        blueprintId: tuple.0,
                        operators: tuple.1,
                        securityRequirements: tuple.2,
                        config: tuple.3,
                        permittedCallers: tuple.4,
                        ttl: tuple.5,
                        paymentToken: tuple.6,
                        paymentAmount: tuple.7,
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
            impl ::core::convert::From<requestServiceWithSecurityReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: requestServiceWithSecurityReturn) -> Self {
                    (value.requestId,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for requestServiceWithSecurityReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { requestId: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for requestServiceWithSecurityCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<64>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
                alloy::sol_types::sol_data::Array<Types::AssetSecurityRequirement>,
                alloy::sol_types::sol_data::Bytes,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
                alloy::sol_types::sol_data::Uint<64>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = requestServiceWithSecurityReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<64>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "requestServiceWithSecurity(uint64,address[],((uint8,address),uint16,uint16)[],bytes,address[],uint64,address,uint256)";
            const SELECTOR: [u8; 4] = [236u8, 159u8, 15u8, 221u8];
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
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Address,
                    > as alloy_sol_types::SolType>::tokenize(&self.operators),
                    <alloy::sol_types::sol_data::Array<
                        Types::AssetSecurityRequirement,
                    > as alloy_sol_types::SolType>::tokenize(&self.securityRequirements),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.config,
                    ),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Address,
                    > as alloy_sol_types::SolType>::tokenize(&self.permittedCallers),
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::tokenize(&self.ttl),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.paymentToken,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.paymentAmount),
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
    /**Function with signature `serviceCount()` and selector `0x06237526`.
```solidity
function serviceCount() external view returns (uint64);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct serviceCountCall {}
    ///Container type for the return parameters of the [`serviceCount()`](serviceCountCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct serviceCountReturn {
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
            impl ::core::convert::From<serviceCountCall> for UnderlyingRustTuple<'_> {
                fn from(value: serviceCountCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for serviceCountCall {
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
            impl ::core::convert::From<serviceCountReturn> for UnderlyingRustTuple<'_> {
                fn from(value: serviceCountReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for serviceCountReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for serviceCountCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = serviceCountReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<64>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "serviceCount()";
            const SELECTOR: [u8; 4] = [6u8, 35u8, 117u8, 38u8];
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
    /**Function with signature `setOperatorOnline(uint64,bool)` and selector `0x7e8ac13e`.
```solidity
function setOperatorOnline(uint64 blueprintId, bool online) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setOperatorOnlineCall {
        #[allow(missing_docs)]
        pub blueprintId: u64,
        #[allow(missing_docs)]
        pub online: bool,
    }
    ///Container type for the return parameters of the [`setOperatorOnline(uint64,bool)`](setOperatorOnlineCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setOperatorOnlineReturn {}
    #[allow(
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
            impl ::core::convert::From<setOperatorOnlineCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: setOperatorOnlineCall) -> Self {
                    (value.blueprintId, value.online)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for setOperatorOnlineCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        blueprintId: tuple.0,
                        online: tuple.1,
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
            impl ::core::convert::From<setOperatorOnlineReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: setOperatorOnlineReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for setOperatorOnlineReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for setOperatorOnlineCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<64>,
                alloy::sol_types::sol_data::Bool,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = setOperatorOnlineReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "setOperatorOnline(uint64,bool)";
            const SELECTOR: [u8; 4] = [126u8, 138u8, 193u8, 62u8];
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
                    <alloy::sol_types::sol_data::Bool as alloy_sol_types::SolType>::tokenize(
                        &self.online,
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
    /**Function with signature `setPaymentSplit((uint16,uint16,uint16,uint16))` and selector `0xb81741ac`.
```solidity
function setPaymentSplit(Types.PaymentSplit memory split) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setPaymentSplitCall {
        #[allow(missing_docs)]
        pub split: <Types::PaymentSplit as alloy::sol_types::SolType>::RustType,
    }
    ///Container type for the return parameters of the [`setPaymentSplit((uint16,uint16,uint16,uint16))`](setPaymentSplitCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setPaymentSplitReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (Types::PaymentSplit,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <Types::PaymentSplit as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<setPaymentSplitCall> for UnderlyingRustTuple<'_> {
                fn from(value: setPaymentSplitCall) -> Self {
                    (value.split,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for setPaymentSplitCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { split: tuple.0 }
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
            impl ::core::convert::From<setPaymentSplitReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: setPaymentSplitReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for setPaymentSplitReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for setPaymentSplitCall {
            type Parameters<'a> = (Types::PaymentSplit,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = setPaymentSplitReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "setPaymentSplit((uint16,uint16,uint16,uint16))";
            const SELECTOR: [u8; 4] = [184u8, 23u8, 65u8, 172u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <Types::PaymentSplit as alloy_sol_types::SolType>::tokenize(
                        &self.split,
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
    /**Function with signature `setRestaking(address)` and selector `0x99ff3654`.
```solidity
function setRestaking(address restaking) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setRestakingCall {
        #[allow(missing_docs)]
        pub restaking: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`setRestaking(address)`](setRestakingCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setRestakingReturn {}
    #[allow(
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
            impl ::core::convert::From<setRestakingCall> for UnderlyingRustTuple<'_> {
                fn from(value: setRestakingCall) -> Self {
                    (value.restaking,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for setRestakingCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { restaking: tuple.0 }
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
            impl ::core::convert::From<setRestakingReturn> for UnderlyingRustTuple<'_> {
                fn from(value: setRestakingReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for setRestakingReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for setRestakingCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = setRestakingReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "setRestaking(address)";
            const SELECTOR: [u8; 4] = [153u8, 255u8, 54u8, 84u8];
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
                        &self.restaking,
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
    /**Function with signature `setTreasury(address)` and selector `0xf0f44260`.
```solidity
function setTreasury(address treasury) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setTreasuryCall {
        #[allow(missing_docs)]
        pub treasury: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`setTreasury(address)`](setTreasuryCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setTreasuryReturn {}
    #[allow(
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
            impl ::core::convert::From<setTreasuryCall> for UnderlyingRustTuple<'_> {
                fn from(value: setTreasuryCall) -> Self {
                    (value.treasury,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for setTreasuryCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { treasury: tuple.0 }
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
            impl ::core::convert::From<setTreasuryReturn> for UnderlyingRustTuple<'_> {
                fn from(value: setTreasuryReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for setTreasuryReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for setTreasuryCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = setTreasuryReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "setTreasury(address)";
            const SELECTOR: [u8; 4] = [240u8, 244u8, 66u8, 96u8];
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
                        &self.treasury,
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
    /**Function with signature `terminateService(uint64)` and selector `0x5b71c934`.
```solidity
function terminateService(uint64 serviceId) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct terminateServiceCall {
        #[allow(missing_docs)]
        pub serviceId: u64,
    }
    ///Container type for the return parameters of the [`terminateService(uint64)`](terminateServiceCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct terminateServiceReturn {}
    #[allow(
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
            impl ::core::convert::From<terminateServiceCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: terminateServiceCall) -> Self {
                    (value.serviceId,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for terminateServiceCall {
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
            impl ::core::convert::From<terminateServiceReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: terminateServiceReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for terminateServiceReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for terminateServiceCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<64>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = terminateServiceReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "terminateService(uint64)";
            const SELECTOR: [u8; 4] = [91u8, 113u8, 201u8, 52u8];
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
    /**Function with signature `transferBlueprint(uint64,address)` and selector `0x8a4cf763`.
```solidity
function transferBlueprint(uint64 blueprintId, address newOwner) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct transferBlueprintCall {
        #[allow(missing_docs)]
        pub blueprintId: u64,
        #[allow(missing_docs)]
        pub newOwner: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`transferBlueprint(uint64,address)`](transferBlueprintCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct transferBlueprintReturn {}
    #[allow(
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
            impl ::core::convert::From<transferBlueprintCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: transferBlueprintCall) -> Self {
                    (value.blueprintId, value.newOwner)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for transferBlueprintCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        blueprintId: tuple.0,
                        newOwner: tuple.1,
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
            impl ::core::convert::From<transferBlueprintReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: transferBlueprintReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for transferBlueprintReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for transferBlueprintCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<64>,
                alloy::sol_types::sol_data::Address,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = transferBlueprintReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "transferBlueprint(uint64,address)";
            const SELECTOR: [u8; 4] = [138u8, 76u8, 247u8, 99u8];
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
    /**Function with signature `unpause()` and selector `0x3f4ba83a`.
```solidity
function unpause() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct unpauseCall {}
    ///Container type for the return parameters of the [`unpause()`](unpauseCall) function.
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
            impl ::core::convert::From<unpauseCall> for UnderlyingRustTuple<'_> {
                fn from(value: unpauseCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for unpauseCall {
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
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = unpauseReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "unpause()";
            const SELECTOR: [u8; 4] = [63u8, 75u8, 168u8, 58u8];
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
    /**Function with signature `unregisterOperator(uint64)` and selector `0x0afd3738`.
```solidity
function unregisterOperator(uint64 blueprintId) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct unregisterOperatorCall {
        #[allow(missing_docs)]
        pub blueprintId: u64,
    }
    ///Container type for the return parameters of the [`unregisterOperator(uint64)`](unregisterOperatorCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct unregisterOperatorReturn {}
    #[allow(
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
            impl ::core::convert::From<unregisterOperatorCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: unregisterOperatorCall) -> Self {
                    (value.blueprintId,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for unregisterOperatorCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { blueprintId: tuple.0 }
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
            impl ::core::convert::From<unregisterOperatorReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: unregisterOperatorReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for unregisterOperatorReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for unregisterOperatorCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<64>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = unregisterOperatorReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "unregisterOperator(uint64)";
            const SELECTOR: [u8; 4] = [10u8, 253u8, 55u8, 56u8];
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
    /**Function with signature `updateBlueprint(uint64,string)` and selector `0xe538da66`.
```solidity
function updateBlueprint(uint64 blueprintId, string memory metadataUri) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct updateBlueprintCall {
        #[allow(missing_docs)]
        pub blueprintId: u64,
        #[allow(missing_docs)]
        pub metadataUri: alloy::sol_types::private::String,
    }
    ///Container type for the return parameters of the [`updateBlueprint(uint64,string)`](updateBlueprintCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct updateBlueprintReturn {}
    #[allow(
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
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (u64, alloy::sol_types::private::String);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<updateBlueprintCall> for UnderlyingRustTuple<'_> {
                fn from(value: updateBlueprintCall) -> Self {
                    (value.blueprintId, value.metadataUri)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for updateBlueprintCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        blueprintId: tuple.0,
                        metadataUri: tuple.1,
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
            impl ::core::convert::From<updateBlueprintReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: updateBlueprintReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for updateBlueprintReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for updateBlueprintCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<64>,
                alloy::sol_types::sol_data::String,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = updateBlueprintReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "updateBlueprint(uint64,string)";
            const SELECTOR: [u8; 4] = [229u8, 56u8, 218u8, 102u8];
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
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        &self.metadataUri,
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
    /**Function with signature `updateOperatorPreferences(uint64,bytes)` and selector `0xdfe62aba`.
```solidity
function updateOperatorPreferences(uint64 blueprintId, bytes memory preferences) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct updateOperatorPreferencesCall {
        #[allow(missing_docs)]
        pub blueprintId: u64,
        #[allow(missing_docs)]
        pub preferences: alloy::sol_types::private::Bytes,
    }
    ///Container type for the return parameters of the [`updateOperatorPreferences(uint64,bytes)`](updateOperatorPreferencesCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct updateOperatorPreferencesReturn {}
    #[allow(
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
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (u64, alloy::sol_types::private::Bytes);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<updateOperatorPreferencesCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: updateOperatorPreferencesCall) -> Self {
                    (value.blueprintId, value.preferences)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for updateOperatorPreferencesCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        blueprintId: tuple.0,
                        preferences: tuple.1,
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
            impl ::core::convert::From<updateOperatorPreferencesReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: updateOperatorPreferencesReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for updateOperatorPreferencesReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for updateOperatorPreferencesCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<64>,
                alloy::sol_types::sol_data::Bytes,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = updateOperatorPreferencesReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "updateOperatorPreferences(uint64,bytes)";
            const SELECTOR: [u8; 4] = [223u8, 230u8, 42u8, 186u8];
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
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.preferences,
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
    ///Container for all the [`ITangleFull`](self) function calls.
    pub enum ITangleFullCalls {
        #[allow(missing_docs)]
        addPermittedCaller(addPermittedCallerCall),
        #[allow(missing_docs)]
        approveService(approveServiceCall),
        #[allow(missing_docs)]
        approveServiceWithCommitments(approveServiceWithCommitmentsCall),
        #[allow(missing_docs)]
        billSubscription(billSubscriptionCall),
        #[allow(missing_docs)]
        blueprintCount(blueprintCountCall),
        #[allow(missing_docs)]
        blueprintOperatorCount(blueprintOperatorCountCall),
        #[allow(missing_docs)]
        claimRewards(claimRewardsCall),
        #[allow(missing_docs)]
        createBlueprint(createBlueprintCall),
        #[allow(missing_docs)]
        createBlueprintWithConfig(createBlueprintWithConfigCall),
        #[allow(missing_docs)]
        createServiceFromQuotes(createServiceFromQuotesCall),
        #[allow(missing_docs)]
        deactivateBlueprint(deactivateBlueprintCall),
        #[allow(missing_docs)]
        getBlueprint(getBlueprintCall),
        #[allow(missing_docs)]
        getBlueprintConfig(getBlueprintConfigCall),
        #[allow(missing_docs)]
        getJobCall(getJobCallCall),
        #[allow(missing_docs)]
        getOperatorRegistration(getOperatorRegistrationCall),
        #[allow(missing_docs)]
        getService(getServiceCall),
        #[allow(missing_docs)]
        getServiceOperator(getServiceOperatorCall),
        #[allow(missing_docs)]
        getServiceOperators(getServiceOperatorsCall),
        #[allow(missing_docs)]
        getServiceRequest(getServiceRequestCall),
        #[allow(missing_docs)]
        getServiceTotalExposure(getServiceTotalExposureCall),
        #[allow(missing_docs)]
        isOperatorRegistered(isOperatorRegisteredCall),
        #[allow(missing_docs)]
        isPermittedCaller(isPermittedCallerCall),
        #[allow(missing_docs)]
        isServiceActive(isServiceActiveCall),
        #[allow(missing_docs)]
        isServiceOperator(isServiceOperatorCall),
        #[allow(missing_docs)]
        joinService(joinServiceCall),
        #[allow(missing_docs)]
        leaveService(leaveServiceCall),
        #[allow(missing_docs)]
        pause(pauseCall),
        #[allow(missing_docs)]
        pendingRewards(pendingRewardsCall),
        #[allow(missing_docs)]
        proposeSlash(proposeSlashCall),
        #[allow(missing_docs)]
        registerOperator(registerOperatorCall),
        #[allow(missing_docs)]
        rejectService(rejectServiceCall),
        #[allow(missing_docs)]
        removePermittedCaller(removePermittedCallerCall),
        #[allow(missing_docs)]
        requestService(requestServiceCall),
        #[allow(missing_docs)]
        requestServiceWithExposure(requestServiceWithExposureCall),
        #[allow(missing_docs)]
        requestServiceWithSecurity(requestServiceWithSecurityCall),
        #[allow(missing_docs)]
        serviceCount(serviceCountCall),
        #[allow(missing_docs)]
        setOperatorOnline(setOperatorOnlineCall),
        #[allow(missing_docs)]
        setPaymentSplit(setPaymentSplitCall),
        #[allow(missing_docs)]
        setRestaking(setRestakingCall),
        #[allow(missing_docs)]
        setTreasury(setTreasuryCall),
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
        #[allow(missing_docs)]
        terminateService(terminateServiceCall),
        #[allow(missing_docs)]
        transferBlueprint(transferBlueprintCall),
        #[allow(missing_docs)]
        unpause(unpauseCall),
        #[allow(missing_docs)]
        unregisterOperator(unregisterOperatorCall),
        #[allow(missing_docs)]
        updateBlueprint(updateBlueprintCall),
        #[allow(missing_docs)]
        updateOperatorPreferences(updateOperatorPreferencesCall),
    }
    #[automatically_derived]
    impl ITangleFullCalls {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [6u8, 35u8, 117u8, 38u8],
            [10u8, 253u8, 55u8, 56u8],
            [16u8, 138u8, 125u8, 99u8],
            [26u8, 89u8, 248u8, 88u8],
            [35u8, 215u8, 179u8, 225u8],
            [43u8, 157u8, 167u8, 26u8],
            [45u8, 7u8, 230u8, 85u8],
            [46u8, 194u8, 189u8, 3u8],
            [47u8, 70u8, 39u8, 159u8],
            [48u8, 174u8, 40u8, 156u8],
            [49u8, 215u8, 162u8, 98u8],
            [52u8, 19u8, 232u8, 238u8],
            [55u8, 37u8, 0u8, 171u8],
            [61u8, 192u8, 213u8, 254u8],
            [62u8, 53u8, 69u8, 20u8],
            [63u8, 75u8, 168u8, 58u8],
            [68u8, 155u8, 184u8, 73u8],
            [86u8, 58u8, 137u8, 249u8],
            [91u8, 113u8, 201u8, 52u8],
            [95u8, 53u8, 153u8, 36u8],
            [95u8, 155u8, 77u8, 250u8],
            [97u8, 72u8, 35u8, 178u8],
            [100u8, 70u8, 37u8, 149u8],
            [103u8, 120u8, 175u8, 188u8],
            [107u8, 218u8, 66u8, 243u8],
            [109u8, 139u8, 136u8, 195u8],
            [120u8, 68u8, 134u8, 55u8],
            [126u8, 138u8, 193u8, 62u8],
            [129u8, 93u8, 106u8, 38u8],
            [132u8, 37u8, 36u8, 187u8],
            [132u8, 86u8, 203u8, 89u8],
            [138u8, 76u8, 247u8, 99u8],
            [141u8, 63u8, 101u8, 190u8],
            [153u8, 255u8, 54u8, 84u8],
            [163u8, 123u8, 146u8, 134u8],
            [165u8, 149u8, 178u8, 30u8],
            [166u8, 114u8, 188u8, 10u8],
            [170u8, 205u8, 186u8, 159u8],
            [183u8, 105u8, 109u8, 187u8],
            [184u8, 23u8, 65u8, 172u8],
            [187u8, 32u8, 127u8, 31u8],
            [195u8, 37u8, 174u8, 18u8],
            [198u8, 2u8, 212u8, 250u8],
            [204u8, 61u8, 25u8, 195u8],
            [207u8, 56u8, 6u8, 198u8],
            [223u8, 230u8, 42u8, 186u8],
            [229u8, 56u8, 218u8, 102u8],
            [231u8, 33u8, 70u8, 169u8],
            [236u8, 159u8, 15u8, 221u8],
            [240u8, 65u8, 26u8, 243u8],
            [240u8, 244u8, 66u8, 96u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for ITangleFullCalls {
        const NAME: &'static str = "ITangleFullCalls";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 51usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::addPermittedCaller(_) => {
                    <addPermittedCallerCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::approveService(_) => {
                    <approveServiceCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::approveServiceWithCommitments(_) => {
                    <approveServiceWithCommitmentsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::billSubscription(_) => {
                    <billSubscriptionCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::blueprintCount(_) => {
                    <blueprintCountCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::blueprintOperatorCount(_) => {
                    <blueprintOperatorCountCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::claimRewards(_) => {
                    <claimRewardsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::createBlueprint(_) => {
                    <createBlueprintCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::createBlueprintWithConfig(_) => {
                    <createBlueprintWithConfigCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::createServiceFromQuotes(_) => {
                    <createServiceFromQuotesCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::deactivateBlueprint(_) => {
                    <deactivateBlueprintCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getBlueprint(_) => {
                    <getBlueprintCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getBlueprintConfig(_) => {
                    <getBlueprintConfigCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getJobCall(_) => {
                    <getJobCallCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getOperatorRegistration(_) => {
                    <getOperatorRegistrationCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getService(_) => {
                    <getServiceCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getServiceOperator(_) => {
                    <getServiceOperatorCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getServiceOperators(_) => {
                    <getServiceOperatorsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getServiceRequest(_) => {
                    <getServiceRequestCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getServiceTotalExposure(_) => {
                    <getServiceTotalExposureCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::isOperatorRegistered(_) => {
                    <isOperatorRegisteredCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::isPermittedCaller(_) => {
                    <isPermittedCallerCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::isServiceActive(_) => {
                    <isServiceActiveCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::isServiceOperator(_) => {
                    <isServiceOperatorCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::joinService(_) => {
                    <joinServiceCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::leaveService(_) => {
                    <leaveServiceCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::pause(_) => <pauseCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::pendingRewards(_) => {
                    <pendingRewardsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::proposeSlash(_) => {
                    <proposeSlashCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::registerOperator(_) => {
                    <registerOperatorCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::rejectService(_) => {
                    <rejectServiceCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::removePermittedCaller(_) => {
                    <removePermittedCallerCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::requestService(_) => {
                    <requestServiceCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::requestServiceWithExposure(_) => {
                    <requestServiceWithExposureCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::requestServiceWithSecurity(_) => {
                    <requestServiceWithSecurityCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::serviceCount(_) => {
                    <serviceCountCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::setOperatorOnline(_) => {
                    <setOperatorOnlineCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::setPaymentSplit(_) => {
                    <setPaymentSplitCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::setRestaking(_) => {
                    <setRestakingCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::setTreasury(_) => {
                    <setTreasuryCall as alloy_sol_types::SolCall>::SELECTOR
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
                Self::terminateService(_) => {
                    <terminateServiceCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::transferBlueprint(_) => {
                    <transferBlueprintCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::unpause(_) => <unpauseCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::unregisterOperator(_) => {
                    <unregisterOperatorCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::updateBlueprint(_) => {
                    <updateBlueprintCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::updateOperatorPreferences(_) => {
                    <updateOperatorPreferencesCall as alloy_sol_types::SolCall>::SELECTOR
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
            ) -> alloy_sol_types::Result<ITangleFullCalls>] = &[
                {
                    fn serviceCount(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ITangleFullCalls> {
                        <serviceCountCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ITangleFullCalls::serviceCount)
                    }
                    serviceCount
                },
                {
                    fn unregisterOperator(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ITangleFullCalls> {
                        <unregisterOperatorCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ITangleFullCalls::unregisterOperator)
                    }
                    unregisterOperator
                },
                {
                    fn requestServiceWithExposure(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ITangleFullCalls> {
                        <requestServiceWithExposureCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ITangleFullCalls::requestServiceWithExposure)
                    }
                    requestServiceWithExposure
                },
                {
                    fn submitJobs(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ITangleFullCalls> {
                        <submitJobsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ITangleFullCalls::submitJobs)
                    }
                    submitJobs
                },
                {
                    fn approveServiceWithCommitments(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ITangleFullCalls> {
                        <approveServiceWithCommitmentsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ITangleFullCalls::approveServiceWithCommitments)
                    }
                    approveServiceWithCommitments
                },
                {
                    fn joinService(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ITangleFullCalls> {
                        <joinServiceCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ITangleFullCalls::joinService)
                    }
                    joinService
                },
                {
                    fn submitResult(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ITangleFullCalls> {
                        <submitResultCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ITangleFullCalls::submitResult)
                    }
                    submitResult
                },
                {
                    fn isPermittedCaller(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ITangleFullCalls> {
                        <isPermittedCallerCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ITangleFullCalls::isPermittedCaller)
                    }
                    isPermittedCaller
                },
                {
                    fn isServiceActive(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ITangleFullCalls> {
                        <isServiceActiveCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ITangleFullCalls::isServiceActive)
                    }
                    isServiceActive
                },
                {
                    fn proposeSlash(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ITangleFullCalls> {
                        <proposeSlashCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ITangleFullCalls::proposeSlash)
                    }
                    proposeSlash
                },
                {
                    fn pendingRewards(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ITangleFullCalls> {
                        <pendingRewardsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ITangleFullCalls::pendingRewards)
                    }
                    pendingRewards
                },
                {
                    fn submitJob(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ITangleFullCalls> {
                        <submitJobCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ITangleFullCalls::submitJob)
                    }
                    submitJob
                },
                {
                    fn claimRewards(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ITangleFullCalls> {
                        <claimRewardsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ITangleFullCalls::claimRewards)
                    }
                    claimRewards
                },
                {
                    fn getService(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ITangleFullCalls> {
                        <getServiceCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ITangleFullCalls::getService)
                    }
                    getService
                },
                {
                    fn getServiceTotalExposure(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ITangleFullCalls> {
                        <getServiceTotalExposureCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ITangleFullCalls::getServiceTotalExposure)
                    }
                    getServiceTotalExposure
                },
                {
                    fn unpause(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ITangleFullCalls> {
                        <unpauseCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ITangleFullCalls::unpause)
                    }
                    unpause
                },
                {
                    fn deactivateBlueprint(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ITangleFullCalls> {
                        <deactivateBlueprintCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ITangleFullCalls::deactivateBlueprint)
                    }
                    deactivateBlueprint
                },
                {
                    fn getBlueprintConfig(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ITangleFullCalls> {
                        <getBlueprintConfigCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ITangleFullCalls::getBlueprintConfig)
                    }
                    getBlueprintConfig
                },
                {
                    fn terminateService(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ITangleFullCalls> {
                        <terminateServiceCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ITangleFullCalls::terminateService)
                    }
                    terminateService
                },
                {
                    fn removePermittedCaller(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ITangleFullCalls> {
                        <removePermittedCallerCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ITangleFullCalls::removePermittedCaller)
                    }
                    removePermittedCaller
                },
                {
                    fn getServiceRequest(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ITangleFullCalls> {
                        <getServiceRequestCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ITangleFullCalls::getServiceRequest)
                    }
                    getServiceRequest
                },
                {
                    fn approveService(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ITangleFullCalls> {
                        <approveServiceCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ITangleFullCalls::approveService)
                    }
                    approveService
                },
                {
                    fn isOperatorRegistered(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ITangleFullCalls> {
                        <isOperatorRegisteredCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ITangleFullCalls::isOperatorRegistered)
                    }
                    isOperatorRegistered
                },
                {
                    fn leaveService(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ITangleFullCalls> {
                        <leaveServiceCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ITangleFullCalls::leaveService)
                    }
                    leaveService
                },
                {
                    fn getOperatorRegistration(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ITangleFullCalls> {
                        <getOperatorRegistrationCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ITangleFullCalls::getOperatorRegistration)
                    }
                    getOperatorRegistration
                },
                {
                    fn rejectService(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ITangleFullCalls> {
                        <rejectServiceCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ITangleFullCalls::rejectService)
                    }
                    rejectService
                },
                {
                    fn createBlueprintWithConfig(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ITangleFullCalls> {
                        <createBlueprintWithConfigCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ITangleFullCalls::createBlueprintWithConfig)
                    }
                    createBlueprintWithConfig
                },
                {
                    fn setOperatorOnline(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ITangleFullCalls> {
                        <setOperatorOnlineCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ITangleFullCalls::setOperatorOnline)
                    }
                    setOperatorOnline
                },
                {
                    fn addPermittedCaller(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ITangleFullCalls> {
                        <addPermittedCallerCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ITangleFullCalls::addPermittedCaller)
                    }
                    addPermittedCaller
                },
                {
                    fn getServiceOperator(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ITangleFullCalls> {
                        <getServiceOperatorCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ITangleFullCalls::getServiceOperator)
                    }
                    getServiceOperator
                },
                {
                    fn pause(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ITangleFullCalls> {
                        <pauseCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ITangleFullCalls::pause)
                    }
                    pause
                },
                {
                    fn transferBlueprint(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ITangleFullCalls> {
                        <transferBlueprintCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ITangleFullCalls::transferBlueprint)
                    }
                    transferBlueprint
                },
                {
                    fn blueprintOperatorCount(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ITangleFullCalls> {
                        <blueprintOperatorCountCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ITangleFullCalls::blueprintOperatorCount)
                    }
                    blueprintOperatorCount
                },
                {
                    fn setRestaking(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ITangleFullCalls> {
                        <setRestakingCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ITangleFullCalls::setRestaking)
                    }
                    setRestaking
                },
                {
                    fn requestService(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ITangleFullCalls> {
                        <requestServiceCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ITangleFullCalls::requestService)
                    }
                    requestService
                },
                {
                    fn isServiceOperator(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ITangleFullCalls> {
                        <isServiceOperatorCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ITangleFullCalls::isServiceOperator)
                    }
                    isServiceOperator
                },
                {
                    fn getJobCall(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ITangleFullCalls> {
                        <getJobCallCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ITangleFullCalls::getJobCall)
                    }
                    getJobCall
                },
                {
                    fn submitResults(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ITangleFullCalls> {
                        <submitResultsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ITangleFullCalls::submitResults)
                    }
                    submitResults
                },
                {
                    fn getBlueprint(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ITangleFullCalls> {
                        <getBlueprintCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ITangleFullCalls::getBlueprint)
                    }
                    getBlueprint
                },
                {
                    fn setPaymentSplit(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ITangleFullCalls> {
                        <setPaymentSplitCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ITangleFullCalls::setPaymentSplit)
                    }
                    setPaymentSplit
                },
                {
                    fn getServiceOperators(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ITangleFullCalls> {
                        <getServiceOperatorsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ITangleFullCalls::getServiceOperators)
                    }
                    getServiceOperators
                },
                {
                    fn submitAggregatedResult(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ITangleFullCalls> {
                        <submitAggregatedResultCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ITangleFullCalls::submitAggregatedResult)
                    }
                    submitAggregatedResult
                },
                {
                    fn blueprintCount(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ITangleFullCalls> {
                        <blueprintCountCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ITangleFullCalls::blueprintCount)
                    }
                    blueprintCount
                },
                {
                    fn registerOperator(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ITangleFullCalls> {
                        <registerOperatorCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ITangleFullCalls::registerOperator)
                    }
                    registerOperator
                },
                {
                    fn billSubscription(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ITangleFullCalls> {
                        <billSubscriptionCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ITangleFullCalls::billSubscription)
                    }
                    billSubscription
                },
                {
                    fn updateOperatorPreferences(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ITangleFullCalls> {
                        <updateOperatorPreferencesCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ITangleFullCalls::updateOperatorPreferences)
                    }
                    updateOperatorPreferences
                },
                {
                    fn updateBlueprint(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ITangleFullCalls> {
                        <updateBlueprintCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ITangleFullCalls::updateBlueprint)
                    }
                    updateBlueprint
                },
                {
                    fn createBlueprint(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ITangleFullCalls> {
                        <createBlueprintCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ITangleFullCalls::createBlueprint)
                    }
                    createBlueprint
                },
                {
                    fn requestServiceWithSecurity(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ITangleFullCalls> {
                        <requestServiceWithSecurityCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ITangleFullCalls::requestServiceWithSecurity)
                    }
                    requestServiceWithSecurity
                },
                {
                    fn createServiceFromQuotes(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ITangleFullCalls> {
                        <createServiceFromQuotesCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ITangleFullCalls::createServiceFromQuotes)
                    }
                    createServiceFromQuotes
                },
                {
                    fn setTreasury(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ITangleFullCalls> {
                        <setTreasuryCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ITangleFullCalls::setTreasury)
                    }
                    setTreasury
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
                Self::addPermittedCaller(inner) => {
                    <addPermittedCallerCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::approveService(inner) => {
                    <approveServiceCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::approveServiceWithCommitments(inner) => {
                    <approveServiceWithCommitmentsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::billSubscription(inner) => {
                    <billSubscriptionCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::blueprintCount(inner) => {
                    <blueprintCountCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::blueprintOperatorCount(inner) => {
                    <blueprintOperatorCountCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::claimRewards(inner) => {
                    <claimRewardsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::createBlueprint(inner) => {
                    <createBlueprintCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::createBlueprintWithConfig(inner) => {
                    <createBlueprintWithConfigCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::createServiceFromQuotes(inner) => {
                    <createServiceFromQuotesCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::deactivateBlueprint(inner) => {
                    <deactivateBlueprintCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getBlueprint(inner) => {
                    <getBlueprintCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getBlueprintConfig(inner) => {
                    <getBlueprintConfigCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getJobCall(inner) => {
                    <getJobCallCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::getOperatorRegistration(inner) => {
                    <getOperatorRegistrationCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getService(inner) => {
                    <getServiceCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::getServiceOperator(inner) => {
                    <getServiceOperatorCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getServiceOperators(inner) => {
                    <getServiceOperatorsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getServiceRequest(inner) => {
                    <getServiceRequestCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getServiceTotalExposure(inner) => {
                    <getServiceTotalExposureCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::isOperatorRegistered(inner) => {
                    <isOperatorRegisteredCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::isPermittedCaller(inner) => {
                    <isPermittedCallerCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::isServiceActive(inner) => {
                    <isServiceActiveCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::isServiceOperator(inner) => {
                    <isServiceOperatorCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::joinService(inner) => {
                    <joinServiceCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::leaveService(inner) => {
                    <leaveServiceCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::pause(inner) => {
                    <pauseCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::pendingRewards(inner) => {
                    <pendingRewardsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::proposeSlash(inner) => {
                    <proposeSlashCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::registerOperator(inner) => {
                    <registerOperatorCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::rejectService(inner) => {
                    <rejectServiceCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::removePermittedCaller(inner) => {
                    <removePermittedCallerCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::requestService(inner) => {
                    <requestServiceCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::requestServiceWithExposure(inner) => {
                    <requestServiceWithExposureCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::requestServiceWithSecurity(inner) => {
                    <requestServiceWithSecurityCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::serviceCount(inner) => {
                    <serviceCountCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::setOperatorOnline(inner) => {
                    <setOperatorOnlineCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::setPaymentSplit(inner) => {
                    <setPaymentSplitCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::setRestaking(inner) => {
                    <setRestakingCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::setTreasury(inner) => {
                    <setTreasuryCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
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
                Self::terminateService(inner) => {
                    <terminateServiceCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::transferBlueprint(inner) => {
                    <transferBlueprintCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::unpause(inner) => {
                    <unpauseCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::unregisterOperator(inner) => {
                    <unregisterOperatorCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::updateBlueprint(inner) => {
                    <updateBlueprintCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::updateOperatorPreferences(inner) => {
                    <updateOperatorPreferencesCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
            }
        }
        #[inline]
        fn abi_encode_raw(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
            match self {
                Self::addPermittedCaller(inner) => {
                    <addPermittedCallerCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::approveService(inner) => {
                    <approveServiceCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::approveServiceWithCommitments(inner) => {
                    <approveServiceWithCommitmentsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::billSubscription(inner) => {
                    <billSubscriptionCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::blueprintCount(inner) => {
                    <blueprintCountCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::blueprintOperatorCount(inner) => {
                    <blueprintOperatorCountCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::claimRewards(inner) => {
                    <claimRewardsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::createBlueprint(inner) => {
                    <createBlueprintCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::createBlueprintWithConfig(inner) => {
                    <createBlueprintWithConfigCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::createServiceFromQuotes(inner) => {
                    <createServiceFromQuotesCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::deactivateBlueprint(inner) => {
                    <deactivateBlueprintCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getBlueprint(inner) => {
                    <getBlueprintCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getBlueprintConfig(inner) => {
                    <getBlueprintConfigCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getJobCall(inner) => {
                    <getJobCallCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getOperatorRegistration(inner) => {
                    <getOperatorRegistrationCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getService(inner) => {
                    <getServiceCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getServiceOperator(inner) => {
                    <getServiceOperatorCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getServiceOperators(inner) => {
                    <getServiceOperatorsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getServiceRequest(inner) => {
                    <getServiceRequestCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getServiceTotalExposure(inner) => {
                    <getServiceTotalExposureCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::isOperatorRegistered(inner) => {
                    <isOperatorRegisteredCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::isPermittedCaller(inner) => {
                    <isPermittedCallerCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::isServiceActive(inner) => {
                    <isServiceActiveCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::isServiceOperator(inner) => {
                    <isServiceOperatorCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::joinService(inner) => {
                    <joinServiceCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::leaveService(inner) => {
                    <leaveServiceCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::pause(inner) => {
                    <pauseCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::pendingRewards(inner) => {
                    <pendingRewardsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::proposeSlash(inner) => {
                    <proposeSlashCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::rejectService(inner) => {
                    <rejectServiceCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::removePermittedCaller(inner) => {
                    <removePermittedCallerCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::requestService(inner) => {
                    <requestServiceCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::requestServiceWithExposure(inner) => {
                    <requestServiceWithExposureCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::requestServiceWithSecurity(inner) => {
                    <requestServiceWithSecurityCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::serviceCount(inner) => {
                    <serviceCountCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::setOperatorOnline(inner) => {
                    <setOperatorOnlineCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::setPaymentSplit(inner) => {
                    <setPaymentSplitCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::setRestaking(inner) => {
                    <setRestakingCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::setTreasury(inner) => {
                    <setTreasuryCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::terminateService(inner) => {
                    <terminateServiceCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::transferBlueprint(inner) => {
                    <transferBlueprintCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::unpause(inner) => {
                    <unpauseCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::unregisterOperator(inner) => {
                    <unregisterOperatorCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::updateBlueprint(inner) => {
                    <updateBlueprintCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::updateOperatorPreferences(inner) => {
                    <updateOperatorPreferencesCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
            }
        }
    }
    ///Container for all the [`ITangleFull`](self) events.
    pub enum ITangleFullEvents {
        #[allow(missing_docs)]
        BlueprintCreated(BlueprintCreated),
        #[allow(missing_docs)]
        BlueprintDeactivated(BlueprintDeactivated),
        #[allow(missing_docs)]
        BlueprintTransferred(BlueprintTransferred),
        #[allow(missing_docs)]
        BlueprintUpdated(BlueprintUpdated),
        #[allow(missing_docs)]
        JobCompleted(JobCompleted),
        #[allow(missing_docs)]
        JobResultSubmitted(JobResultSubmitted),
        #[allow(missing_docs)]
        JobSubmitted(JobSubmitted),
        #[allow(missing_docs)]
        OperatorJoinedService(OperatorJoinedService),
        #[allow(missing_docs)]
        OperatorLeftService(OperatorLeftService),
        #[allow(missing_docs)]
        OperatorOnlineStatusChanged(OperatorOnlineStatusChanged),
        #[allow(missing_docs)]
        OperatorPreferencesUpdated(OperatorPreferencesUpdated),
        #[allow(missing_docs)]
        OperatorRegistered(OperatorRegistered),
        #[allow(missing_docs)]
        OperatorUnregistered(OperatorUnregistered),
        #[allow(missing_docs)]
        RewardsClaimed(RewardsClaimed),
        #[allow(missing_docs)]
        RewardsDistributed(RewardsDistributed),
        #[allow(missing_docs)]
        ServiceActivated(ServiceActivated),
        #[allow(missing_docs)]
        ServiceApproved(ServiceApproved),
        #[allow(missing_docs)]
        ServiceCreatedFromQuotes(ServiceCreatedFromQuotes),
        #[allow(missing_docs)]
        ServiceRejected(ServiceRejected),
        #[allow(missing_docs)]
        ServiceRequested(ServiceRequested),
        #[allow(missing_docs)]
        ServiceRequestedWithSecurity(ServiceRequestedWithSecurity),
        #[allow(missing_docs)]
        ServiceTerminated(ServiceTerminated),
        #[allow(missing_docs)]
        SlashExecuted(SlashExecuted),
        #[allow(missing_docs)]
        SlashProposed(SlashProposed),
        #[allow(missing_docs)]
        SubscriptionBilled(SubscriptionBilled),
    }
    #[automatically_derived]
    impl ITangleFullEvents {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 32usize]] = &[
            [
                15u8,
                182u8,
                234u8,
                65u8,
                190u8,
                126u8,
                75u8,
                77u8,
                3u8,
                15u8,
                243u8,
                148u8,
                226u8,
                128u8,
                151u8,
                58u8,
                1u8,
                251u8,
                142u8,
                44u8,
                169u8,
                218u8,
                124u8,
                144u8,
                10u8,
                227u8,
                9u8,
                102u8,
                154u8,
                23u8,
                117u8,
                25u8,
            ],
            [
                36u8,
                87u8,
                145u8,
                135u8,
                150u8,
                7u8,
                132u8,
                64u8,
                6u8,
                131u8,
                133u8,
                231u8,
                62u8,
                66u8,
                37u8,
                57u8,
                200u8,
                79u8,
                47u8,
                144u8,
                69u8,
                221u8,
                68u8,
                46u8,
                18u8,
                61u8,
                66u8,
                132u8,
                37u8,
                138u8,
                226u8,
                74u8,
            ],
            [
                36u8,
                220u8,
                72u8,
                91u8,
                239u8,
                4u8,
                180u8,
                215u8,
                144u8,
                217u8,
                122u8,
                193u8,
                3u8,
                129u8,
                216u8,
                217u8,
                230u8,
                108u8,
                86u8,
                221u8,
                111u8,
                186u8,
                153u8,
                49u8,
                183u8,
                3u8,
                202u8,
                22u8,
                149u8,
                17u8,
                119u8,
                138u8,
            ],
            [
                38u8,
                197u8,
                196u8,
                38u8,
                223u8,
                69u8,
                45u8,
                104u8,
                227u8,
                179u8,
                210u8,
                124u8,
                123u8,
                6u8,
                154u8,
                98u8,
                205u8,
                38u8,
                138u8,
                217u8,
                225u8,
                205u8,
                85u8,
                201u8,
                201u8,
                242u8,
                57u8,
                50u8,
                8u8,
                144u8,
                222u8,
                35u8,
            ],
            [
                48u8,
                252u8,
                69u8,
                224u8,
                90u8,
                51u8,
                253u8,
                144u8,
                104u8,
                207u8,
                106u8,
                225u8,
                168u8,
                243u8,
                219u8,
                43u8,
                157u8,
                21u8,
                55u8,
                45u8,
                214u8,
                73u8,
                78u8,
                204u8,
                200u8,
                205u8,
                93u8,
                48u8,
                95u8,
                223u8,
                56u8,
                179u8,
            ],
            [
                51u8,
                61u8,
                162u8,
                69u8,
                43u8,
                119u8,
                13u8,
                130u8,
                151u8,
                205u8,
                150u8,
                48u8,
                185u8,
                58u8,
                42u8,
                145u8,
                144u8,
                200u8,
                36u8,
                131u8,
                50u8,
                122u8,
                22u8,
                71u8,
                47u8,
                98u8,
                30u8,
                252u8,
                17u8,
                172u8,
                224u8,
                16u8,
            ],
            [
                133u8,
                71u8,
                233u8,
                75u8,
                217u8,
                92u8,
                167u8,
                211u8,
                199u8,
                163u8,
                190u8,
                155u8,
                102u8,
                162u8,
                158u8,
                111u8,
                54u8,
                21u8,
                27u8,
                229u8,
                74u8,
                37u8,
                99u8,
                227u8,
                80u8,
                152u8,
                67u8,
                136u8,
                97u8,
                164u8,
                200u8,
                113u8,
            ],
            [
                136u8,
                228u8,
                110u8,
                149u8,
                188u8,
                223u8,
                16u8,
                32u8,
                192u8,
                202u8,
                249u8,
                243u8,
                221u8,
                95u8,
                222u8,
                105u8,
                196u8,
                29u8,
                33u8,
                81u8,
                219u8,
                188u8,
                160u8,
                167u8,
                124u8,
                3u8,
                196u8,
                226u8,
                139u8,
                77u8,
                62u8,
                193u8,
            ],
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
                140u8,
                229u8,
                62u8,
                75u8,
                96u8,
                2u8,
                255u8,
                9u8,
                14u8,
                100u8,
                29u8,
                222u8,
                205u8,
                175u8,
                126u8,
                221u8,
                40u8,
                19u8,
                169u8,
                142u8,
                25u8,
                19u8,
                159u8,
                167u8,
                209u8,
                199u8,
                159u8,
                204u8,
                195u8,
                196u8,
                222u8,
                173u8,
            ],
            [
                147u8,
                0u8,
                59u8,
                72u8,
                18u8,
                249u8,
                206u8,
                82u8,
                192u8,
                88u8,
                203u8,
                222u8,
                217u8,
                13u8,
                112u8,
                191u8,
                23u8,
                91u8,
                79u8,
                119u8,
                213u8,
                153u8,
                142u8,
                175u8,
                159u8,
                38u8,
                60u8,
                114u8,
                253u8,
                204u8,
                135u8,
                202u8,
            ],
            [
                148u8,
                107u8,
                200u8,
                99u8,
                238u8,
                8u8,
                198u8,
                193u8,
                74u8,
                244u8,
                231u8,
                1u8,
                99u8,
                189u8,
                21u8,
                151u8,
                199u8,
                17u8,
                34u8,
                242u8,
                240u8,
                88u8,
                154u8,
                188u8,
                230u8,
                10u8,
                120u8,
                224u8,
                30u8,
                140u8,
                251u8,
                116u8,
            ],
            [
                169u8,
                207u8,
                153u8,
                196u8,
                199u8,
                91u8,
                240u8,
                135u8,
                69u8,
                118u8,
                230u8,
                238u8,
                112u8,
                176u8,
                11u8,
                146u8,
                14u8,
                156u8,
                54u8,
                85u8,
                30u8,
                77u8,
                108u8,
                23u8,
                105u8,
                78u8,
                202u8,
                166u8,
                132u8,
                98u8,
                77u8,
                43u8,
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
                198u8,
                48u8,
                80u8,
                185u8,
                99u8,
                236u8,
                110u8,
                44u8,
                243u8,
                109u8,
                249u8,
                14u8,
                202u8,
                202u8,
                164u8,
                50u8,
                18u8,
                192u8,
                231u8,
                75u8,
                140u8,
                58u8,
                88u8,
                160u8,
                248u8,
                204u8,
                227u8,
                189u8,
                90u8,
                42u8,
                118u8,
                117u8,
            ],
            [
                206u8,
                218u8,
                26u8,
                68u8,
                234u8,
                212u8,
                218u8,
                61u8,
                58u8,
                186u8,
                170u8,
                148u8,
                26u8,
                250u8,
                109u8,
                149u8,
                48u8,
                253u8,
                132u8,
                221u8,
                142u8,
                153u8,
                56u8,
                92u8,
                145u8,
                131u8,
                217u8,
                134u8,
                220u8,
                171u8,
                152u8,
                15u8,
            ],
            [
                210u8,
                185u8,
                88u8,
                161u8,
                176u8,
                185u8,
                120u8,
                206u8,
                222u8,
                212u8,
                152u8,
                107u8,
                79u8,
                73u8,
                200u8,
                178u8,
                95u8,
                189u8,
                110u8,
                129u8,
                44u8,
                184u8,
                146u8,
                94u8,
                19u8,
                239u8,
                199u8,
                49u8,
                234u8,
                74u8,
                162u8,
                77u8,
            ],
            [
                212u8,
                12u8,
                6u8,
                238u8,
                178u8,
                13u8,
                33u8,
                157u8,
                46u8,
                51u8,
                166u8,
                223u8,
                82u8,
                160u8,
                107u8,
                62u8,
                209u8,
                250u8,
                13u8,
                220u8,
                34u8,
                4u8,
                120u8,
                172u8,
                152u8,
                224u8,
                4u8,
                100u8,
                228u8,
                131u8,
                73u8,
                91u8,
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
            [
                225u8,
                66u8,
                134u8,
                243u8,
                173u8,
                73u8,
                173u8,
                166u8,
                208u8,
                145u8,
                26u8,
                221u8,
                168u8,
                239u8,
                144u8,
                97u8,
                105u8,
                153u8,
                4u8,
                91u8,
                222u8,
                42u8,
                51u8,
                227u8,
                145u8,
                167u8,
                181u8,
                174u8,
                101u8,
                137u8,
                231u8,
                137u8,
            ],
            [
                233u8,
                185u8,
                217u8,
                65u8,
                211u8,
                23u8,
                98u8,
                235u8,
                235u8,
                122u8,
                0u8,
                0u8,
                139u8,
                76u8,
                249u8,
                94u8,
                154u8,
                25u8,
                37u8,
                92u8,
                182u8,
                105u8,
                92u8,
                157u8,
                53u8,
                33u8,
                92u8,
                145u8,
                74u8,
                126u8,
                132u8,
                134u8,
            ],
            [
                239u8,
                174u8,
                43u8,
                67u8,
                197u8,
                156u8,
                186u8,
                182u8,
                201u8,
                43u8,
                187u8,
                227u8,
                244u8,
                74u8,
                110u8,
                115u8,
                204u8,
                156u8,
                203u8,
                191u8,
                148u8,
                6u8,
                92u8,
                220u8,
                56u8,
                130u8,
                151u8,
                165u8,
                65u8,
                201u8,
                151u8,
                110u8,
            ],
            [
                240u8,
                213u8,
                148u8,
                58u8,
                16u8,
                161u8,
                125u8,
                133u8,
                230u8,
                155u8,
                20u8,
                186u8,
                179u8,
                219u8,
                105u8,
                206u8,
                255u8,
                63u8,
                145u8,
                155u8,
                183u8,
                252u8,
                247u8,
                154u8,
                175u8,
                239u8,
                252u8,
                57u8,
                186u8,
                26u8,
                126u8,
                175u8,
            ],
            [
                244u8,
                173u8,
                125u8,
                110u8,
                151u8,
                114u8,
                213u8,
                146u8,
                210u8,
                166u8,
                201u8,
                68u8,
                157u8,
                140u8,
                196u8,
                11u8,
                103u8,
                185u8,
                240u8,
                228u8,
                184u8,
                28u8,
                60u8,
                118u8,
                151u8,
                156u8,
                121u8,
                155u8,
                59u8,
                235u8,
                52u8,
                194u8,
            ],
            [
                252u8,
                48u8,
                205u8,
                222u8,
                163u8,
                142u8,
                43u8,
                244u8,
                214u8,
                234u8,
                125u8,
                63u8,
                158u8,
                211u8,
                182u8,
                173u8,
                127u8,
                23u8,
                100u8,
                25u8,
                244u8,
                150u8,
                59u8,
                216u8,
                19u8,
                24u8,
                6u8,
                122u8,
                74u8,
                238u8,
                115u8,
                254u8,
            ],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolEventInterface for ITangleFullEvents {
        const NAME: &'static str = "ITangleFullEvents";
        const COUNT: usize = 25usize;
        fn decode_raw_log(
            topics: &[alloy_sol_types::Word],
            data: &[u8],
            validate: bool,
        ) -> alloy_sol_types::Result<Self> {
            match topics.first().copied() {
                Some(<BlueprintCreated as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <BlueprintCreated as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::BlueprintCreated)
                }
                Some(
                    <BlueprintDeactivated as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <BlueprintDeactivated as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::BlueprintDeactivated)
                }
                Some(
                    <BlueprintTransferred as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <BlueprintTransferred as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::BlueprintTransferred)
                }
                Some(<BlueprintUpdated as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <BlueprintUpdated as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::BlueprintUpdated)
                }
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
                Some(
                    <OperatorJoinedService as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <OperatorJoinedService as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::OperatorJoinedService)
                }
                Some(
                    <OperatorLeftService as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <OperatorLeftService as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::OperatorLeftService)
                }
                Some(
                    <OperatorOnlineStatusChanged as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <OperatorOnlineStatusChanged as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::OperatorOnlineStatusChanged)
                }
                Some(
                    <OperatorPreferencesUpdated as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <OperatorPreferencesUpdated as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::OperatorPreferencesUpdated)
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
                    <OperatorUnregistered as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <OperatorUnregistered as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::OperatorUnregistered)
                }
                Some(<RewardsClaimed as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <RewardsClaimed as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::RewardsClaimed)
                }
                Some(
                    <RewardsDistributed as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <RewardsDistributed as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::RewardsDistributed)
                }
                Some(<ServiceActivated as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <ServiceActivated as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::ServiceActivated)
                }
                Some(<ServiceApproved as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <ServiceApproved as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::ServiceApproved)
                }
                Some(
                    <ServiceCreatedFromQuotes as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <ServiceCreatedFromQuotes as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::ServiceCreatedFromQuotes)
                }
                Some(<ServiceRejected as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <ServiceRejected as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::ServiceRejected)
                }
                Some(<ServiceRequested as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <ServiceRequested as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::ServiceRequested)
                }
                Some(
                    <ServiceRequestedWithSecurity as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <ServiceRequestedWithSecurity as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::ServiceRequestedWithSecurity)
                }
                Some(
                    <ServiceTerminated as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <ServiceTerminated as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::ServiceTerminated)
                }
                Some(<SlashExecuted as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <SlashExecuted as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::SlashExecuted)
                }
                Some(<SlashProposed as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <SlashProposed as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::SlashProposed)
                }
                Some(
                    <SubscriptionBilled as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <SubscriptionBilled as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::SubscriptionBilled)
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
    impl alloy_sol_types::private::IntoLogData for ITangleFullEvents {
        fn to_log_data(&self) -> alloy_sol_types::private::LogData {
            match self {
                Self::BlueprintCreated(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::BlueprintDeactivated(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::BlueprintTransferred(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::BlueprintUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::JobCompleted(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::JobResultSubmitted(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::JobSubmitted(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::OperatorJoinedService(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::OperatorLeftService(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::OperatorOnlineStatusChanged(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::OperatorPreferencesUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::OperatorRegistered(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::OperatorUnregistered(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::RewardsClaimed(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::RewardsDistributed(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::ServiceActivated(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::ServiceApproved(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::ServiceCreatedFromQuotes(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::ServiceRejected(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::ServiceRequested(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::ServiceRequestedWithSecurity(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::ServiceTerminated(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::SlashExecuted(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::SlashProposed(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::SubscriptionBilled(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
            }
        }
        fn into_log_data(self) -> alloy_sol_types::private::LogData {
            match self {
                Self::BlueprintCreated(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::BlueprintDeactivated(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::BlueprintTransferred(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::BlueprintUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::JobCompleted(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::JobResultSubmitted(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::JobSubmitted(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::OperatorJoinedService(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::OperatorLeftService(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::OperatorOnlineStatusChanged(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::OperatorPreferencesUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::OperatorRegistered(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::OperatorUnregistered(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::RewardsClaimed(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::RewardsDistributed(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::ServiceActivated(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::ServiceApproved(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::ServiceCreatedFromQuotes(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::ServiceRejected(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::ServiceRequested(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::ServiceRequestedWithSecurity(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::ServiceTerminated(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::SlashExecuted(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::SlashProposed(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::SubscriptionBilled(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
            }
        }
    }
    use alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`ITangleFull`](self) contract instance.

See the [wrapper's documentation](`ITangleFullInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> ITangleFullInstance<T, P, N> {
        ITangleFullInstance::<T, P, N>::new(address, provider)
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
        Output = alloy_contract::Result<ITangleFullInstance<T, P, N>>,
    > {
        ITangleFullInstance::<T, P, N>::deploy(provider)
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
        ITangleFullInstance::<T, P, N>::deploy_builder(provider)
    }
    /**A [`ITangleFull`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`ITangleFull`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct ITangleFullInstance<T, P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for ITangleFullInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("ITangleFullInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > ITangleFullInstance<T, P, N> {
        /**Creates a new wrapper around an on-chain [`ITangleFull`](self) contract instance.

See the [wrapper's documentation](`ITangleFullInstance`) for more details.*/
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
        ) -> alloy_contract::Result<ITangleFullInstance<T, P, N>> {
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
    impl<T, P: ::core::clone::Clone, N> ITangleFullInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> ITangleFullInstance<T, P, N> {
            ITangleFullInstance {
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
    > ITangleFullInstance<T, P, N> {
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
        ///Creates a new call builder for the [`addPermittedCaller`] function.
        pub fn addPermittedCaller(
            &self,
            serviceId: u64,
            caller: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, addPermittedCallerCall, N> {
            self.call_builder(
                &addPermittedCallerCall {
                    serviceId,
                    caller,
                },
            )
        }
        ///Creates a new call builder for the [`approveService`] function.
        pub fn approveService(
            &self,
            requestId: u64,
            restakingPercent: u8,
        ) -> alloy_contract::SolCallBuilder<T, &P, approveServiceCall, N> {
            self.call_builder(
                &approveServiceCall {
                    requestId,
                    restakingPercent,
                },
            )
        }
        ///Creates a new call builder for the [`approveServiceWithCommitments`] function.
        pub fn approveServiceWithCommitments(
            &self,
            requestId: u64,
            commitments: alloy::sol_types::private::Vec<
                <Types::AssetSecurityCommitment as alloy::sol_types::SolType>::RustType,
            >,
        ) -> alloy_contract::SolCallBuilder<
            T,
            &P,
            approveServiceWithCommitmentsCall,
            N,
        > {
            self.call_builder(
                &approveServiceWithCommitmentsCall {
                    requestId,
                    commitments,
                },
            )
        }
        ///Creates a new call builder for the [`billSubscription`] function.
        pub fn billSubscription(
            &self,
            serviceId: u64,
        ) -> alloy_contract::SolCallBuilder<T, &P, billSubscriptionCall, N> {
            self.call_builder(&billSubscriptionCall { serviceId })
        }
        ///Creates a new call builder for the [`blueprintCount`] function.
        pub fn blueprintCount(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, blueprintCountCall, N> {
            self.call_builder(&blueprintCountCall {})
        }
        ///Creates a new call builder for the [`blueprintOperatorCount`] function.
        pub fn blueprintOperatorCount(
            &self,
            blueprintId: u64,
        ) -> alloy_contract::SolCallBuilder<T, &P, blueprintOperatorCountCall, N> {
            self.call_builder(
                &blueprintOperatorCountCall {
                    blueprintId,
                },
            )
        }
        ///Creates a new call builder for the [`claimRewards`] function.
        pub fn claimRewards(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, claimRewardsCall, N> {
            self.call_builder(&claimRewardsCall {})
        }
        ///Creates a new call builder for the [`createBlueprint`] function.
        pub fn createBlueprint(
            &self,
            metadataUri: alloy::sol_types::private::String,
            manager: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, createBlueprintCall, N> {
            self.call_builder(
                &createBlueprintCall {
                    metadataUri,
                    manager,
                },
            )
        }
        ///Creates a new call builder for the [`createBlueprintWithConfig`] function.
        pub fn createBlueprintWithConfig(
            &self,
            metadataUri: alloy::sol_types::private::String,
            manager: alloy::sol_types::private::Address,
            config: <Types::BlueprintConfig as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<T, &P, createBlueprintWithConfigCall, N> {
            self.call_builder(
                &createBlueprintWithConfigCall {
                    metadataUri,
                    manager,
                    config,
                },
            )
        }
        ///Creates a new call builder for the [`createServiceFromQuotes`] function.
        pub fn createServiceFromQuotes(
            &self,
            blueprintId: u64,
            quotes: alloy::sol_types::private::Vec<
                <Types::SignedQuote as alloy::sol_types::SolType>::RustType,
            >,
            config: alloy::sol_types::private::Bytes,
            permittedCallers: alloy::sol_types::private::Vec<
                alloy::sol_types::private::Address,
            >,
            ttl: u64,
        ) -> alloy_contract::SolCallBuilder<T, &P, createServiceFromQuotesCall, N> {
            self.call_builder(
                &createServiceFromQuotesCall {
                    blueprintId,
                    quotes,
                    config,
                    permittedCallers,
                    ttl,
                },
            )
        }
        ///Creates a new call builder for the [`deactivateBlueprint`] function.
        pub fn deactivateBlueprint(
            &self,
            blueprintId: u64,
        ) -> alloy_contract::SolCallBuilder<T, &P, deactivateBlueprintCall, N> {
            self.call_builder(
                &deactivateBlueprintCall {
                    blueprintId,
                },
            )
        }
        ///Creates a new call builder for the [`getBlueprint`] function.
        pub fn getBlueprint(
            &self,
            blueprintId: u64,
        ) -> alloy_contract::SolCallBuilder<T, &P, getBlueprintCall, N> {
            self.call_builder(&getBlueprintCall { blueprintId })
        }
        ///Creates a new call builder for the [`getBlueprintConfig`] function.
        pub fn getBlueprintConfig(
            &self,
            blueprintId: u64,
        ) -> alloy_contract::SolCallBuilder<T, &P, getBlueprintConfigCall, N> {
            self.call_builder(
                &getBlueprintConfigCall {
                    blueprintId,
                },
            )
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
        ///Creates a new call builder for the [`getOperatorRegistration`] function.
        pub fn getOperatorRegistration(
            &self,
            blueprintId: u64,
            operator: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, getOperatorRegistrationCall, N> {
            self.call_builder(
                &getOperatorRegistrationCall {
                    blueprintId,
                    operator,
                },
            )
        }
        ///Creates a new call builder for the [`getService`] function.
        pub fn getService(
            &self,
            serviceId: u64,
        ) -> alloy_contract::SolCallBuilder<T, &P, getServiceCall, N> {
            self.call_builder(&getServiceCall { serviceId })
        }
        ///Creates a new call builder for the [`getServiceOperator`] function.
        pub fn getServiceOperator(
            &self,
            serviceId: u64,
            operator: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, getServiceOperatorCall, N> {
            self.call_builder(
                &getServiceOperatorCall {
                    serviceId,
                    operator,
                },
            )
        }
        ///Creates a new call builder for the [`getServiceOperators`] function.
        pub fn getServiceOperators(
            &self,
            serviceId: u64,
        ) -> alloy_contract::SolCallBuilder<T, &P, getServiceOperatorsCall, N> {
            self.call_builder(
                &getServiceOperatorsCall {
                    serviceId,
                },
            )
        }
        ///Creates a new call builder for the [`getServiceRequest`] function.
        pub fn getServiceRequest(
            &self,
            requestId: u64,
        ) -> alloy_contract::SolCallBuilder<T, &P, getServiceRequestCall, N> {
            self.call_builder(&getServiceRequestCall { requestId })
        }
        ///Creates a new call builder for the [`getServiceTotalExposure`] function.
        pub fn getServiceTotalExposure(
            &self,
            serviceId: u64,
        ) -> alloy_contract::SolCallBuilder<T, &P, getServiceTotalExposureCall, N> {
            self.call_builder(
                &getServiceTotalExposureCall {
                    serviceId,
                },
            )
        }
        ///Creates a new call builder for the [`isOperatorRegistered`] function.
        pub fn isOperatorRegistered(
            &self,
            blueprintId: u64,
            operator: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, isOperatorRegisteredCall, N> {
            self.call_builder(
                &isOperatorRegisteredCall {
                    blueprintId,
                    operator,
                },
            )
        }
        ///Creates a new call builder for the [`isPermittedCaller`] function.
        pub fn isPermittedCaller(
            &self,
            serviceId: u64,
            caller: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, isPermittedCallerCall, N> {
            self.call_builder(
                &isPermittedCallerCall {
                    serviceId,
                    caller,
                },
            )
        }
        ///Creates a new call builder for the [`isServiceActive`] function.
        pub fn isServiceActive(
            &self,
            serviceId: u64,
        ) -> alloy_contract::SolCallBuilder<T, &P, isServiceActiveCall, N> {
            self.call_builder(&isServiceActiveCall { serviceId })
        }
        ///Creates a new call builder for the [`isServiceOperator`] function.
        pub fn isServiceOperator(
            &self,
            serviceId: u64,
            operator: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, isServiceOperatorCall, N> {
            self.call_builder(
                &isServiceOperatorCall {
                    serviceId,
                    operator,
                },
            )
        }
        ///Creates a new call builder for the [`joinService`] function.
        pub fn joinService(
            &self,
            serviceId: u64,
            exposureBps: u16,
        ) -> alloy_contract::SolCallBuilder<T, &P, joinServiceCall, N> {
            self.call_builder(
                &joinServiceCall {
                    serviceId,
                    exposureBps,
                },
            )
        }
        ///Creates a new call builder for the [`leaveService`] function.
        pub fn leaveService(
            &self,
            serviceId: u64,
        ) -> alloy_contract::SolCallBuilder<T, &P, leaveServiceCall, N> {
            self.call_builder(&leaveServiceCall { serviceId })
        }
        ///Creates a new call builder for the [`pause`] function.
        pub fn pause(&self) -> alloy_contract::SolCallBuilder<T, &P, pauseCall, N> {
            self.call_builder(&pauseCall {})
        }
        ///Creates a new call builder for the [`pendingRewards`] function.
        pub fn pendingRewards(
            &self,
            account: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, pendingRewardsCall, N> {
            self.call_builder(&pendingRewardsCall { account })
        }
        ///Creates a new call builder for the [`proposeSlash`] function.
        pub fn proposeSlash(
            &self,
            serviceId: u64,
            operator: alloy::sol_types::private::Address,
            amount: alloy::sol_types::private::primitives::aliases::U256,
            evidence: alloy::sol_types::private::FixedBytes<32>,
        ) -> alloy_contract::SolCallBuilder<T, &P, proposeSlashCall, N> {
            self.call_builder(
                &proposeSlashCall {
                    serviceId,
                    operator,
                    amount,
                    evidence,
                },
            )
        }
        ///Creates a new call builder for the [`registerOperator`] function.
        pub fn registerOperator(
            &self,
            blueprintId: u64,
            preferences: alloy::sol_types::private::Bytes,
        ) -> alloy_contract::SolCallBuilder<T, &P, registerOperatorCall, N> {
            self.call_builder(
                &registerOperatorCall {
                    blueprintId,
                    preferences,
                },
            )
        }
        ///Creates a new call builder for the [`rejectService`] function.
        pub fn rejectService(
            &self,
            requestId: u64,
        ) -> alloy_contract::SolCallBuilder<T, &P, rejectServiceCall, N> {
            self.call_builder(&rejectServiceCall { requestId })
        }
        ///Creates a new call builder for the [`removePermittedCaller`] function.
        pub fn removePermittedCaller(
            &self,
            serviceId: u64,
            caller: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, removePermittedCallerCall, N> {
            self.call_builder(
                &removePermittedCallerCall {
                    serviceId,
                    caller,
                },
            )
        }
        ///Creates a new call builder for the [`requestService`] function.
        pub fn requestService(
            &self,
            blueprintId: u64,
            operators: alloy::sol_types::private::Vec<
                alloy::sol_types::private::Address,
            >,
            config: alloy::sol_types::private::Bytes,
            permittedCallers: alloy::sol_types::private::Vec<
                alloy::sol_types::private::Address,
            >,
            ttl: u64,
            paymentToken: alloy::sol_types::private::Address,
            paymentAmount: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, requestServiceCall, N> {
            self.call_builder(
                &requestServiceCall {
                    blueprintId,
                    operators,
                    config,
                    permittedCallers,
                    ttl,
                    paymentToken,
                    paymentAmount,
                },
            )
        }
        ///Creates a new call builder for the [`requestServiceWithExposure`] function.
        pub fn requestServiceWithExposure(
            &self,
            blueprintId: u64,
            operators: alloy::sol_types::private::Vec<
                alloy::sol_types::private::Address,
            >,
            exposureBps: alloy::sol_types::private::Vec<u16>,
            config: alloy::sol_types::private::Bytes,
            permittedCallers: alloy::sol_types::private::Vec<
                alloy::sol_types::private::Address,
            >,
            ttl: u64,
            paymentToken: alloy::sol_types::private::Address,
            paymentAmount: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, requestServiceWithExposureCall, N> {
            self.call_builder(
                &requestServiceWithExposureCall {
                    blueprintId,
                    operators,
                    exposureBps,
                    config,
                    permittedCallers,
                    ttl,
                    paymentToken,
                    paymentAmount,
                },
            )
        }
        ///Creates a new call builder for the [`requestServiceWithSecurity`] function.
        pub fn requestServiceWithSecurity(
            &self,
            blueprintId: u64,
            operators: alloy::sol_types::private::Vec<
                alloy::sol_types::private::Address,
            >,
            securityRequirements: alloy::sol_types::private::Vec<
                <Types::AssetSecurityRequirement as alloy::sol_types::SolType>::RustType,
            >,
            config: alloy::sol_types::private::Bytes,
            permittedCallers: alloy::sol_types::private::Vec<
                alloy::sol_types::private::Address,
            >,
            ttl: u64,
            paymentToken: alloy::sol_types::private::Address,
            paymentAmount: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, requestServiceWithSecurityCall, N> {
            self.call_builder(
                &requestServiceWithSecurityCall {
                    blueprintId,
                    operators,
                    securityRequirements,
                    config,
                    permittedCallers,
                    ttl,
                    paymentToken,
                    paymentAmount,
                },
            )
        }
        ///Creates a new call builder for the [`serviceCount`] function.
        pub fn serviceCount(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, serviceCountCall, N> {
            self.call_builder(&serviceCountCall {})
        }
        ///Creates a new call builder for the [`setOperatorOnline`] function.
        pub fn setOperatorOnline(
            &self,
            blueprintId: u64,
            online: bool,
        ) -> alloy_contract::SolCallBuilder<T, &P, setOperatorOnlineCall, N> {
            self.call_builder(
                &setOperatorOnlineCall {
                    blueprintId,
                    online,
                },
            )
        }
        ///Creates a new call builder for the [`setPaymentSplit`] function.
        pub fn setPaymentSplit(
            &self,
            split: <Types::PaymentSplit as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<T, &P, setPaymentSplitCall, N> {
            self.call_builder(&setPaymentSplitCall { split })
        }
        ///Creates a new call builder for the [`setRestaking`] function.
        pub fn setRestaking(
            &self,
            restaking: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, setRestakingCall, N> {
            self.call_builder(&setRestakingCall { restaking })
        }
        ///Creates a new call builder for the [`setTreasury`] function.
        pub fn setTreasury(
            &self,
            treasury: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, setTreasuryCall, N> {
            self.call_builder(&setTreasuryCall { treasury })
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
        ///Creates a new call builder for the [`terminateService`] function.
        pub fn terminateService(
            &self,
            serviceId: u64,
        ) -> alloy_contract::SolCallBuilder<T, &P, terminateServiceCall, N> {
            self.call_builder(&terminateServiceCall { serviceId })
        }
        ///Creates a new call builder for the [`transferBlueprint`] function.
        pub fn transferBlueprint(
            &self,
            blueprintId: u64,
            newOwner: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, transferBlueprintCall, N> {
            self.call_builder(
                &transferBlueprintCall {
                    blueprintId,
                    newOwner,
                },
            )
        }
        ///Creates a new call builder for the [`unpause`] function.
        pub fn unpause(&self) -> alloy_contract::SolCallBuilder<T, &P, unpauseCall, N> {
            self.call_builder(&unpauseCall {})
        }
        ///Creates a new call builder for the [`unregisterOperator`] function.
        pub fn unregisterOperator(
            &self,
            blueprintId: u64,
        ) -> alloy_contract::SolCallBuilder<T, &P, unregisterOperatorCall, N> {
            self.call_builder(
                &unregisterOperatorCall {
                    blueprintId,
                },
            )
        }
        ///Creates a new call builder for the [`updateBlueprint`] function.
        pub fn updateBlueprint(
            &self,
            blueprintId: u64,
            metadataUri: alloy::sol_types::private::String,
        ) -> alloy_contract::SolCallBuilder<T, &P, updateBlueprintCall, N> {
            self.call_builder(
                &updateBlueprintCall {
                    blueprintId,
                    metadataUri,
                },
            )
        }
        ///Creates a new call builder for the [`updateOperatorPreferences`] function.
        pub fn updateOperatorPreferences(
            &self,
            blueprintId: u64,
            preferences: alloy::sol_types::private::Bytes,
        ) -> alloy_contract::SolCallBuilder<T, &P, updateOperatorPreferencesCall, N> {
            self.call_builder(
                &updateOperatorPreferencesCall {
                    blueprintId,
                    preferences,
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
    > ITangleFullInstance<T, P, N> {
        /// Creates a new event filter using this contract instance's provider and address.
        ///
        /// Note that the type can be any event, not just those defined in this contract.
        /// Prefer using the other methods for building type-safe event filters.
        pub fn event_filter<E: alloy_sol_types::SolEvent>(
            &self,
        ) -> alloy_contract::Event<T, &P, E, N> {
            alloy_contract::Event::new_sol(&self.provider, &self.address)
        }
        ///Creates a new event filter for the [`BlueprintCreated`] event.
        pub fn BlueprintCreated_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, BlueprintCreated, N> {
            self.event_filter::<BlueprintCreated>()
        }
        ///Creates a new event filter for the [`BlueprintDeactivated`] event.
        pub fn BlueprintDeactivated_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, BlueprintDeactivated, N> {
            self.event_filter::<BlueprintDeactivated>()
        }
        ///Creates a new event filter for the [`BlueprintTransferred`] event.
        pub fn BlueprintTransferred_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, BlueprintTransferred, N> {
            self.event_filter::<BlueprintTransferred>()
        }
        ///Creates a new event filter for the [`BlueprintUpdated`] event.
        pub fn BlueprintUpdated_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, BlueprintUpdated, N> {
            self.event_filter::<BlueprintUpdated>()
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
        ///Creates a new event filter for the [`OperatorJoinedService`] event.
        pub fn OperatorJoinedService_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, OperatorJoinedService, N> {
            self.event_filter::<OperatorJoinedService>()
        }
        ///Creates a new event filter for the [`OperatorLeftService`] event.
        pub fn OperatorLeftService_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, OperatorLeftService, N> {
            self.event_filter::<OperatorLeftService>()
        }
        ///Creates a new event filter for the [`OperatorOnlineStatusChanged`] event.
        pub fn OperatorOnlineStatusChanged_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, OperatorOnlineStatusChanged, N> {
            self.event_filter::<OperatorOnlineStatusChanged>()
        }
        ///Creates a new event filter for the [`OperatorPreferencesUpdated`] event.
        pub fn OperatorPreferencesUpdated_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, OperatorPreferencesUpdated, N> {
            self.event_filter::<OperatorPreferencesUpdated>()
        }
        ///Creates a new event filter for the [`OperatorRegistered`] event.
        pub fn OperatorRegistered_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, OperatorRegistered, N> {
            self.event_filter::<OperatorRegistered>()
        }
        ///Creates a new event filter for the [`OperatorUnregistered`] event.
        pub fn OperatorUnregistered_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, OperatorUnregistered, N> {
            self.event_filter::<OperatorUnregistered>()
        }
        ///Creates a new event filter for the [`RewardsClaimed`] event.
        pub fn RewardsClaimed_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, RewardsClaimed, N> {
            self.event_filter::<RewardsClaimed>()
        }
        ///Creates a new event filter for the [`RewardsDistributed`] event.
        pub fn RewardsDistributed_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, RewardsDistributed, N> {
            self.event_filter::<RewardsDistributed>()
        }
        ///Creates a new event filter for the [`ServiceActivated`] event.
        pub fn ServiceActivated_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, ServiceActivated, N> {
            self.event_filter::<ServiceActivated>()
        }
        ///Creates a new event filter for the [`ServiceApproved`] event.
        pub fn ServiceApproved_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, ServiceApproved, N> {
            self.event_filter::<ServiceApproved>()
        }
        ///Creates a new event filter for the [`ServiceCreatedFromQuotes`] event.
        pub fn ServiceCreatedFromQuotes_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, ServiceCreatedFromQuotes, N> {
            self.event_filter::<ServiceCreatedFromQuotes>()
        }
        ///Creates a new event filter for the [`ServiceRejected`] event.
        pub fn ServiceRejected_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, ServiceRejected, N> {
            self.event_filter::<ServiceRejected>()
        }
        ///Creates a new event filter for the [`ServiceRequested`] event.
        pub fn ServiceRequested_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, ServiceRequested, N> {
            self.event_filter::<ServiceRequested>()
        }
        ///Creates a new event filter for the [`ServiceRequestedWithSecurity`] event.
        pub fn ServiceRequestedWithSecurity_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, ServiceRequestedWithSecurity, N> {
            self.event_filter::<ServiceRequestedWithSecurity>()
        }
        ///Creates a new event filter for the [`ServiceTerminated`] event.
        pub fn ServiceTerminated_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, ServiceTerminated, N> {
            self.event_filter::<ServiceTerminated>()
        }
        ///Creates a new event filter for the [`SlashExecuted`] event.
        pub fn SlashExecuted_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, SlashExecuted, N> {
            self.event_filter::<SlashExecuted>()
        }
        ///Creates a new event filter for the [`SlashProposed`] event.
        pub fn SlashProposed_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, SlashProposed, N> {
            self.event_filter::<SlashProposed>()
        }
        ///Creates a new event filter for the [`SubscriptionBilled`] event.
        pub fn SubscriptionBilled_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, SubscriptionBilled, N> {
            self.event_filter::<SubscriptionBilled>()
        }
    }
}
