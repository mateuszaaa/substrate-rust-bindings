///Module containing a contract's types and functions.
/**

```solidity
library IRolldownPrimitives {
    type ChainId is uint8;
    type Origin is uint8;
    struct Cancel { RequestId requestId; Range range; bytes32 hash; }
    struct CancelResolution { RequestId requestId; uint256 l2RequestId; bool cancelJustified; uint256 timeStamp; }
    struct Deposit { RequestId requestId; address depositRecipient; address tokenAddress; uint256 amount; uint256 timeStamp; uint256 ferryTip; }
    struct FailedDepositResolution { RequestId requestId; uint256 originRequestId; address ferry; }
    struct L1Update { ChainId chain; Deposit[] pendingDeposits; CancelResolution[] pendingCancelResolutions; }
    struct Range { uint256 start; uint256 end; }
    struct RequestId { Origin origin; uint256 id; }
    struct Withdrawal { RequestId requestId; address recipient; address tokenAddress; uint256 amount; uint256 ferryTip; }
}
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style
)]
pub mod IRolldownPrimitives {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ChainId(u8);
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<ChainId> for u8 {
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
        impl ChainId {
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
        impl alloy_sol_types::SolType for ChainId {
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
        impl alloy_sol_types::EventTopic for ChainId {
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
    pub struct Origin(u8);
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Origin> for u8 {
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
        impl Origin {
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
        impl alloy_sol_types::SolType for Origin {
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
        impl alloy_sol_types::EventTopic for Origin {
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
struct Cancel { RequestId requestId; Range range; bytes32 hash; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct Cancel {
        pub requestId: <RequestId as alloy::sol_types::SolType>::RustType,
        pub range: <Range as alloy::sol_types::SolType>::RustType,
        pub hash: alloy::sol_types::private::FixedBytes<32>,
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
            RequestId,
            Range,
            alloy::sol_types::sol_data::FixedBytes<32>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            <RequestId as alloy::sol_types::SolType>::RustType,
            <Range as alloy::sol_types::SolType>::RustType,
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
        impl ::core::convert::From<Cancel> for UnderlyingRustTuple<'_> {
            fn from(value: Cancel) -> Self {
                (value.requestId, value.range, value.hash)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for Cancel {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    requestId: tuple.0,
                    range: tuple.1,
                    hash: tuple.2,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for Cancel {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for Cancel {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <RequestId as alloy_sol_types::SolType>::tokenize(&self.requestId),
                    <Range as alloy_sol_types::SolType>::tokenize(&self.range),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.hash),
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
        impl alloy_sol_types::SolType for Cancel {
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
        impl alloy_sol_types::SolStruct for Cancel {
            const NAME: &'static str = "Cancel";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "Cancel(RequestId requestId,Range range,bytes32 hash)",
                )
            }
            #[inline]
            fn eip712_components() -> alloy_sol_types::private::Vec<
                alloy_sol_types::private::Cow<'static, str>,
            > {
                let mut components = alloy_sol_types::private::Vec::with_capacity(2);
                components
                    .push(<RequestId as alloy_sol_types::SolStruct>::eip712_root_type());
                components
                    .extend(
                        <RequestId as alloy_sol_types::SolStruct>::eip712_components(),
                    );
                components
                    .push(<Range as alloy_sol_types::SolStruct>::eip712_root_type());
                components
                    .extend(<Range as alloy_sol_types::SolStruct>::eip712_components());
                components
            }
            #[inline]
            fn eip712_encode_data(&self) -> alloy_sol_types::private::Vec<u8> {
                [
                    <RequestId as alloy_sol_types::SolType>::eip712_data_word(
                            &self.requestId,
                        )
                        .0,
                    <Range as alloy_sol_types::SolType>::eip712_data_word(&self.range).0,
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.hash)
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for Cancel {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <RequestId as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.requestId,
                    )
                    + <Range as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.range,
                    )
                    + <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(&rust.hash)
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                out.reserve(
                    <Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust),
                );
                <RequestId as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.requestId,
                    out,
                );
                <Range as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.range,
                    out,
                );
                <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.hash,
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
struct CancelResolution { RequestId requestId; uint256 l2RequestId; bool cancelJustified; uint256 timeStamp; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct CancelResolution {
        pub requestId: <RequestId as alloy::sol_types::SolType>::RustType,
        pub l2RequestId: alloy::sol_types::private::primitives::aliases::U256,
        pub cancelJustified: bool,
        pub timeStamp: alloy::sol_types::private::primitives::aliases::U256,
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
            RequestId,
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Bool,
            alloy::sol_types::sol_data::Uint<256>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            <RequestId as alloy::sol_types::SolType>::RustType,
            alloy::sol_types::private::primitives::aliases::U256,
            bool,
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
        impl ::core::convert::From<CancelResolution> for UnderlyingRustTuple<'_> {
            fn from(value: CancelResolution) -> Self {
                (
                    value.requestId,
                    value.l2RequestId,
                    value.cancelJustified,
                    value.timeStamp,
                )
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for CancelResolution {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    requestId: tuple.0,
                    l2RequestId: tuple.1,
                    cancelJustified: tuple.2,
                    timeStamp: tuple.3,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for CancelResolution {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for CancelResolution {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <RequestId as alloy_sol_types::SolType>::tokenize(&self.requestId),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.l2RequestId),
                    <alloy::sol_types::sol_data::Bool as alloy_sol_types::SolType>::tokenize(
                        &self.cancelJustified,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.timeStamp),
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
        impl alloy_sol_types::SolType for CancelResolution {
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
        impl alloy_sol_types::SolStruct for CancelResolution {
            const NAME: &'static str = "CancelResolution";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "CancelResolution(RequestId requestId,uint256 l2RequestId,bool cancelJustified,uint256 timeStamp)",
                )
            }
            #[inline]
            fn eip712_components() -> alloy_sol_types::private::Vec<
                alloy_sol_types::private::Cow<'static, str>,
            > {
                let mut components = alloy_sol_types::private::Vec::with_capacity(1);
                components
                    .push(<RequestId as alloy_sol_types::SolStruct>::eip712_root_type());
                components
                    .extend(
                        <RequestId as alloy_sol_types::SolStruct>::eip712_components(),
                    );
                components
            }
            #[inline]
            fn eip712_encode_data(&self) -> alloy_sol_types::private::Vec<u8> {
                [
                    <RequestId as alloy_sol_types::SolType>::eip712_data_word(
                            &self.requestId,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.l2RequestId)
                        .0,
                    <alloy::sol_types::sol_data::Bool as alloy_sol_types::SolType>::eip712_data_word(
                            &self.cancelJustified,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.timeStamp)
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for CancelResolution {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <RequestId as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.requestId,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.l2RequestId,
                    )
                    + <alloy::sol_types::sol_data::Bool as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.cancelJustified,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.timeStamp,
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
                <RequestId as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.requestId,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.l2RequestId,
                    out,
                );
                <alloy::sol_types::sol_data::Bool as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.cancelJustified,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.timeStamp,
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
struct Deposit { RequestId requestId; address depositRecipient; address tokenAddress; uint256 amount; uint256 timeStamp; uint256 ferryTip; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct Deposit {
        pub requestId: <RequestId as alloy::sol_types::SolType>::RustType,
        pub depositRecipient: alloy::sol_types::private::Address,
        pub tokenAddress: alloy::sol_types::private::Address,
        pub amount: alloy::sol_types::private::primitives::aliases::U256,
        pub timeStamp: alloy::sol_types::private::primitives::aliases::U256,
        pub ferryTip: alloy::sol_types::private::primitives::aliases::U256,
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
            RequestId,
            alloy::sol_types::sol_data::Address,
            alloy::sol_types::sol_data::Address,
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Uint<256>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            <RequestId as alloy::sol_types::SolType>::RustType,
            alloy::sol_types::private::Address,
            alloy::sol_types::private::Address,
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
        impl ::core::convert::From<Deposit> for UnderlyingRustTuple<'_> {
            fn from(value: Deposit) -> Self {
                (
                    value.requestId,
                    value.depositRecipient,
                    value.tokenAddress,
                    value.amount,
                    value.timeStamp,
                    value.ferryTip,
                )
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for Deposit {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    requestId: tuple.0,
                    depositRecipient: tuple.1,
                    tokenAddress: tuple.2,
                    amount: tuple.3,
                    timeStamp: tuple.4,
                    ferryTip: tuple.5,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for Deposit {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for Deposit {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <RequestId as alloy_sol_types::SolType>::tokenize(&self.requestId),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.depositRecipient,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.tokenAddress,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.amount),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.timeStamp),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.ferryTip),
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
        impl alloy_sol_types::SolType for Deposit {
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
        impl alloy_sol_types::SolStruct for Deposit {
            const NAME: &'static str = "Deposit";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "Deposit(RequestId requestId,address depositRecipient,address tokenAddress,uint256 amount,uint256 timeStamp,uint256 ferryTip)",
                )
            }
            #[inline]
            fn eip712_components() -> alloy_sol_types::private::Vec<
                alloy_sol_types::private::Cow<'static, str>,
            > {
                let mut components = alloy_sol_types::private::Vec::with_capacity(1);
                components
                    .push(<RequestId as alloy_sol_types::SolStruct>::eip712_root_type());
                components
                    .extend(
                        <RequestId as alloy_sol_types::SolStruct>::eip712_components(),
                    );
                components
            }
            #[inline]
            fn eip712_encode_data(&self) -> alloy_sol_types::private::Vec<u8> {
                [
                    <RequestId as alloy_sol_types::SolType>::eip712_data_word(
                            &self.requestId,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.depositRecipient,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.tokenAddress,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.amount)
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.timeStamp)
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.ferryTip)
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for Deposit {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <RequestId as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.requestId,
                    )
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.depositRecipient,
                    )
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.tokenAddress,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.amount,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.timeStamp,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.ferryTip,
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
                <RequestId as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.requestId,
                    out,
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.depositRecipient,
                    out,
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.tokenAddress,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.amount,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.timeStamp,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.ferryTip,
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
struct FailedDepositResolution { RequestId requestId; uint256 originRequestId; address ferry; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct FailedDepositResolution {
        pub requestId: <RequestId as alloy::sol_types::SolType>::RustType,
        pub originRequestId: alloy::sol_types::private::primitives::aliases::U256,
        pub ferry: alloy::sol_types::private::Address,
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
            RequestId,
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Address,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            <RequestId as alloy::sol_types::SolType>::RustType,
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
        impl ::core::convert::From<FailedDepositResolution> for UnderlyingRustTuple<'_> {
            fn from(value: FailedDepositResolution) -> Self {
                (value.requestId, value.originRequestId, value.ferry)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for FailedDepositResolution {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    requestId: tuple.0,
                    originRequestId: tuple.1,
                    ferry: tuple.2,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for FailedDepositResolution {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for FailedDepositResolution {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <RequestId as alloy_sol_types::SolType>::tokenize(&self.requestId),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.originRequestId),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.ferry,
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
        impl alloy_sol_types::SolType for FailedDepositResolution {
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
        impl alloy_sol_types::SolStruct for FailedDepositResolution {
            const NAME: &'static str = "FailedDepositResolution";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "FailedDepositResolution(RequestId requestId,uint256 originRequestId,address ferry)",
                )
            }
            #[inline]
            fn eip712_components() -> alloy_sol_types::private::Vec<
                alloy_sol_types::private::Cow<'static, str>,
            > {
                let mut components = alloy_sol_types::private::Vec::with_capacity(1);
                components
                    .push(<RequestId as alloy_sol_types::SolStruct>::eip712_root_type());
                components
                    .extend(
                        <RequestId as alloy_sol_types::SolStruct>::eip712_components(),
                    );
                components
            }
            #[inline]
            fn eip712_encode_data(&self) -> alloy_sol_types::private::Vec<u8> {
                [
                    <RequestId as alloy_sol_types::SolType>::eip712_data_word(
                            &self.requestId,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.originRequestId,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.ferry,
                        )
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for FailedDepositResolution {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <RequestId as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.requestId,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.originRequestId,
                    )
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.ferry,
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
                <RequestId as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.requestId,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.originRequestId,
                    out,
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.ferry,
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
struct L1Update { ChainId chain; Deposit[] pendingDeposits; CancelResolution[] pendingCancelResolutions; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct L1Update {
        pub chain: <ChainId as alloy::sol_types::SolType>::RustType,
        pub pendingDeposits: alloy::sol_types::private::Vec<
            <Deposit as alloy::sol_types::SolType>::RustType,
        >,
        pub pendingCancelResolutions: alloy::sol_types::private::Vec<
            <CancelResolution as alloy::sol_types::SolType>::RustType,
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
            ChainId,
            alloy::sol_types::sol_data::Array<Deposit>,
            alloy::sol_types::sol_data::Array<CancelResolution>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            <ChainId as alloy::sol_types::SolType>::RustType,
            alloy::sol_types::private::Vec<
                <Deposit as alloy::sol_types::SolType>::RustType,
            >,
            alloy::sol_types::private::Vec<
                <CancelResolution as alloy::sol_types::SolType>::RustType,
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
        impl ::core::convert::From<L1Update> for UnderlyingRustTuple<'_> {
            fn from(value: L1Update) -> Self {
                (value.chain, value.pendingDeposits, value.pendingCancelResolutions)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for L1Update {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    chain: tuple.0,
                    pendingDeposits: tuple.1,
                    pendingCancelResolutions: tuple.2,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for L1Update {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for L1Update {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <ChainId as alloy_sol_types::SolType>::tokenize(&self.chain),
                    <alloy::sol_types::sol_data::Array<
                        Deposit,
                    > as alloy_sol_types::SolType>::tokenize(&self.pendingDeposits),
                    <alloy::sol_types::sol_data::Array<
                        CancelResolution,
                    > as alloy_sol_types::SolType>::tokenize(
                        &self.pendingCancelResolutions,
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
        impl alloy_sol_types::SolType for L1Update {
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
        impl alloy_sol_types::SolStruct for L1Update {
            const NAME: &'static str = "L1Update";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "L1Update(uint8 chain,Deposit[] pendingDeposits,CancelResolution[] pendingCancelResolutions)",
                )
            }
            #[inline]
            fn eip712_components() -> alloy_sol_types::private::Vec<
                alloy_sol_types::private::Cow<'static, str>,
            > {
                let mut components = alloy_sol_types::private::Vec::with_capacity(2);
                components
                    .push(<Deposit as alloy_sol_types::SolStruct>::eip712_root_type());
                components
                    .extend(
                        <Deposit as alloy_sol_types::SolStruct>::eip712_components(),
                    );
                components
                    .push(
                        <CancelResolution as alloy_sol_types::SolStruct>::eip712_root_type(),
                    );
                components
                    .extend(
                        <CancelResolution as alloy_sol_types::SolStruct>::eip712_components(),
                    );
                components
            }
            #[inline]
            fn eip712_encode_data(&self) -> alloy_sol_types::private::Vec<u8> {
                [
                    <ChainId as alloy_sol_types::SolType>::eip712_data_word(&self.chain)
                        .0,
                    <alloy::sol_types::sol_data::Array<
                        Deposit,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.pendingDeposits,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Array<
                        CancelResolution,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.pendingCancelResolutions,
                        )
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for L1Update {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <ChainId as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.chain,
                    )
                    + <alloy::sol_types::sol_data::Array<
                        Deposit,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.pendingDeposits,
                    )
                    + <alloy::sol_types::sol_data::Array<
                        CancelResolution,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.pendingCancelResolutions,
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
                <ChainId as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.chain,
                    out,
                );
                <alloy::sol_types::sol_data::Array<
                    Deposit,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.pendingDeposits,
                    out,
                );
                <alloy::sol_types::sol_data::Array<
                    CancelResolution,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.pendingCancelResolutions,
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
struct Range { uint256 start; uint256 end; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct Range {
        pub start: alloy::sol_types::private::primitives::aliases::U256,
        pub end: alloy::sol_types::private::primitives::aliases::U256,
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
        impl ::core::convert::From<Range> for UnderlyingRustTuple<'_> {
            fn from(value: Range) -> Self {
                (value.start, value.end)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for Range {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    start: tuple.0,
                    end: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for Range {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for Range {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.start),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.end),
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
        impl alloy_sol_types::SolType for Range {
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
        impl alloy_sol_types::SolStruct for Range {
            const NAME: &'static str = "Range";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "Range(uint256 start,uint256 end)",
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
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.start)
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.end)
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for Range {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(&rust.start)
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(&rust.end)
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
                    &rust.start,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(&rust.end, out);
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
struct RequestId { Origin origin; uint256 id; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct RequestId {
        pub origin: <Origin as alloy::sol_types::SolType>::RustType,
        pub id: alloy::sol_types::private::primitives::aliases::U256,
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
        type UnderlyingSolTuple<'a> = (Origin, alloy::sol_types::sol_data::Uint<256>);
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            <Origin as alloy::sol_types::SolType>::RustType,
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
        impl ::core::convert::From<RequestId> for UnderlyingRustTuple<'_> {
            fn from(value: RequestId) -> Self {
                (value.origin, value.id)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for RequestId {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    origin: tuple.0,
                    id: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for RequestId {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for RequestId {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <Origin as alloy_sol_types::SolType>::tokenize(&self.origin),
                    <alloy::sol_types::sol_data::Uint<
                        256,
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
        impl alloy_sol_types::SolType for RequestId {
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
        impl alloy_sol_types::SolStruct for RequestId {
            const NAME: &'static str = "RequestId";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "RequestId(uint8 origin,uint256 id)",
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
                    <Origin as alloy_sol_types::SolType>::eip712_data_word(&self.origin)
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.id)
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for RequestId {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <Origin as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.origin,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        256,
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
                <Origin as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.origin,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
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
    /**```solidity
struct Withdrawal { RequestId requestId; address recipient; address tokenAddress; uint256 amount; uint256 ferryTip; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct Withdrawal {
        pub requestId: <RequestId as alloy::sol_types::SolType>::RustType,
        pub recipient: alloy::sol_types::private::Address,
        pub tokenAddress: alloy::sol_types::private::Address,
        pub amount: alloy::sol_types::private::primitives::aliases::U256,
        pub ferryTip: alloy::sol_types::private::primitives::aliases::U256,
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
            RequestId,
            alloy::sol_types::sol_data::Address,
            alloy::sol_types::sol_data::Address,
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Uint<256>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            <RequestId as alloy::sol_types::SolType>::RustType,
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
        impl ::core::convert::From<Withdrawal> for UnderlyingRustTuple<'_> {
            fn from(value: Withdrawal) -> Self {
                (
                    value.requestId,
                    value.recipient,
                    value.tokenAddress,
                    value.amount,
                    value.ferryTip,
                )
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for Withdrawal {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    requestId: tuple.0,
                    recipient: tuple.1,
                    tokenAddress: tuple.2,
                    amount: tuple.3,
                    ferryTip: tuple.4,
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
                    <RequestId as alloy_sol_types::SolType>::tokenize(&self.requestId),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.recipient,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.tokenAddress,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.amount),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.ferryTip),
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
                    "Withdrawal(RequestId requestId,address recipient,address tokenAddress,uint256 amount,uint256 ferryTip)",
                )
            }
            #[inline]
            fn eip712_components() -> alloy_sol_types::private::Vec<
                alloy_sol_types::private::Cow<'static, str>,
            > {
                let mut components = alloy_sol_types::private::Vec::with_capacity(1);
                components
                    .push(<RequestId as alloy_sol_types::SolStruct>::eip712_root_type());
                components
                    .extend(
                        <RequestId as alloy_sol_types::SolStruct>::eip712_components(),
                    );
                components
            }
            #[inline]
            fn eip712_encode_data(&self) -> alloy_sol_types::private::Vec<u8> {
                [
                    <RequestId as alloy_sol_types::SolType>::eip712_data_word(
                            &self.requestId,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.recipient,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.tokenAddress,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.amount)
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.ferryTip)
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
                    + <RequestId as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.requestId,
                    )
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.recipient,
                    )
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.tokenAddress,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.amount,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.ferryTip,
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
                <RequestId as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.requestId,
                    out,
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.recipient,
                    out,
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.tokenAddress,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.amount,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.ferryTip,
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
    /**Creates a new wrapper around an on-chain [`IRolldownPrimitives`](self) contract instance.

See the [wrapper's documentation](`IRolldownPrimitivesInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> IRolldownPrimitivesInstance<T, P, N> {
        IRolldownPrimitivesInstance::<T, P, N>::new(address, provider)
    }
    /**A [`IRolldownPrimitives`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`IRolldownPrimitives`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct IRolldownPrimitivesInstance<T, P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for IRolldownPrimitivesInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("IRolldownPrimitivesInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > IRolldownPrimitivesInstance<T, P, N> {
        /**Creates a new wrapper around an on-chain [`IRolldownPrimitives`](self) contract instance.

See the [wrapper's documentation](`IRolldownPrimitivesInstance`) for more details.*/
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
    impl<T, P: ::core::clone::Clone, N> IRolldownPrimitivesInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> IRolldownPrimitivesInstance<T, P, N> {
            IRolldownPrimitivesInstance {
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
    > IRolldownPrimitivesInstance<T, P, N> {
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
    > IRolldownPrimitivesInstance<T, P, N> {
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
library IRolldownPrimitives {
    type ChainId is uint8;
    type Origin is uint8;
    struct Cancel {
        RequestId requestId;
        Range range;
        bytes32 hash;
    }
    struct CancelResolution {
        RequestId requestId;
        uint256 l2RequestId;
        bool cancelJustified;
        uint256 timeStamp;
    }
    struct Deposit {
        RequestId requestId;
        address depositRecipient;
        address tokenAddress;
        uint256 amount;
        uint256 timeStamp;
        uint256 ferryTip;
    }
    struct FailedDepositResolution {
        RequestId requestId;
        uint256 originRequestId;
        address ferry;
    }
    struct L1Update {
        ChainId chain;
        Deposit[] pendingDeposits;
        CancelResolution[] pendingCancelResolutions;
    }
    struct Range {
        uint256 start;
        uint256 end;
    }
    struct RequestId {
        Origin origin;
        uint256 id;
    }
    struct Withdrawal {
        RequestId requestId;
        address recipient;
        address tokenAddress;
        uint256 amount;
        uint256 ferryTip;
    }
}

interface Rolldown {
    event DepositAcceptedIntoQueue(uint256 requestId, address depositRecipient, address tokenAddress, uint256 amount, uint256 ferryTip);
    event DisputeResolutionAcceptedIntoQueue(uint256 requestId, bool cancelJustified, bytes32 cancelResolutionHash);
    event ERC20TokensWithdrawn(address sender, address token_address, uint256 amount);
    event FailedDepositResolutionClosed(uint256 requestId, uint256 originDepositId, bytes32 failedDespotiResolutionHash);
    event FerriedWithdrawalClosed(uint256 requestId, bytes32 withdrawalHash);
    event Initialized(uint8 version);
    event L2UpdateAccepted(bytes32 root, IRolldownPrimitives.Range range);
    event NativeTokensWithdrawn(address sender, uint256 amount);
    event NewUpdaterSet(address updater);
    event OwnershipTransferred(address indexed previousOwner, address indexed newOwner);
    event Paused(address indexed account, uint256 newPausedStatus);
    event PauserRegistrySet(address pauserRegistry, address newPauserRegistry);
    event Unpaused(address indexed account, uint256 newPausedStatus);
    event WithdrawalClosed(uint256 requestId, bytes32 withdrawalHash);
    event WithdrawalFerried(uint256 requestId, uint256 amount, address recipient, address ferry, bytes32 withdrawalHash);

    function CLOSED() external view returns (address);
    function NATIVE_TOKEN_ADDRESS() external view returns (address);
    function calculate_root(bytes32 leave_hash, uint32 leave_idx, bytes32[] memory proof, uint32 leaves_count) external pure returns (bytes32);
    function calculate_root_impl(uint32 level, uint32 pos, bytes32 hash, bytes32[] memory proofs, uint32 proof_idx, uint32 max_index) external pure returns (bytes32);
    function cancelResolutions(uint256) external view returns (IRolldownPrimitives.RequestId memory requestId, uint256 l2RequestId, bool cancelJustified, uint256 timeStamp);
    function chain() external view returns (IRolldownPrimitives.ChainId);
    function close_cancel(IRolldownPrimitives.Cancel memory cancel, bytes32 merkle_root, bytes32[] memory proof) external;
    function close_deposit_refund(IRolldownPrimitives.FailedDepositResolution memory failedDeposit, bytes32 merkle_root, bytes32[] memory proof) external;
    function close_withdrawal(IRolldownPrimitives.Withdrawal memory withdrawal, bytes32 merkle_root, bytes32[] memory proof) external;
    function counter() external view returns (uint256);
    function deposit(address tokenAddress, uint256 amount, uint256 ferryTip) external;
    function deposit(address tokenAddress, uint256 amount) external;
    function deposit_erc20(address tokenAddress, uint256 amount, uint256 ferryTip) external;
    function deposit_erc20(address tokenAddress, uint256 amount) external;
    function deposit_native() external payable;
    function deposit_native(uint256 ferryTip) external payable;
    function deposits(uint256) external view returns (IRolldownPrimitives.RequestId memory requestId, address depositRecipient, address tokenAddress, uint256 amount, uint256 timeStamp, uint256 ferryTip);
    function ferry_withdrawal(IRolldownPrimitives.Withdrawal memory withdrawal) external payable;
    function find_l2_batch(uint256 requestId) external view returns (bytes32);
    function getMerkleRootsLength() external view returns (uint256);
    function getPendingRequests(uint256 start, uint256 end) external view returns (IRolldownPrimitives.L1Update memory);
    function getUpdateForL2() external view returns (IRolldownPrimitives.L1Update memory);
    function hashCancel(IRolldownPrimitives.Cancel memory cancel) external pure returns (bytes32);
    function hashFailedDepositResolution(IRolldownPrimitives.FailedDepositResolution memory failedDeposit) external pure returns (bytes32);
    function hashWithdrawal(IRolldownPrimitives.Withdrawal memory withdrawal) external pure returns (bytes32);
    function initialize(address _pauserRegistry, address initialOwner, IRolldownPrimitives.ChainId chainId, address updater) external;
    function lastProcessedUpdate_origin_l1() external view returns (uint256);
    function lastProcessedUpdate_origin_l2() external view returns (uint256);
    function merkleRootRange(bytes32) external view returns (uint256 start, uint256 end);
    function owner() external view returns (address);
    function pause(uint256 newPausedStatus) external;
    function pauseAll() external;
    function paused(uint8 index) external view returns (bool);
    function paused() external view returns (uint256);
    function pauserRegistry() external view returns (address);
    function processedL2Requests(bytes32) external view returns (address);
    function renounceOwnership() external;
    function roots(uint256) external view returns (bytes32);
    function setPauserRegistry(address newPauserRegistry) external;
    function setUpdater(address updater) external;
    function transferOwnership(address newOwner) external;
    function unpause(uint256 newPausedStatus) external;
    function update_l1_from_l2(bytes32 merkle_root, IRolldownPrimitives.Range memory range) external;
    function updaterAccount() external view returns (address);
}
```

...which was generated by the following JSON ABI:
```json
[
  {
    "type": "function",
    "name": "CLOSED",
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
    "name": "NATIVE_TOKEN_ADDRESS",
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
    "name": "calculate_root",
    "inputs": [
      {
        "name": "leave_hash",
        "type": "bytes32",
        "internalType": "bytes32"
      },
      {
        "name": "leave_idx",
        "type": "uint32",
        "internalType": "uint32"
      },
      {
        "name": "proof",
        "type": "bytes32[]",
        "internalType": "bytes32[]"
      },
      {
        "name": "leaves_count",
        "type": "uint32",
        "internalType": "uint32"
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
    "name": "calculate_root_impl",
    "inputs": [
      {
        "name": "level",
        "type": "uint32",
        "internalType": "uint32"
      },
      {
        "name": "pos",
        "type": "uint32",
        "internalType": "uint32"
      },
      {
        "name": "hash",
        "type": "bytes32",
        "internalType": "bytes32"
      },
      {
        "name": "proofs",
        "type": "bytes32[]",
        "internalType": "bytes32[]"
      },
      {
        "name": "proof_idx",
        "type": "uint32",
        "internalType": "uint32"
      },
      {
        "name": "max_index",
        "type": "uint32",
        "internalType": "uint32"
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
    "name": "cancelResolutions",
    "inputs": [
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [
      {
        "name": "requestId",
        "type": "tuple",
        "internalType": "struct IRolldownPrimitives.RequestId",
        "components": [
          {
            "name": "origin",
            "type": "uint8",
            "internalType": "enum IRolldownPrimitives.Origin"
          },
          {
            "name": "id",
            "type": "uint256",
            "internalType": "uint256"
          }
        ]
      },
      {
        "name": "l2RequestId",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "cancelJustified",
        "type": "bool",
        "internalType": "bool"
      },
      {
        "name": "timeStamp",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "chain",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "uint8",
        "internalType": "enum IRolldownPrimitives.ChainId"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "close_cancel",
    "inputs": [
      {
        "name": "cancel",
        "type": "tuple",
        "internalType": "struct IRolldownPrimitives.Cancel",
        "components": [
          {
            "name": "requestId",
            "type": "tuple",
            "internalType": "struct IRolldownPrimitives.RequestId",
            "components": [
              {
                "name": "origin",
                "type": "uint8",
                "internalType": "enum IRolldownPrimitives.Origin"
              },
              {
                "name": "id",
                "type": "uint256",
                "internalType": "uint256"
              }
            ]
          },
          {
            "name": "range",
            "type": "tuple",
            "internalType": "struct IRolldownPrimitives.Range",
            "components": [
              {
                "name": "start",
                "type": "uint256",
                "internalType": "uint256"
              },
              {
                "name": "end",
                "type": "uint256",
                "internalType": "uint256"
              }
            ]
          },
          {
            "name": "hash",
            "type": "bytes32",
            "internalType": "bytes32"
          }
        ]
      },
      {
        "name": "merkle_root",
        "type": "bytes32",
        "internalType": "bytes32"
      },
      {
        "name": "proof",
        "type": "bytes32[]",
        "internalType": "bytes32[]"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "close_deposit_refund",
    "inputs": [
      {
        "name": "failedDeposit",
        "type": "tuple",
        "internalType": "struct IRolldownPrimitives.FailedDepositResolution",
        "components": [
          {
            "name": "requestId",
            "type": "tuple",
            "internalType": "struct IRolldownPrimitives.RequestId",
            "components": [
              {
                "name": "origin",
                "type": "uint8",
                "internalType": "enum IRolldownPrimitives.Origin"
              },
              {
                "name": "id",
                "type": "uint256",
                "internalType": "uint256"
              }
            ]
          },
          {
            "name": "originRequestId",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "ferry",
            "type": "address",
            "internalType": "address"
          }
        ]
      },
      {
        "name": "merkle_root",
        "type": "bytes32",
        "internalType": "bytes32"
      },
      {
        "name": "proof",
        "type": "bytes32[]",
        "internalType": "bytes32[]"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "close_withdrawal",
    "inputs": [
      {
        "name": "withdrawal",
        "type": "tuple",
        "internalType": "struct IRolldownPrimitives.Withdrawal",
        "components": [
          {
            "name": "requestId",
            "type": "tuple",
            "internalType": "struct IRolldownPrimitives.RequestId",
            "components": [
              {
                "name": "origin",
                "type": "uint8",
                "internalType": "enum IRolldownPrimitives.Origin"
              },
              {
                "name": "id",
                "type": "uint256",
                "internalType": "uint256"
              }
            ]
          },
          {
            "name": "recipient",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "tokenAddress",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "amount",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "ferryTip",
            "type": "uint256",
            "internalType": "uint256"
          }
        ]
      },
      {
        "name": "merkle_root",
        "type": "bytes32",
        "internalType": "bytes32"
      },
      {
        "name": "proof",
        "type": "bytes32[]",
        "internalType": "bytes32[]"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "counter",
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
    "name": "deposit",
    "inputs": [
      {
        "name": "tokenAddress",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "amount",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "ferryTip",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "deposit",
    "inputs": [
      {
        "name": "tokenAddress",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "amount",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "deposit_erc20",
    "inputs": [
      {
        "name": "tokenAddress",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "amount",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "ferryTip",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "deposit_erc20",
    "inputs": [
      {
        "name": "tokenAddress",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "amount",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "deposit_native",
    "inputs": [],
    "outputs": [],
    "stateMutability": "payable"
  },
  {
    "type": "function",
    "name": "deposit_native",
    "inputs": [
      {
        "name": "ferryTip",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [],
    "stateMutability": "payable"
  },
  {
    "type": "function",
    "name": "deposits",
    "inputs": [
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [
      {
        "name": "requestId",
        "type": "tuple",
        "internalType": "struct IRolldownPrimitives.RequestId",
        "components": [
          {
            "name": "origin",
            "type": "uint8",
            "internalType": "enum IRolldownPrimitives.Origin"
          },
          {
            "name": "id",
            "type": "uint256",
            "internalType": "uint256"
          }
        ]
      },
      {
        "name": "depositRecipient",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "tokenAddress",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "amount",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "timeStamp",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "ferryTip",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "ferry_withdrawal",
    "inputs": [
      {
        "name": "withdrawal",
        "type": "tuple",
        "internalType": "struct IRolldownPrimitives.Withdrawal",
        "components": [
          {
            "name": "requestId",
            "type": "tuple",
            "internalType": "struct IRolldownPrimitives.RequestId",
            "components": [
              {
                "name": "origin",
                "type": "uint8",
                "internalType": "enum IRolldownPrimitives.Origin"
              },
              {
                "name": "id",
                "type": "uint256",
                "internalType": "uint256"
              }
            ]
          },
          {
            "name": "recipient",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "tokenAddress",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "amount",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "ferryTip",
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
    "name": "find_l2_batch",
    "inputs": [
      {
        "name": "requestId",
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
    "name": "getMerkleRootsLength",
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
    "name": "getPendingRequests",
    "inputs": [
      {
        "name": "start",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "end",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "tuple",
        "internalType": "struct IRolldownPrimitives.L1Update",
        "components": [
          {
            "name": "chain",
            "type": "uint8",
            "internalType": "enum IRolldownPrimitives.ChainId"
          },
          {
            "name": "pendingDeposits",
            "type": "tuple[]",
            "internalType": "struct IRolldownPrimitives.Deposit[]",
            "components": [
              {
                "name": "requestId",
                "type": "tuple",
                "internalType": "struct IRolldownPrimitives.RequestId",
                "components": [
                  {
                    "name": "origin",
                    "type": "uint8",
                    "internalType": "enum IRolldownPrimitives.Origin"
                  },
                  {
                    "name": "id",
                    "type": "uint256",
                    "internalType": "uint256"
                  }
                ]
              },
              {
                "name": "depositRecipient",
                "type": "address",
                "internalType": "address"
              },
              {
                "name": "tokenAddress",
                "type": "address",
                "internalType": "address"
              },
              {
                "name": "amount",
                "type": "uint256",
                "internalType": "uint256"
              },
              {
                "name": "timeStamp",
                "type": "uint256",
                "internalType": "uint256"
              },
              {
                "name": "ferryTip",
                "type": "uint256",
                "internalType": "uint256"
              }
            ]
          },
          {
            "name": "pendingCancelResolutions",
            "type": "tuple[]",
            "internalType": "struct IRolldownPrimitives.CancelResolution[]",
            "components": [
              {
                "name": "requestId",
                "type": "tuple",
                "internalType": "struct IRolldownPrimitives.RequestId",
                "components": [
                  {
                    "name": "origin",
                    "type": "uint8",
                    "internalType": "enum IRolldownPrimitives.Origin"
                  },
                  {
                    "name": "id",
                    "type": "uint256",
                    "internalType": "uint256"
                  }
                ]
              },
              {
                "name": "l2RequestId",
                "type": "uint256",
                "internalType": "uint256"
              },
              {
                "name": "cancelJustified",
                "type": "bool",
                "internalType": "bool"
              },
              {
                "name": "timeStamp",
                "type": "uint256",
                "internalType": "uint256"
              }
            ]
          }
        ]
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getUpdateForL2",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "tuple",
        "internalType": "struct IRolldownPrimitives.L1Update",
        "components": [
          {
            "name": "chain",
            "type": "uint8",
            "internalType": "enum IRolldownPrimitives.ChainId"
          },
          {
            "name": "pendingDeposits",
            "type": "tuple[]",
            "internalType": "struct IRolldownPrimitives.Deposit[]",
            "components": [
              {
                "name": "requestId",
                "type": "tuple",
                "internalType": "struct IRolldownPrimitives.RequestId",
                "components": [
                  {
                    "name": "origin",
                    "type": "uint8",
                    "internalType": "enum IRolldownPrimitives.Origin"
                  },
                  {
                    "name": "id",
                    "type": "uint256",
                    "internalType": "uint256"
                  }
                ]
              },
              {
                "name": "depositRecipient",
                "type": "address",
                "internalType": "address"
              },
              {
                "name": "tokenAddress",
                "type": "address",
                "internalType": "address"
              },
              {
                "name": "amount",
                "type": "uint256",
                "internalType": "uint256"
              },
              {
                "name": "timeStamp",
                "type": "uint256",
                "internalType": "uint256"
              },
              {
                "name": "ferryTip",
                "type": "uint256",
                "internalType": "uint256"
              }
            ]
          },
          {
            "name": "pendingCancelResolutions",
            "type": "tuple[]",
            "internalType": "struct IRolldownPrimitives.CancelResolution[]",
            "components": [
              {
                "name": "requestId",
                "type": "tuple",
                "internalType": "struct IRolldownPrimitives.RequestId",
                "components": [
                  {
                    "name": "origin",
                    "type": "uint8",
                    "internalType": "enum IRolldownPrimitives.Origin"
                  },
                  {
                    "name": "id",
                    "type": "uint256",
                    "internalType": "uint256"
                  }
                ]
              },
              {
                "name": "l2RequestId",
                "type": "uint256",
                "internalType": "uint256"
              },
              {
                "name": "cancelJustified",
                "type": "bool",
                "internalType": "bool"
              },
              {
                "name": "timeStamp",
                "type": "uint256",
                "internalType": "uint256"
              }
            ]
          }
        ]
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "hashCancel",
    "inputs": [
      {
        "name": "cancel",
        "type": "tuple",
        "internalType": "struct IRolldownPrimitives.Cancel",
        "components": [
          {
            "name": "requestId",
            "type": "tuple",
            "internalType": "struct IRolldownPrimitives.RequestId",
            "components": [
              {
                "name": "origin",
                "type": "uint8",
                "internalType": "enum IRolldownPrimitives.Origin"
              },
              {
                "name": "id",
                "type": "uint256",
                "internalType": "uint256"
              }
            ]
          },
          {
            "name": "range",
            "type": "tuple",
            "internalType": "struct IRolldownPrimitives.Range",
            "components": [
              {
                "name": "start",
                "type": "uint256",
                "internalType": "uint256"
              },
              {
                "name": "end",
                "type": "uint256",
                "internalType": "uint256"
              }
            ]
          },
          {
            "name": "hash",
            "type": "bytes32",
            "internalType": "bytes32"
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
    "name": "hashFailedDepositResolution",
    "inputs": [
      {
        "name": "failedDeposit",
        "type": "tuple",
        "internalType": "struct IRolldownPrimitives.FailedDepositResolution",
        "components": [
          {
            "name": "requestId",
            "type": "tuple",
            "internalType": "struct IRolldownPrimitives.RequestId",
            "components": [
              {
                "name": "origin",
                "type": "uint8",
                "internalType": "enum IRolldownPrimitives.Origin"
              },
              {
                "name": "id",
                "type": "uint256",
                "internalType": "uint256"
              }
            ]
          },
          {
            "name": "originRequestId",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "ferry",
            "type": "address",
            "internalType": "address"
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
    "name": "hashWithdrawal",
    "inputs": [
      {
        "name": "withdrawal",
        "type": "tuple",
        "internalType": "struct IRolldownPrimitives.Withdrawal",
        "components": [
          {
            "name": "requestId",
            "type": "tuple",
            "internalType": "struct IRolldownPrimitives.RequestId",
            "components": [
              {
                "name": "origin",
                "type": "uint8",
                "internalType": "enum IRolldownPrimitives.Origin"
              },
              {
                "name": "id",
                "type": "uint256",
                "internalType": "uint256"
              }
            ]
          },
          {
            "name": "recipient",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "tokenAddress",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "amount",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "ferryTip",
            "type": "uint256",
            "internalType": "uint256"
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
    "name": "initialize",
    "inputs": [
      {
        "name": "_pauserRegistry",
        "type": "address",
        "internalType": "contract IPauserRegistry"
      },
      {
        "name": "initialOwner",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "chainId",
        "type": "uint8",
        "internalType": "enum IRolldownPrimitives.ChainId"
      },
      {
        "name": "updater",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "lastProcessedUpdate_origin_l1",
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
    "name": "lastProcessedUpdate_origin_l2",
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
    "name": "merkleRootRange",
    "inputs": [
      {
        "name": "",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "outputs": [
      {
        "name": "start",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "end",
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
    "name": "processedL2Requests",
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
        "type": "address",
        "internalType": "address"
      }
    ],
    "stateMutability": "view"
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
    "name": "roots",
    "inputs": [
      {
        "name": "",
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
    "name": "setPauserRegistry",
    "inputs": [
      {
        "name": "newPauserRegistry",
        "type": "address",
        "internalType": "contract IPauserRegistry"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "setUpdater",
    "inputs": [
      {
        "name": "updater",
        "type": "address",
        "internalType": "address"
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
    "name": "update_l1_from_l2",
    "inputs": [
      {
        "name": "merkle_root",
        "type": "bytes32",
        "internalType": "bytes32"
      },
      {
        "name": "range",
        "type": "tuple",
        "internalType": "struct IRolldownPrimitives.Range",
        "components": [
          {
            "name": "start",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "end",
            "type": "uint256",
            "internalType": "uint256"
          }
        ]
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "updaterAccount",
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
    "name": "DepositAcceptedIntoQueue",
    "inputs": [
      {
        "name": "requestId",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "depositRecipient",
        "type": "address",
        "indexed": false,
        "internalType": "address"
      },
      {
        "name": "tokenAddress",
        "type": "address",
        "indexed": false,
        "internalType": "address"
      },
      {
        "name": "amount",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "ferryTip",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "DisputeResolutionAcceptedIntoQueue",
    "inputs": [
      {
        "name": "requestId",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "cancelJustified",
        "type": "bool",
        "indexed": false,
        "internalType": "bool"
      },
      {
        "name": "cancelResolutionHash",
        "type": "bytes32",
        "indexed": false,
        "internalType": "bytes32"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "ERC20TokensWithdrawn",
    "inputs": [
      {
        "name": "sender",
        "type": "address",
        "indexed": false,
        "internalType": "address"
      },
      {
        "name": "token_address",
        "type": "address",
        "indexed": false,
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
    "name": "FailedDepositResolutionClosed",
    "inputs": [
      {
        "name": "requestId",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "originDepositId",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "failedDespotiResolutionHash",
        "type": "bytes32",
        "indexed": false,
        "internalType": "bytes32"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "FerriedWithdrawalClosed",
    "inputs": [
      {
        "name": "requestId",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "withdrawalHash",
        "type": "bytes32",
        "indexed": false,
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
        "type": "uint8",
        "indexed": false,
        "internalType": "uint8"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "L2UpdateAccepted",
    "inputs": [
      {
        "name": "root",
        "type": "bytes32",
        "indexed": false,
        "internalType": "bytes32"
      },
      {
        "name": "range",
        "type": "tuple",
        "indexed": false,
        "internalType": "struct IRolldownPrimitives.Range",
        "components": [
          {
            "name": "start",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "end",
            "type": "uint256",
            "internalType": "uint256"
          }
        ]
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "NativeTokensWithdrawn",
    "inputs": [
      {
        "name": "sender",
        "type": "address",
        "indexed": false,
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
    "name": "NewUpdaterSet",
    "inputs": [
      {
        "name": "updater",
        "type": "address",
        "indexed": false,
        "internalType": "address"
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
    "name": "PauserRegistrySet",
    "inputs": [
      {
        "name": "pauserRegistry",
        "type": "address",
        "indexed": false,
        "internalType": "contract IPauserRegistry"
      },
      {
        "name": "newPauserRegistry",
        "type": "address",
        "indexed": false,
        "internalType": "contract IPauserRegistry"
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
    "type": "event",
    "name": "WithdrawalClosed",
    "inputs": [
      {
        "name": "requestId",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "withdrawalHash",
        "type": "bytes32",
        "indexed": false,
        "internalType": "bytes32"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "WithdrawalFerried",
    "inputs": [
      {
        "name": "requestId",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "amount",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "recipient",
        "type": "address",
        "indexed": false,
        "internalType": "address"
      },
      {
        "name": "ferry",
        "type": "address",
        "indexed": false,
        "internalType": "address"
      },
      {
        "name": "withdrawalHash",
        "type": "bytes32",
        "indexed": false,
        "internalType": "bytes32"
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
    clippy::style
)]
pub mod Rolldown {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /// The creation / init bytecode of the contract.
    ///
    /// ```text
    ///0x608060405234801561001057600080fd5b50600160d255613fff806100256000396000f3fe6080604052600436106102675760003560e01c8063950ac48711610144578063d16544f0116100b6578063f26ee9d01161007a578063f26ee9d014610738578063f2fde38b1461074e578063f9ecd01e1461076e578063fabc1cbc1461078e578063ff2bae86146107ae578063ffea632b146107c357600080fd5b8063d16544f014610366578063de70e0b8146106ba578063df2ebdbb146106f0578063dffbdd9f14610705578063ef0ba5d01461071857600080fd5b8063b153870611610108578063b153870614610606578063c2b40ae41461061b578063c763e5a11461063b578063c87c222414610662578063ca9b21ae1461066a578063cc8c909f1461069a57600080fd5b8063950ac487146105545780639d54f41914610574578063ae46db1114610594578063af26c695146105b4578063b02c43d0146105d457600080fd5b8063595c6a67116101dd57806371c54461116101a157806371c544611461048e57806379e041f2146104b35780637fd4f845146104e0578063886f1195146104f6578063890e95ce146105165780638da5cb5b1461053657600080fd5b8063595c6a67146103ef5780635ac86ab7146104045780635c975abb1461044457806361bc221a14610463578063715018a61461047957600080fd5b80630efe6a8b1161022f5780630efe6a8b1461028e57806310d67a2f14610326578063136439dd1461034657806347e7ef24146103665780634bf5fec3146103865780634f48eedf146103a657600080fd5b806301ef69661461026c57806308aba1b21461028e57806308f42d40146102ae5780630cac57ab146102ce5780630e2636a3146102e1575b600080fd5b34801561027857600080fd5b5061028c6102873660046134e9565b6107e3565b005b34801561029a57600080fd5b5061028c6102a9366004613559565b610898565b3480156102ba57600080fd5b5061028c6102c936600461358e565b6108f4565b61028c6102dc3660046135d8565b610b47565b3480156102ed57600080fd5b5061030973111111111111111111111111111111111111111181565b6040516001600160a01b0390911681526020015b60405180910390f35b34801561033257600080fd5b5061028c6103413660046135f4565b610f52565b34801561035257600080fd5b5061028c610361366004613611565b611005565b34801561037257600080fd5b5061028c61038136600461362a565b611144565b34801561039257600080fd5b5061028c6103a1366004613656565b6111a0565b3480156103b257600080fd5b506103da6103c1366004613611565b609d602052600090815260409020805460019091015482565b6040805192835260208301919091520161031d565b3480156103fb57600080fd5b5061028c6113f0565b34801561041057600080fd5b5061043461041f36600461369a565b606654600160ff9092169190911b9081161490565b604051901515815260200161031d565b34801561045057600080fd5b506066545b60405190815260200161031d565b34801561046f57600080fd5b5061045560975481565b34801561048557600080fd5b5061028c6114b7565b34801561049a57600080fd5b50609a546103099061010090046001600160a01b031681565b3480156104bf57600080fd5b506104d36104ce3660046136bd565b6114cb565b60405161031d9190613785565b3480156104ec57600080fd5b5061045560985481565b34801561050257600080fd5b50606554610309906001600160a01b031681565b34801561052257600080fd5b506104556105313660046135d8565b6118e5565b34801561054257600080fd5b506033546001600160a01b0316610309565b34801561056057600080fd5b5061028c61056f36600461385a565b611953565b34801561058057600080fd5b5061028c61058f3660046135f4565b6119c2565b3480156105a057600080fd5b506104556105af36600461389d565b611a49565b3480156105c057600080fd5b506104556105cf3660046138d2565b611a7d565b3480156105e057600080fd5b506105f46105ef366004613611565b611ace565b60405161031d96959493929190613941565b34801561061257600080fd5b506104d3611b55565b34801561062757600080fd5b50610455610636366004613611565b611ba0565b34801561064757600080fd5b50609a546106559060ff1681565b60405161031d9190613983565b61028c611bc1565b34801561067657600080fd5b5061068a610685366004613611565b611c19565b60405161031d9493929190613996565b3480156106a657600080fd5b506104556106b53660046139bf565b611c8b565b3480156106c657600080fd5b506103096106d5366004613611565b609e602052600090815260409020546001600160a01b031681565b3480156106fc57600080fd5b50610309600181565b61028c610713366004613611565b611cbf565b34801561072457600080fd5b506104556107333660046139db565b611d17565b34801561074457600080fd5b5061045560995481565b34801561075a57600080fd5b5061028c6107693660046135f4565b611e50565b34801561077a57600080fd5b50610455610789366004613611565b611ec6565b34801561079a57600080fd5b5061028c6107a9366004613611565b612038565b3480156107ba57600080fd5b50609f54610455565b3480156107cf57600080fd5b5061028c6107de366004613a78565b612194565b6066541561080c5760405162461bcd60e51b815260040161080390613ad4565b60405180910390fd5b600260d2540361082e5760405162461bcd60e51b815260040161080390613b0b565b600260d255600061083e85611c8b565b905061085160208601358286868661230d565b61085b85826125a0565b6000908152609e6020526040902080546001600160a01b0319167311111111111111111111111111111111111111111790555050600160d2555050565b606654156108b85760405162461bcd60e51b815260040161080390613ad4565b600260d254036108da5760405162461bcd60e51b815260040161080390613b0b565b600260d2556108ea838383612724565b5050600160d25550565b606654156109145760405162461bcd60e51b815260040161080390613ad4565b609a5461010090046001600160a01b031633146109635760405162461bcd60e51b815260206004820152600d60248201526c2737ba103a34329037bbb732b960991b6044820152606401610803565b6099548160200135116109b85760405162461bcd60e51b815260206004820152601960248201527f557064617465206272696e6773206e6f206e65772064617461000000000000006044820152606401610803565b8035610a065760405162461bcd60e51b815260206004820152601f60248201527f72616e6765206964206d7573742062652067726561746572207468616e2030006044820152606401610803565b609954610a1560018335613b58565b1115610a635760405162461bcd60e51b815260206004820152601760248201527f50726576696f757320757064617465206d697373696e670000000000000000006044820152606401610803565b803560208201351015610aa85760405162461bcd60e51b815260206004820152600d60248201526c496e76616c69642072616e676560981b6044820152606401610803565b609f8054600181019091557f0bc14066c33013fe88f66e314e4cf150b0b2d4d6451a1a51dbbd1c27cd11de28018290556000828152609d602052604090208190610aff828281358155602082013560018201555050565b505060208101356099556040517f49c158d490db9e066f01b5d4f1a094485a6598cb6c5296b4c07e46c12a1dc11c90610b3b9084908490613b6f565b60405180910390a15050565b60665415610b675760405162461bcd60e51b815260040161080390613ad4565b600260d25403610b895760405162461bcd60e51b815260040161080390613b0b565b600260d255608081013560a08201351115610bb65760405162461bcd60e51b815260040161080390613b8d565b6000610bca60a08301356080840135613b58565b90506000610bd7836118e5565b6000818152609e60205260409020549091506001600160a01b031615610c315760405162461bcd60e51b815260206004820152600f60248201526e105b1c9958591e4819995c9c9a5959608a1b6044820152606401610803565b6000818152609e6020526040902080546001600160a01b031916331790556001610c6160808501606086016135f4565b6001600160a01b031603610df65760003411610cb75760405162461bcd60e51b815260206004820152601560248201527413985d1a5d99481d1bdad95b881b9bdd081cd95b9d605a1b6044820152606401610803565b813414610d3d5760405162461bcd60e51b815260206004820152604860248201527f53656e7420616d6f756e742073686f756c642065786163746c79206d6174636860448201527f207769746864726177616c2e616d6f756e74202d207769746864726177616c2e606482015267066657272795469760c41b608482015260a401610803565b610d4d60608401604085016135f4565b6001600160a01b03166108fc839081150290604051600060405180830381858888f19350505050158015610d85573d6000803e3d6000fd5b507f7a9abd9eb86bdbca89cba4069a632c37d93db82e3e14ad8119a3a7812814853e602084013583610dbd60608701604088016135f4565b6040805193845260208401929092526001600160a01b0316908201523360608201526080810183905260a00160405180910390a16108ea565b6000610e0860808501606086016135f4565b6040516370a0823160e01b815233600482015290915083906001600160a01b038316906370a0823190602401602060405180830381865afa158015610e51573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610e759190613bc4565b1015610eb65760405162461bcd60e51b815260206004820152601060248201526f4e6f7420656e6f7567682066756e647360801b6044820152606401610803565b610edc33610eca60608701604088016135f4565b6001600160a01b03841691908661293a565b7f7a9abd9eb86bdbca89cba4069a632c37d93db82e3e14ad8119a3a7812814853e602085013584610f1360608801604089016135f4565b6040805193845260208401929092526001600160a01b0316908201523360608201526080810184905260a00160405180910390a1505050600160d25550565b606560009054906101000a90046001600160a01b03166001600160a01b031663eab66d7a6040518163ffffffff1660e01b8152600401602060405180830381865afa158015610fa5573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610fc99190613bdd565b6001600160a01b0316336001600160a01b031614610ff95760405162461bcd60e51b815260040161080390613bfa565b611002816129ab565b50565b60655460405163237dfb4760e11b81523360048201526001600160a01b03909116906346fbf68e90602401602060405180830381865afa15801561104d573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906110719190613c44565b61108d5760405162461bcd60e51b815260040161080390613c66565b606654818116146111065760405162461bcd60e51b815260206004820152603860248201527f5061757361626c652e70617573653a20696e76616c696420617474656d70742060448201527f746f20756e70617573652066756e6374696f6e616c69747900000000000000006064820152608401610803565b606681905560405181815233907fab40a374bc51de372200a8bc981af8c9ecdc08dfdaef0bb6e09f88f3c616ef3d906020015b60405180910390a250565b606654156111645760405162461bcd60e51b815260040161080390613ad4565b600260d254036111865760405162461bcd60e51b815260040161080390613b0b565b600260d25561119782826000612724565b5050600160d255565b606654156111c05760405162461bcd60e51b815260040161080390613ad4565b600260d254036111e25760405162461bcd60e51b815260040161080390613b0b565b600260d25560006111f2856118e5565b905061120560208601358286868661230d565b6000818152609e6020526040902080546001600160a01b03198116731111111111111111111111111111111111111111179091556001600160a01b03168015158061135457600161125c6080890160608a016135f4565b6001600160a01b0316036112b15761129461127d6060890160408a016135f4565b61128f60a08a013560808b0135613b58565b612aa2565b60a0870135156112ac576112ac338860a00135612aa2565b611313565b6112eb6112c46060890160408a016135f4565b6112d460808a0160608b016135f4565b6112e660a08b013560808c0135613b58565b612b63565b60a087013515611313576113133361130960808a0160608b016135f4565b8960a00135612b63565b60408051602089810135825281018590527f935f26d94be31907080aa78b3e6e2ac6d48a072af096c20268388621bbc11789910160405180910390a16113e2565b60016113666080890160608a016135f4565b6001600160a01b03160361138757611382828860800135612aa2565b6113a5565b6113a58261139b60808a0160608b016135f4565b8960800135612b63565b60408051602089810135825281018590527f2996fd546c37d74c170466ea6aa4a308e3ca2d4aa689e6e9e32994db5039cc0e910160405180910390a15b5050600160d2555050505050565b60655460405163237dfb4760e11b81523360048201526001600160a01b03909116906346fbf68e90602401602060405180830381865afa158015611438573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061145c9190613c44565b6114785760405162461bcd60e51b815260040161080390613c66565b600019606681905560405190815233907fab40a374bc51de372200a8bc981af8c9ecdc08dfdaef0bb6e09f88f3c616ef3d9060200160405180910390a2565b6114bf612ca2565b6114c96000612cfc565b565b6114f06040805160608101909152806000815260200160608152602001606081525090565b6115156040805160608101909152806000815260200160608152602001606081525090565b609a54819060ff16600181111561152e5761152e6136df565b90816001811115611541576115416136df565b90525060008085158015611553575084155b15611563578293505050506118df565b855b858111611608576000818152609c602052604090206001015415611595578261158d81613cae565b9350506115f6565b6000818152609b6020526040902060010154156115be57816115b681613cae565b9250506115f6565b60405162461bcd60e51b815260206004820152600d60248201526c496e76616c69642072616e676560981b6044820152606401610803565b8061160081613cae565b915050611565565b508167ffffffffffffffff81111561162257611622613cc7565b60405190808252806020026020018201604052801561169057816020015b6040805161010081018252600060c0820181815260e0830182905282526020808301829052928201819052606082018190526080820181905260a082015282526000199092019101816116405790505b5060208401528067ffffffffffffffff8111156116af576116af613cc7565b60405190808252806020026020018201604052801561170e57816020015b6040805160c08101825260006080820181815260a0830182905282526020808301829052928201819052606082015282526000199092019101816116cd5790505b506040840152506000905080855b8581116118d8576000818152609c602052604090206001015415611806576000818152609c602052604090819020815161010081019092528054829060c08201908390829060ff166001811115611775576117756136df565b6001811115611786576117866136df565b81526001919091015460209182015290825260028301546001600160a01b03908116838301526003840154166040830152600483015460608301526005830154608083015260069092015460a090910152850151846117e481613cae565b9550815181106117f6576117f6613cdd565b60200260200101819052506118c6565b6000818152609b6020526040902060020154156118c1576000818152609b602052604090819020815160c081019092528054829060808201908390829060ff166001811115611857576118576136df565b6001811115611868576118686136df565b815260019190910154602091820152908252600283015490820152600382015460ff161515604080830191909152600490920154606090910152850151836118af81613cae565b9450815181106117f6576117f6613cdd565b6118d8565b806118d081613cae565b91505061171c565b5091925050505b92915050565b6000806040516020016118f89190613cf3565b604051602081830303815290604052826040516020016119189190613d28565b60408051601f19818403018152908290526119369291602001613db0565b604051602081830303815290604052805190602001209050919050565b606654156119735760405162461bcd60e51b815260040161080390613ad4565b600260d254036119955760405162461bcd60e51b815260040161080390613b0b565b600260d25560006119a585611a49565b90506119b860208601358286868661230d565b61085b8582612d4e565b6119ca612ca2565b606654156119ea5760405162461bcd60e51b815260040161080390613ad4565b609a8054610100600160a81b0319166101006001600160a01b038481168202929092179283905560405192041681527f1b0f2f500df596b42b73e80dbec6a1fb570f01978a586723f988a5fc54d773a19060200160405180910390a150565b60006002604051602001611a5d9190613cf3565b604051602081830303815290604052826040516020016119189190613ddf565b600080825b63ffffffff811615611aad57611a99600282613e33565b9050611aa6600183613e56565b9150611a82565b611ac282888a8989600061073360018c613e75565b98975050505050505050565b609c6020526000908152604090819020815180830190925280549091908290829060ff166001811115611b0357611b036136df565b6001811115611b1457611b146136df565b815260019190910154602090910152600282015460038301546004840154600585015460069095015493946001600160a01b03938416949290931692909186565b611b7a6040805160608101909152806000815260200160608152602001606081525090565b611b9b6098546001611b8c9190613e9a565b60016097546104ce9190613b58565b905090565b609f8181548110611bb057600080fd5b600091825260209091200154905081565b600260d25403611be35760405162461bcd60e51b815260040161080390613b0b565b600260d25560665415611c085760405162461bcd60e51b815260040161080390613ad4565b611c126000612e33565b600160d255565b609b6020526000908152604090819020815180830190925280549091908290829060ff166001811115611c4e57611c4e6136df565b6001811115611c5f57611c5f6136df565b815260019190910154602090910152600282015460038301546004909301549192909160ff9091169084565b60006001604051602001611c9f9190613cf3565b604051602081830303815290604052826040516020016119189190613eb2565b600260d25403611ce15760405162461bcd60e51b815260040161080390613b0b565b600260d25560665415611d065760405162461bcd60e51b815260040161080390613ad4565b611d0f81612e33565b50600160d255565b6000611d24600288613ee8565b63ffffffff16600003611da7578163ffffffff168763ffffffff160315611e055785858585611d5281613f0b565b965063ffffffff16818110611d6957611d69613cdd565b90506020020135604051602001611d8a929190918252602082015260400190565b604051602081830303815290604052805190602001209550611e05565b848484611db381613f0b565b955063ffffffff16818110611dca57611dca613cdd565b9050602002013586604051602001611dec929190918252602082015260400190565b6040516020818303038152906040528051906020012095505b8763ffffffff16600103611e1a575084611e45565b611e42611e2860018a613e75565b611e3360028a613e33565b8888888861073360028a613e33565b90505b979650505050505050565b611e58612ca2565b6001600160a01b038116611ebd5760405162461bcd60e51b815260206004820152602660248201527f4f776e61626c653a206e6577206f776e657220697320746865207a65726f206160448201526564647265737360d01b6064820152608401610803565b61100281612cfc565b6000609954821115611f0f5760405162461bcd60e51b8152602060048201526012602482015271125b9d985b1a59081c995c5d595cdd081a5960721b6044820152606401610803565b609f54600003611f705760405162461bcd60e51b815260206004820152602660248201527f746865726520617265206e6f20726f6f747320796574206f6e2074686520636f6044820152651b9d1c9858dd60d21b6064820152608401610803565b609f54600090611f8290600190613b58565b90505b609d6000609f8381548110611f9c57611f9c613cdd565b90600052602060002001548152602001908152602001600020600001548310158015611ffb5750609d6000609f8381548110611fda57611fda613cdd565b90600052602060002001548152602001908152602001600020600101548311155b1561202657609f818154811061201357612013613cdd565b9060005260206000200154915050919050565b8061203081613f2e565b915050611f85565b606560009054906101000a90046001600160a01b03166001600160a01b031663eab66d7a6040518163ffffffff1660e01b8152600401602060405180830381865afa15801561208b573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906120af9190613bdd565b6001600160a01b0316336001600160a01b0316146120df5760405162461bcd60e51b815260040161080390613bfa565b60665419811960665419161461215d5760405162461bcd60e51b815260206004820152603860248201527f5061757361626c652e756e70617573653a20696e76616c696420617474656d7060448201527f7420746f2070617573652066756e6374696f6e616c69747900000000000000006064820152608401610803565b606681905560405181815233907f3582d1828e26bf56bd801502bc021ac0bc8afb57c826e4986b45593c8fad389c90602001611139565b600054610100900460ff16158080156121b45750600054600160ff909116105b806121ce5750303b1580156121ce575060005460ff166001145b6122315760405162461bcd60e51b815260206004820152602e60248201527f496e697469616c697a61626c653a20636f6e747261637420697320616c72656160448201526d191e481a5b9a5d1a585b1a5e995960921b6064820152608401610803565b6000805460ff191660011790558015612254576000805461ff0019166101001790555b61225f85600061300c565b61226884612cfc565b6000609881905560016097819055609991909155609a8054859260ff1990911690838181111561229a5761229a6136df565b0217905550609a8054610100600160a81b0319166101006001600160a01b038516021790558015612306576000805461ff0019169055604051600181527f7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb3847402498906020015b60405180910390a15b5050505050565b6000838152609d602090815260409182902082518084019093528054808452600190910154918301919091521580159061234a5750602081015115155b61238c5760405162461bcd60e51b8152602060048201526013602482015272155b9adb9bdddb881b595c9adb19481c9bdbdd606a1b6044820152606401610803565b6000858152609e60205260409020546001600160a01b031673111111111111111111111111111111111111111019016123fb5760405162461bcd60e51b8152602060048201526011602482015270105b1c9958591e481c1c9bd8d95cdcd959607a1b6044820152606401610803565b80516020820151101561245b5760405162461bcd60e51b815260206004820152602260248201527f496e76616c696420726571756573742072616e67652c20656e64203c207374616044820152611c9d60f21b6064820152608401610803565b805186108061246d5750806020015186115b156124ba5760405162461bcd60e51b815260206004820152601b60248201527f52657175657374206964206f757473696465206f662072616e676500000000006044820152606401610803565b8051602082015163ffffffff916124d091613b58565b6124db906001613e9a565b11156125195760405162461bcd60e51b815260206004820152600d60248201526c52616e676520746f6f2062696760981b6044820152606401610803565b8051602082015160009161252c91613b58565b612537906001613e9a565b82519091506000906125499089613b58565b9050856125598883888887611a7d565b146125965760405162461bcd60e51b815260206004820152600d60248201526c24b73b30b634b210383937b7b360991b6044820152606401610803565b5050505050505050565b600060016097546125b19190613b58565b606084013511156125c457506001612611565b60006125d8604085013560608601356114cb565b90506000816040516020016125ed9190613785565b60408051601f19818403018152919052805160209091012060808601351415925050505b6040805160c0810190915242906000908060808101808481526020016097600081548092919061264090613cae565b909155509052815260208781013581830152851515604080840191909152606090920185905282518101516000908152609b909152208151805182549394508493839190829060ff19166001838181111561269d5761269d6136df565b02179055506020918201516001919091015582810151600283015560408084015160038401805460ff1916911515919091179055606093840151600490930192909255838101518483015183519182521515918101919091529081018690527f9ef11353afd97d339a77732850b7c22704656558d9ba63cc7e321e0ac4c20aa991016122fd565b818111156127445760405162461bcd60e51b815260040161080390613b8d565b6001600160a01b0383166127925760405162461bcd60e51b8152602060048201526015602482015274496e76616c696420746f6b656e206164647265737360581b6044820152606401610803565b600082116127b25760405162461bcd60e51b815260040161080390613f45565b33836127c96001600160a01b03821683308761293a565b60408051610100810190915242906000908060c0810180848152602001609760008154809291906127f990613cae565b90915550905281526001600160a01b03808716602080840191909152908a16604080840191909152606083018a90526080830186905260a090920188905282518101516000908152609c909152208151805182549394508493839190829060ff19166001838181111561286e5761286e6136df565b021790555060209182015160019190910155828101516002830180546001600160a01b03199081166001600160a01b0393841617909155604080860151600386018054909316908416179091556060808601516004860155608080870151600587015560a096870151600690960195909555865184015182519081528a841694810194909452918c169083015281018990529081018790527f225305ecb66fa9b9b29f8dead9baea365a6c225d639dfd866e782ccb63e2f05b910160405180910390a150505050505050565b6040516001600160a01b03808516602483015283166044820152606481018290526129a59085906323b872dd60e01b906084015b60408051601f198184030181529190526020810180516001600160e01b03166001600160e01b0319909316929092179091526130f2565b50505050565b6001600160a01b038116612a395760405162461bcd60e51b815260206004820152604960248201527f5061757361626c652e5f73657450617573657252656769737472793a206e657760448201527f50617573657252656769737472792063616e6e6f7420626520746865207a65726064820152686f206164647265737360b81b608482015260a401610803565b606554604080516001600160a01b03928316815291831660208301527f6e9fcd539896fca60e8b0f01dd580233e48a6b0f7df013b89ba7f565869acdb6910160405180910390a1606580546001600160a01b0319166001600160a01b0392909216919091179055565b3031811115612af35760405162461bcd60e51b815260206004820152601c60248201527f4e6f7420656e6f7567682066756e647320696e20636f6e7472616374000000006044820152606401610803565b60008111612b135760405162461bcd60e51b815260040161080390613f45565b604080516001600160a01b0384168152602081018390527fe0495355c1e04c512584521854d222d239a4b782b39ac8a7e835a34f5ec7c1e1910160405180910390a1612b5f82826131c9565b5050565b6040516370a0823160e01b8152306004820152829082906001600160a01b038316906370a0823190602401602060405180830381865afa158015612bab573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190612bcf9190613bc4565b1015612c1d5760405162461bcd60e51b815260206004820152601c60248201527f4e6f7420656e6f7567682066756e647320696e20636f6e7472616374000000006044820152606401610803565b60008211612c3d5760405162461bcd60e51b815260040161080390613f45565b612c516001600160a01b03821685846132e2565b604080516001600160a01b038087168252851660208201529081018390527ee763f7778b8ceef7270c89b7d1df1008b0e482da39c43831417733af96fb0d906060015b60405180910390a150505050565b6033546001600160a01b031633146114c95760405162461bcd60e51b815260206004820181905260248201527f4f776e61626c653a2063616c6c6572206973206e6f7420746865206f776e65726044820152606401610803565b603380546001600160a01b038381166001600160a01b0319831681179093556040519116919082907f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e090600090a35050565b6040808301356000908152609c602052908120600281015490916001600160a01b0390911690612d8460808601606087016135f4565b6001600160a01b031614612da557612da260808501606086016135f4565b90505b60038201546001600160a01b031660001901612dce57612dc9818360040154612aa2565b612def565b60038201546004830154612def9183916001600160a01b0390911690612b63565b60408051602086810135825286830135908201529081018490527f13750c731f87c15242874ce74bf4649502cc8e7c829067ce846505acdb96289d90606001612c94565b34811115612e535760405162461bcd60e51b815260040161080390613b8d565b60003411612ea35760405162461bcd60e51b815260206004820181905260248201527f6d73672076616c7565206d7573742062652067726561746572207468617420306044820152606401610803565b6040805161010081019091523390349042906000908060c081018084815260200160976000815480929190612ed790613cae565b90915550905281526001600160a01b03861660208083019190915260016040808401829052606084018890526080840187905260a090930189905283518201516000908152609c9092529190208251805182549495508594929391928492839160ff1916908381811115612f4d57612f4d6136df565b0217905550602091820151600191820155838201516002840180546001600160a01b03199081166001600160a01b0393841617909155604080870151600387018054909316908416179091556060808701516004870155608080880151600588015560a09788015160069097019690965587518501518251908152928b16948301949094528101919091529081018690529081018790527f225305ecb66fa9b9b29f8dead9baea365a6c225d639dfd866e782ccb63e2f05b91016122fd565b6065546001600160a01b031615801561302d57506001600160a01b03821615155b6130af5760405162461bcd60e51b815260206004820152604760248201527f5061757361626c652e5f696e697469616c697a655061757365723a205f696e6960448201527f7469616c697a6550617573657228292063616e206f6e6c792062652063616c6c6064820152666564206f6e636560c81b608482015260a401610803565b606681905560405181815233907fab40a374bc51de372200a8bc981af8c9ecdc08dfdaef0bb6e09f88f3c616ef3d9060200160405180910390a2612b5f826129ab565b6000613147826040518060400160405280602081526020017f5361666545524332303a206c6f772d6c6576656c2063616c6c206661696c6564815250856001600160a01b03166133129092919063ffffffff16565b8051909150156131c457808060200190518101906131659190613c44565b6131c45760405162461bcd60e51b815260206004820152602a60248201527f5361666545524332303a204552433230206f7065726174696f6e20646964206e6044820152691bdd081cdd58d8d9595960b21b6064820152608401610803565b505050565b804710156132195760405162461bcd60e51b815260206004820152601d60248201527f416464726573733a20696e73756666696369656e742062616c616e63650000006044820152606401610803565b6000826001600160a01b03168260405160006040518083038185875af1925050503d8060008114613266576040519150601f19603f3d011682016040523d82523d6000602084013e61326b565b606091505b50509050806131c45760405162461bcd60e51b815260206004820152603a60248201527f416464726573733a20756e61626c6520746f2073656e642076616c75652c207260448201527f6563697069656e74206d617920686176652072657665727465640000000000006064820152608401610803565b6040516001600160a01b0383166024820152604481018290526131c490849063a9059cbb60e01b9060640161296e565b6060613321848460008561332b565b90505b9392505050565b60608247101561338c5760405162461bcd60e51b815260206004820152602660248201527f416464726573733a20696e73756666696369656e742062616c616e636520666f6044820152651c8818d85b1b60d21b6064820152608401610803565b6001600160a01b0385163b6133e35760405162461bcd60e51b815260206004820152601d60248201527f416464726573733a2063616c6c20746f206e6f6e2d636f6e74726163740000006044820152606401610803565b600080866001600160a01b031685876040516133ff9190613f7a565b60006040518083038185875af1925050503d806000811461343c576040519150601f19603f3d011682016040523d82523d6000602084013e613441565b606091505b5091509150611e458282866060831561345b575081613324565b82511561346b5782518084602001fd5b8160405162461bcd60e51b81526004016108039190613f96565b600060a0828403121561349757600080fd5b50919050565b60008083601f8401126134af57600080fd5b50813567ffffffffffffffff8111156134c757600080fd5b6020830191508360208260051b85010111156134e257600080fd5b9250929050565b60008060008060e085870312156134ff57600080fd5b6135098686613485565b935060a0850135925060c085013567ffffffffffffffff81111561352c57600080fd5b6135388782880161349d565b95989497509550505050565b6001600160a01b038116811461100257600080fd5b60008060006060848603121561356e57600080fd5b833561357981613544565b95602085013595506040909401359392505050565b60008082840360608112156135a257600080fd5b833592506040601f19820112156135b857600080fd5b506020830190509250929050565b600060c0828403121561349757600080fd5b600060c082840312156135ea57600080fd5b61332483836135c6565b60006020828403121561360657600080fd5b813561332481613544565b60006020828403121561362357600080fd5b5035919050565b6000806040838503121561363d57600080fd5b823561364881613544565b946020939093013593505050565b600080600080610100858703121561366d57600080fd5b61367786866135c6565b935060c0850135925060e085013567ffffffffffffffff81111561352c57600080fd5b6000602082840312156136ac57600080fd5b813560ff8116811461332457600080fd5b600080604083850312156136d057600080fd5b50508035926020909101359150565b634e487b7160e01b600052602160045260246000fd5b60028110611002576110026136df565b8051613710816136f5565b8252602090810151910152565b600081518084526020808501945080840160005b8381101561377a578151613746888251613705565b8084015160408981019190915281015115156060808a01919091520151608088015260a09096019590820190600101613731565b509495945050505050565b600060208083526080808401855161379c816136f5565b85840152858301516060604080880182905282519384905260a093928601928489019060005b8181101561381d5785516137d7848251613705565b808a01516001600160a01b03908116858701528582015116868501528581015189850152888101518885015287015160c08401529488019460e0909201916001016137c2565b505089820151898203601f1901848b0152965061383a818861371d565b9a9950505050505050505050565b60006080828403121561349757600080fd5b60008060008060c0858703121561387057600080fd5b61387a8686613848565b93506080850135925060a085013567ffffffffffffffff81111561352c57600080fd5b6000608082840312156138af57600080fd5b6133248383613848565b803563ffffffff811681146138cd57600080fd5b919050565b6000806000806000608086880312156138ea57600080fd5b853594506138fa602087016138b9565b9350604086013567ffffffffffffffff81111561391657600080fd5b6139228882890161349d565b90945092506139359050606087016138b9565b90509295509295909350565b60e0810161394f8289613705565b6001600160a01b039687166040830152949095166060860152608085019290925260a084015260c090920191909152919050565b60208101613990836136f5565b91905290565b60a081016139a48287613705565b60408201949094529115156060830152608090910152919050565b600060a082840312156139d157600080fd5b6133248383613485565b600080600080600080600060c0888a0312156139f657600080fd5b6139ff886138b9565b9650613a0d602089016138b9565b955060408801359450606088013567ffffffffffffffff811115613a3057600080fd5b613a3c8a828b0161349d565b9095509350613a4f9050608089016138b9565b9150613a5d60a089016138b9565b905092959891949750929550565b6002811061100257600080fd5b60008060008060808587031215613a8e57600080fd5b8435613a9981613544565b93506020850135613aa981613544565b92506040850135613ab981613a6b565b91506060850135613ac981613544565b939692955090935050565b6020808252601c908201527f5061757361626c653a20636f6e74726163742069732070617573656400000000604082015260600190565b6020808252601f908201527f5265656e7472616e637947756172643a207265656e7472616e742063616c6c00604082015260600190565b634e487b7160e01b600052601160045260246000fd5b600082821015613b6a57613b6a613b42565b500390565b82815260608101613324602083018480358252602090810135910152565b6020808252601c908201527f5469702065786365656473206465706f736974656420616d6f756e7400000000604082015260600190565b600060208284031215613bd657600080fd5b5051919050565b600060208284031215613bef57600080fd5b815161332481613544565b6020808252602a908201527f6d73672e73656e646572206973206e6f74207065726d697373696f6e6564206160408201526939903ab73830bab9b2b960b11b606082015260800190565b600060208284031215613c5657600080fd5b8151801515811461332457600080fd5b60208082526028908201527f6d73672e73656e646572206973206e6f74207065726d697373696f6e6564206160408201526739903830bab9b2b960c11b606082015260800190565b600060018201613cc057613cc0613b42565b5060010190565b634e487b7160e01b600052604160045260246000fd5b634e487b7160e01b600052603260045260246000fd5b6020810160038310613990576139906136df565b8035613d1281613a6b565b613d1b816136f5565b8252602090810135910152565b60c08101613d368284613d07565b6040830135613d4481613544565b6001600160a01b039081166040840152606084013590613d6382613544565b1660608301526080838101359083015260a092830135929091019190915290565b60005b83811015613d9f578181015183820152602001613d87565b838111156129a55750506000910152565b60008351613dc2818460208801613d84565b835190830190613dd6818360208801613d84565b01949350505050565b60808101613ded8284613d07565b604083013560408301526060830135613e0581613544565b6001600160a01b031660609290920191909152919050565b634e487b7160e01b600052601260045260246000fd5b600063ffffffff80841680613e4a57613e4a613e1d565b92169190910492915050565b600063ffffffff808316818516808303821115613dd657613dd6613b42565b600063ffffffff83811690831681811015613e9257613e92613b42565b039392505050565b60008219821115613ead57613ead613b42565b500190565b60a08101613ec08284613d07565b613eda604083016040850180358252602090810135910152565b608092830135919092015290565b600063ffffffff80841680613eff57613eff613e1d565b92169190910692915050565b600063ffffffff808316818103613f2457613f24613b42565b6001019392505050565b600081613f3d57613f3d613b42565b506000190190565b6020808252818101527f416d6f756e74206d7573742062652067726561746572207468616e207a65726f604082015260600190565b60008251613f8c818460208701613d84565b9190910192915050565b6020815260008251806020840152613fb5816040850160208701613d84565b601f01601f1916919091016040019291505056fea26469706673582212206ee6c29b88fb6599f02119957a22d76f2754fff62d2e3a32f4aad5ab9617116664736f6c634300080d0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x01`\xD2Ua?\xFF\x80a\0%`\09`\0\xF3\xFE`\x80`@R`\x046\x10a\x02gW`\x005`\xE0\x1C\x80c\x95\n\xC4\x87\x11a\x01DW\x80c\xD1eD\xF0\x11a\0\xB6W\x80c\xF2n\xE9\xD0\x11a\0zW\x80c\xF2n\xE9\xD0\x14a\x078W\x80c\xF2\xFD\xE3\x8B\x14a\x07NW\x80c\xF9\xEC\xD0\x1E\x14a\x07nW\x80c\xFA\xBC\x1C\xBC\x14a\x07\x8EW\x80c\xFF+\xAE\x86\x14a\x07\xAEW\x80c\xFF\xEAc+\x14a\x07\xC3W`\0\x80\xFD[\x80c\xD1eD\xF0\x14a\x03fW\x80c\xDEp\xE0\xB8\x14a\x06\xBAW\x80c\xDF.\xBD\xBB\x14a\x06\xF0W\x80c\xDF\xFB\xDD\x9F\x14a\x07\x05W\x80c\xEF\x0B\xA5\xD0\x14a\x07\x18W`\0\x80\xFD[\x80c\xB1S\x87\x06\x11a\x01\x08W\x80c\xB1S\x87\x06\x14a\x06\x06W\x80c\xC2\xB4\n\xE4\x14a\x06\x1BW\x80c\xC7c\xE5\xA1\x14a\x06;W\x80c\xC8|\"$\x14a\x06bW\x80c\xCA\x9B!\xAE\x14a\x06jW\x80c\xCC\x8C\x90\x9F\x14a\x06\x9AW`\0\x80\xFD[\x80c\x95\n\xC4\x87\x14a\x05TW\x80c\x9DT\xF4\x19\x14a\x05tW\x80c\xAEF\xDB\x11\x14a\x05\x94W\x80c\xAF&\xC6\x95\x14a\x05\xB4W\x80c\xB0,C\xD0\x14a\x05\xD4W`\0\x80\xFD[\x80cY\\jg\x11a\x01\xDDW\x80cq\xC5Da\x11a\x01\xA1W\x80cq\xC5Da\x14a\x04\x8EW\x80cy\xE0A\xF2\x14a\x04\xB3W\x80c\x7F\xD4\xF8E\x14a\x04\xE0W\x80c\x88o\x11\x95\x14a\x04\xF6W\x80c\x89\x0E\x95\xCE\x14a\x05\x16W\x80c\x8D\xA5\xCB[\x14a\x056W`\0\x80\xFD[\x80cY\\jg\x14a\x03\xEFW\x80cZ\xC8j\xB7\x14a\x04\x04W\x80c\\\x97Z\xBB\x14a\x04DW\x80ca\xBC\"\x1A\x14a\x04cW\x80cqP\x18\xA6\x14a\x04yW`\0\x80\xFD[\x80c\x0E\xFEj\x8B\x11a\x02/W\x80c\x0E\xFEj\x8B\x14a\x02\x8EW\x80c\x10\xD6z/\x14a\x03&W\x80c\x13d9\xDD\x14a\x03FW\x80cG\xE7\xEF$\x14a\x03fW\x80cK\xF5\xFE\xC3\x14a\x03\x86W\x80cOH\xEE\xDF\x14a\x03\xA6W`\0\x80\xFD[\x80c\x01\xEFif\x14a\x02lW\x80c\x08\xAB\xA1\xB2\x14a\x02\x8EW\x80c\x08\xF4-@\x14a\x02\xAEW\x80c\x0C\xACW\xAB\x14a\x02\xCEW\x80c\x0E&6\xA3\x14a\x02\xE1W[`\0\x80\xFD[4\x80\x15a\x02xW`\0\x80\xFD[Pa\x02\x8Ca\x02\x876`\x04a4\xE9V[a\x07\xE3V[\0[4\x80\x15a\x02\x9AW`\0\x80\xFD[Pa\x02\x8Ca\x02\xA96`\x04a5YV[a\x08\x98V[4\x80\x15a\x02\xBAW`\0\x80\xFD[Pa\x02\x8Ca\x02\xC96`\x04a5\x8EV[a\x08\xF4V[a\x02\x8Ca\x02\xDC6`\x04a5\xD8V[a\x0BGV[4\x80\x15a\x02\xEDW`\0\x80\xFD[Pa\x03\ts\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x032W`\0\x80\xFD[Pa\x02\x8Ca\x03A6`\x04a5\xF4V[a\x0FRV[4\x80\x15a\x03RW`\0\x80\xFD[Pa\x02\x8Ca\x03a6`\x04a6\x11V[a\x10\x05V[4\x80\x15a\x03rW`\0\x80\xFD[Pa\x02\x8Ca\x03\x816`\x04a6*V[a\x11DV[4\x80\x15a\x03\x92W`\0\x80\xFD[Pa\x02\x8Ca\x03\xA16`\x04a6VV[a\x11\xA0V[4\x80\x15a\x03\xB2W`\0\x80\xFD[Pa\x03\xDAa\x03\xC16`\x04a6\x11V[`\x9D` R`\0\x90\x81R`@\x90 \x80T`\x01\x90\x91\x01T\x82V[`@\x80Q\x92\x83R` \x83\x01\x91\x90\x91R\x01a\x03\x1DV[4\x80\x15a\x03\xFBW`\0\x80\xFD[Pa\x02\x8Ca\x13\xF0V[4\x80\x15a\x04\x10W`\0\x80\xFD[Pa\x044a\x04\x1F6`\x04a6\x9AV[`fT`\x01`\xFF\x90\x92\x16\x91\x90\x91\x1B\x90\x81\x16\x14\x90V[`@Q\x90\x15\x15\x81R` \x01a\x03\x1DV[4\x80\x15a\x04PW`\0\x80\xFD[P`fT[`@Q\x90\x81R` \x01a\x03\x1DV[4\x80\x15a\x04oW`\0\x80\xFD[Pa\x04U`\x97T\x81V[4\x80\x15a\x04\x85W`\0\x80\xFD[Pa\x02\x8Ca\x14\xB7V[4\x80\x15a\x04\x9AW`\0\x80\xFD[P`\x9ATa\x03\t\x90a\x01\0\x90\x04`\x01`\x01`\xA0\x1B\x03\x16\x81V[4\x80\x15a\x04\xBFW`\0\x80\xFD[Pa\x04\xD3a\x04\xCE6`\x04a6\xBDV[a\x14\xCBV[`@Qa\x03\x1D\x91\x90a7\x85V[4\x80\x15a\x04\xECW`\0\x80\xFD[Pa\x04U`\x98T\x81V[4\x80\x15a\x05\x02W`\0\x80\xFD[P`eTa\x03\t\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[4\x80\x15a\x05\"W`\0\x80\xFD[Pa\x04Ua\x0516`\x04a5\xD8V[a\x18\xE5V[4\x80\x15a\x05BW`\0\x80\xFD[P`3T`\x01`\x01`\xA0\x1B\x03\x16a\x03\tV[4\x80\x15a\x05`W`\0\x80\xFD[Pa\x02\x8Ca\x05o6`\x04a8ZV[a\x19SV[4\x80\x15a\x05\x80W`\0\x80\xFD[Pa\x02\x8Ca\x05\x8F6`\x04a5\xF4V[a\x19\xC2V[4\x80\x15a\x05\xA0W`\0\x80\xFD[Pa\x04Ua\x05\xAF6`\x04a8\x9DV[a\x1AIV[4\x80\x15a\x05\xC0W`\0\x80\xFD[Pa\x04Ua\x05\xCF6`\x04a8\xD2V[a\x1A}V[4\x80\x15a\x05\xE0W`\0\x80\xFD[Pa\x05\xF4a\x05\xEF6`\x04a6\x11V[a\x1A\xCEV[`@Qa\x03\x1D\x96\x95\x94\x93\x92\x91\x90a9AV[4\x80\x15a\x06\x12W`\0\x80\xFD[Pa\x04\xD3a\x1BUV[4\x80\x15a\x06'W`\0\x80\xFD[Pa\x04Ua\x0666`\x04a6\x11V[a\x1B\xA0V[4\x80\x15a\x06GW`\0\x80\xFD[P`\x9ATa\x06U\x90`\xFF\x16\x81V[`@Qa\x03\x1D\x91\x90a9\x83V[a\x02\x8Ca\x1B\xC1V[4\x80\x15a\x06vW`\0\x80\xFD[Pa\x06\x8Aa\x06\x856`\x04a6\x11V[a\x1C\x19V[`@Qa\x03\x1D\x94\x93\x92\x91\x90a9\x96V[4\x80\x15a\x06\xA6W`\0\x80\xFD[Pa\x04Ua\x06\xB56`\x04a9\xBFV[a\x1C\x8BV[4\x80\x15a\x06\xC6W`\0\x80\xFD[Pa\x03\ta\x06\xD56`\x04a6\x11V[`\x9E` R`\0\x90\x81R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x16\x81V[4\x80\x15a\x06\xFCW`\0\x80\xFD[Pa\x03\t`\x01\x81V[a\x02\x8Ca\x07\x136`\x04a6\x11V[a\x1C\xBFV[4\x80\x15a\x07$W`\0\x80\xFD[Pa\x04Ua\x0736`\x04a9\xDBV[a\x1D\x17V[4\x80\x15a\x07DW`\0\x80\xFD[Pa\x04U`\x99T\x81V[4\x80\x15a\x07ZW`\0\x80\xFD[Pa\x02\x8Ca\x07i6`\x04a5\xF4V[a\x1EPV[4\x80\x15a\x07zW`\0\x80\xFD[Pa\x04Ua\x07\x896`\x04a6\x11V[a\x1E\xC6V[4\x80\x15a\x07\x9AW`\0\x80\xFD[Pa\x02\x8Ca\x07\xA96`\x04a6\x11V[a 8V[4\x80\x15a\x07\xBAW`\0\x80\xFD[P`\x9FTa\x04UV[4\x80\x15a\x07\xCFW`\0\x80\xFD[Pa\x02\x8Ca\x07\xDE6`\x04a:xV[a!\x94V[`fT\x15a\x08\x0CW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08\x03\x90a:\xD4V[`@Q\x80\x91\x03\x90\xFD[`\x02`\xD2T\x03a\x08.W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08\x03\x90a;\x0BV[`\x02`\xD2U`\0a\x08>\x85a\x1C\x8BV[\x90Pa\x08Q` \x86\x015\x82\x86\x86\x86a#\rV[a\x08[\x85\x82a%\xA0V[`\0\x90\x81R`\x9E` R`@\x90 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x16s\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x17\x90UPP`\x01`\xD2UPPV[`fT\x15a\x08\xB8W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08\x03\x90a:\xD4V[`\x02`\xD2T\x03a\x08\xDAW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08\x03\x90a;\x0BV[`\x02`\xD2Ua\x08\xEA\x83\x83\x83a'$V[PP`\x01`\xD2UPV[`fT\x15a\t\x14W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08\x03\x90a:\xD4V[`\x9ATa\x01\0\x90\x04`\x01`\x01`\xA0\x1B\x03\x163\x14a\tcW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\r`$\x82\x01Rl'7\xBA\x10:42\x907\xBB\xB72\xB9`\x99\x1B`D\x82\x01R`d\x01a\x08\x03V[`\x99T\x81` \x015\x11a\t\xB8W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x19`$\x82\x01R\x7FUpdate brings no new data\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x08\x03V[\x805a\n\x06W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7Frange id must be greater than 0\0`D\x82\x01R`d\x01a\x08\x03V[`\x99Ta\n\x15`\x01\x835a;XV[\x11\x15a\ncW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x17`$\x82\x01R\x7FPrevious update missing\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x08\x03V[\x805` \x82\x015\x10\x15a\n\xA8W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\r`$\x82\x01RlInvalid range`\x98\x1B`D\x82\x01R`d\x01a\x08\x03V[`\x9F\x80T`\x01\x81\x01\x90\x91U\x7F\x0B\xC1@f\xC30\x13\xFE\x88\xF6n1NL\xF1P\xB0\xB2\xD4\xD6E\x1A\x1AQ\xDB\xBD\x1C'\xCD\x11\xDE(\x01\x82\x90U`\0\x82\x81R`\x9D` R`@\x90 \x81\x90a\n\xFF\x82\x82\x815\x81U` \x82\x015`\x01\x82\x01UPPV[PP` \x81\x015`\x99U`@Q\x7FI\xC1X\xD4\x90\xDB\x9E\x06o\x01\xB5\xD4\xF1\xA0\x94HZe\x98\xCBlR\x96\xB4\xC0~F\xC1*\x1D\xC1\x1C\x90a\x0B;\x90\x84\x90\x84\x90a;oV[`@Q\x80\x91\x03\x90\xA1PPV[`fT\x15a\x0BgW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08\x03\x90a:\xD4V[`\x02`\xD2T\x03a\x0B\x89W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08\x03\x90a;\x0BV[`\x02`\xD2U`\x80\x81\x015`\xA0\x82\x015\x11\x15a\x0B\xB6W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08\x03\x90a;\x8DV[`\0a\x0B\xCA`\xA0\x83\x015`\x80\x84\x015a;XV[\x90P`\0a\x0B\xD7\x83a\x18\xE5V[`\0\x81\x81R`\x9E` R`@\x90 T\x90\x91P`\x01`\x01`\xA0\x1B\x03\x16\x15a\x0C1W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0F`$\x82\x01Rn\x10[\x1C\x99XY\x1EH\x19\x99\\\x9C\x9AYY`\x8A\x1B`D\x82\x01R`d\x01a\x08\x03V[`\0\x81\x81R`\x9E` R`@\x90 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x163\x17\x90U`\x01a\x0Ca`\x80\x85\x01``\x86\x01a5\xF4V[`\x01`\x01`\xA0\x1B\x03\x16\x03a\r\xF6W`\x004\x11a\x0C\xB7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x15`$\x82\x01Rt\x13\x98]\x1A]\x99H\x1D\x1B\xDA\xD9[\x88\x1B\x9B\xDD\x08\x1C\xD9[\x9D`Z\x1B`D\x82\x01R`d\x01a\x08\x03V[\x814\x14a\r=W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`H`$\x82\x01R\x7FSent amount should exactly match`D\x82\x01R\x7F withdrawal.amount - withdrawal.`d\x82\x01Rg\x06fW''\x95F\x97`\xC4\x1B`\x84\x82\x01R`\xA4\x01a\x08\x03V[a\rM``\x84\x01`@\x85\x01a5\xF4V[`\x01`\x01`\xA0\x1B\x03\x16a\x08\xFC\x83\x90\x81\x15\x02\x90`@Q`\0`@Q\x80\x83\x03\x81\x85\x88\x88\xF1\x93PPPP\x15\x80\x15a\r\x85W=`\0\x80>=`\0\xFD[P\x7Fz\x9A\xBD\x9E\xB8k\xDB\xCA\x89\xCB\xA4\x06\x9Ac,7\xD9=\xB8.>\x14\xAD\x81\x19\xA3\xA7\x81(\x14\x85>` \x84\x015\x83a\r\xBD``\x87\x01`@\x88\x01a5\xF4V[`@\x80Q\x93\x84R` \x84\x01\x92\x90\x92R`\x01`\x01`\xA0\x1B\x03\x16\x90\x82\x01R3``\x82\x01R`\x80\x81\x01\x83\x90R`\xA0\x01`@Q\x80\x91\x03\x90\xA1a\x08\xEAV[`\0a\x0E\x08`\x80\x85\x01``\x86\x01a5\xF4V[`@Qcp\xA0\x821`\xE0\x1B\x81R3`\x04\x82\x01R\x90\x91P\x83\x90`\x01`\x01`\xA0\x1B\x03\x83\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0EQW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0Eu\x91\x90a;\xC4V[\x10\x15a\x0E\xB6W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x10`$\x82\x01RoNot enough funds`\x80\x1B`D\x82\x01R`d\x01a\x08\x03V[a\x0E\xDC3a\x0E\xCA``\x87\x01`@\x88\x01a5\xF4V[`\x01`\x01`\xA0\x1B\x03\x84\x16\x91\x90\x86a):V[\x7Fz\x9A\xBD\x9E\xB8k\xDB\xCA\x89\xCB\xA4\x06\x9Ac,7\xD9=\xB8.>\x14\xAD\x81\x19\xA3\xA7\x81(\x14\x85>` \x85\x015\x84a\x0F\x13``\x88\x01`@\x89\x01a5\xF4V[`@\x80Q\x93\x84R` \x84\x01\x92\x90\x92R`\x01`\x01`\xA0\x1B\x03\x16\x90\x82\x01R3``\x82\x01R`\x80\x81\x01\x84\x90R`\xA0\x01`@Q\x80\x91\x03\x90\xA1PPP`\x01`\xD2UPV[`e`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xEA\xB6mz`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0F\xA5W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0F\xC9\x91\x90a;\xDDV[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x0F\xF9W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08\x03\x90a;\xFAV[a\x10\x02\x81a)\xABV[PV[`eT`@Qc#}\xFBG`\xE1\x1B\x81R3`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cF\xFB\xF6\x8E\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x10MW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x10q\x91\x90a<DV[a\x10\x8DW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08\x03\x90a<fV[`fT\x81\x81\x16\x14a\x11\x06W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`8`$\x82\x01R\x7FPausable.pause: invalid attempt `D\x82\x01R\x7Fto unpause functionality\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x08\x03V[`f\x81\x90U`@Q\x81\x81R3\x90\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=\x90` \x01[`@Q\x80\x91\x03\x90\xA2PV[`fT\x15a\x11dW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08\x03\x90a:\xD4V[`\x02`\xD2T\x03a\x11\x86W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08\x03\x90a;\x0BV[`\x02`\xD2Ua\x11\x97\x82\x82`\0a'$V[PP`\x01`\xD2UV[`fT\x15a\x11\xC0W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08\x03\x90a:\xD4V[`\x02`\xD2T\x03a\x11\xE2W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08\x03\x90a;\x0BV[`\x02`\xD2U`\0a\x11\xF2\x85a\x18\xE5V[\x90Pa\x12\x05` \x86\x015\x82\x86\x86\x86a#\rV[`\0\x81\x81R`\x9E` R`@\x90 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x81\x16s\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x17\x90\x91U`\x01`\x01`\xA0\x1B\x03\x16\x80\x15\x15\x80a\x13TW`\x01a\x12\\`\x80\x89\x01``\x8A\x01a5\xF4V[`\x01`\x01`\xA0\x1B\x03\x16\x03a\x12\xB1Wa\x12\x94a\x12}``\x89\x01`@\x8A\x01a5\xF4V[a\x12\x8F`\xA0\x8A\x015`\x80\x8B\x015a;XV[a*\xA2V[`\xA0\x87\x015\x15a\x12\xACWa\x12\xAC3\x88`\xA0\x015a*\xA2V[a\x13\x13V[a\x12\xEBa\x12\xC4``\x89\x01`@\x8A\x01a5\xF4V[a\x12\xD4`\x80\x8A\x01``\x8B\x01a5\xF4V[a\x12\xE6`\xA0\x8B\x015`\x80\x8C\x015a;XV[a+cV[`\xA0\x87\x015\x15a\x13\x13Wa\x13\x133a\x13\t`\x80\x8A\x01``\x8B\x01a5\xF4V[\x89`\xA0\x015a+cV[`@\x80Q` \x89\x81\x015\x82R\x81\x01\x85\x90R\x7F\x93_&\xD9K\xE3\x19\x07\x08\n\xA7\x8B>n*\xC6\xD4\x8A\x07*\xF0\x96\xC2\x02h8\x86!\xBB\xC1\x17\x89\x91\x01`@Q\x80\x91\x03\x90\xA1a\x13\xE2V[`\x01a\x13f`\x80\x89\x01``\x8A\x01a5\xF4V[`\x01`\x01`\xA0\x1B\x03\x16\x03a\x13\x87Wa\x13\x82\x82\x88`\x80\x015a*\xA2V[a\x13\xA5V[a\x13\xA5\x82a\x13\x9B`\x80\x8A\x01``\x8B\x01a5\xF4V[\x89`\x80\x015a+cV[`@\x80Q` \x89\x81\x015\x82R\x81\x01\x85\x90R\x7F)\x96\xFDTl7\xD7L\x17\x04f\xEAj\xA4\xA3\x08\xE3\xCA-J\xA6\x89\xE6\xE9\xE3)\x94\xDBP9\xCC\x0E\x91\x01`@Q\x80\x91\x03\x90\xA1[PP`\x01`\xD2UPPPPPV[`eT`@Qc#}\xFBG`\xE1\x1B\x81R3`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cF\xFB\xF6\x8E\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x148W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x14\\\x91\x90a<DV[a\x14xW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08\x03\x90a<fV[`\0\x19`f\x81\x90U`@Q\x90\x81R3\x90\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=\x90` \x01`@Q\x80\x91\x03\x90\xA2V[a\x14\xBFa,\xA2V[a\x14\xC9`\0a,\xFCV[V[a\x14\xF0`@\x80Q``\x81\x01\x90\x91R\x80`\0\x81R` \x01``\x81R` \x01``\x81RP\x90V[a\x15\x15`@\x80Q``\x81\x01\x90\x91R\x80`\0\x81R` \x01``\x81R` \x01``\x81RP\x90V[`\x9AT\x81\x90`\xFF\x16`\x01\x81\x11\x15a\x15.Wa\x15.a6\xDFV[\x90\x81`\x01\x81\x11\x15a\x15AWa\x15Aa6\xDFV[\x90RP`\0\x80\x85\x15\x80\x15a\x15SWP\x84\x15[\x15a\x15cW\x82\x93PPPPa\x18\xDFV[\x85[\x85\x81\x11a\x16\x08W`\0\x81\x81R`\x9C` R`@\x90 `\x01\x01T\x15a\x15\x95W\x82a\x15\x8D\x81a<\xAEV[\x93PPa\x15\xF6V[`\0\x81\x81R`\x9B` R`@\x90 `\x01\x01T\x15a\x15\xBEW\x81a\x15\xB6\x81a<\xAEV[\x92PPa\x15\xF6V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\r`$\x82\x01RlInvalid range`\x98\x1B`D\x82\x01R`d\x01a\x08\x03V[\x80a\x16\0\x81a<\xAEV[\x91PPa\x15eV[P\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x16\"Wa\x16\"a<\xC7V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x16\x90W\x81` \x01[`@\x80Qa\x01\0\x81\x01\x82R`\0`\xC0\x82\x01\x81\x81R`\xE0\x83\x01\x82\x90R\x82R` \x80\x83\x01\x82\x90R\x92\x82\x01\x81\x90R``\x82\x01\x81\x90R`\x80\x82\x01\x81\x90R`\xA0\x82\x01R\x82R`\0\x19\x90\x92\x01\x91\x01\x81a\x16@W\x90P[P` \x84\x01R\x80g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x16\xAFWa\x16\xAFa<\xC7V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x17\x0EW\x81` \x01[`@\x80Q`\xC0\x81\x01\x82R`\0`\x80\x82\x01\x81\x81R`\xA0\x83\x01\x82\x90R\x82R` \x80\x83\x01\x82\x90R\x92\x82\x01\x81\x90R``\x82\x01R\x82R`\0\x19\x90\x92\x01\x91\x01\x81a\x16\xCDW\x90P[P`@\x84\x01RP`\0\x90P\x80\x85[\x85\x81\x11a\x18\xD8W`\0\x81\x81R`\x9C` R`@\x90 `\x01\x01T\x15a\x18\x06W`\0\x81\x81R`\x9C` R`@\x90\x81\x90 \x81Qa\x01\0\x81\x01\x90\x92R\x80T\x82\x90`\xC0\x82\x01\x90\x83\x90\x82\x90`\xFF\x16`\x01\x81\x11\x15a\x17uWa\x17ua6\xDFV[`\x01\x81\x11\x15a\x17\x86Wa\x17\x86a6\xDFV[\x81R`\x01\x91\x90\x91\x01T` \x91\x82\x01R\x90\x82R`\x02\x83\x01T`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x83\x83\x01R`\x03\x84\x01T\x16`@\x83\x01R`\x04\x83\x01T``\x83\x01R`\x05\x83\x01T`\x80\x83\x01R`\x06\x90\x92\x01T`\xA0\x90\x91\x01R\x85\x01Q\x84a\x17\xE4\x81a<\xAEV[\x95P\x81Q\x81\x10a\x17\xF6Wa\x17\xF6a<\xDDV[` \x02` \x01\x01\x81\x90RPa\x18\xC6V[`\0\x81\x81R`\x9B` R`@\x90 `\x02\x01T\x15a\x18\xC1W`\0\x81\x81R`\x9B` R`@\x90\x81\x90 \x81Q`\xC0\x81\x01\x90\x92R\x80T\x82\x90`\x80\x82\x01\x90\x83\x90\x82\x90`\xFF\x16`\x01\x81\x11\x15a\x18WWa\x18Wa6\xDFV[`\x01\x81\x11\x15a\x18hWa\x18ha6\xDFV[\x81R`\x01\x91\x90\x91\x01T` \x91\x82\x01R\x90\x82R`\x02\x83\x01T\x90\x82\x01R`\x03\x82\x01T`\xFF\x16\x15\x15`@\x80\x83\x01\x91\x90\x91R`\x04\x90\x92\x01T``\x90\x91\x01R\x85\x01Q\x83a\x18\xAF\x81a<\xAEV[\x94P\x81Q\x81\x10a\x17\xF6Wa\x17\xF6a<\xDDV[a\x18\xD8V[\x80a\x18\xD0\x81a<\xAEV[\x91PPa\x17\x1CV[P\x91\x92PPP[\x92\x91PPV[`\0\x80`@Q` \x01a\x18\xF8\x91\x90a<\xF3V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x82`@Q` \x01a\x19\x18\x91\x90a=(V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra\x196\x92\x91` \x01a=\xB0V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x91\x90PV[`fT\x15a\x19sW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08\x03\x90a:\xD4V[`\x02`\xD2T\x03a\x19\x95W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08\x03\x90a;\x0BV[`\x02`\xD2U`\0a\x19\xA5\x85a\x1AIV[\x90Pa\x19\xB8` \x86\x015\x82\x86\x86\x86a#\rV[a\x08[\x85\x82a-NV[a\x19\xCAa,\xA2V[`fT\x15a\x19\xEAW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08\x03\x90a:\xD4V[`\x9A\x80Ta\x01\0`\x01`\xA8\x1B\x03\x19\x16a\x01\0`\x01`\x01`\xA0\x1B\x03\x84\x81\x16\x82\x02\x92\x90\x92\x17\x92\x83\x90U`@Q\x92\x04\x16\x81R\x7F\x1B\x0F/P\r\xF5\x96\xB4+s\xE8\r\xBE\xC6\xA1\xFBW\x0F\x01\x97\x8AXg#\xF9\x88\xA5\xFCT\xD7s\xA1\x90` \x01`@Q\x80\x91\x03\x90\xA1PV[`\0`\x02`@Q` \x01a\x1A]\x91\x90a<\xF3V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x82`@Q` \x01a\x19\x18\x91\x90a=\xDFV[`\0\x80\x82[c\xFF\xFF\xFF\xFF\x81\x16\x15a\x1A\xADWa\x1A\x99`\x02\x82a>3V[\x90Pa\x1A\xA6`\x01\x83a>VV[\x91Pa\x1A\x82V[a\x1A\xC2\x82\x88\x8A\x89\x89`\0a\x073`\x01\x8Ca>uV[\x98\x97PPPPPPPPV[`\x9C` R`\0\x90\x81R`@\x90\x81\x90 \x81Q\x80\x83\x01\x90\x92R\x80T\x90\x91\x90\x82\x90\x82\x90`\xFF\x16`\x01\x81\x11\x15a\x1B\x03Wa\x1B\x03a6\xDFV[`\x01\x81\x11\x15a\x1B\x14Wa\x1B\x14a6\xDFV[\x81R`\x01\x91\x90\x91\x01T` \x90\x91\x01R`\x02\x82\x01T`\x03\x83\x01T`\x04\x84\x01T`\x05\x85\x01T`\x06\x90\x95\x01T\x93\x94`\x01`\x01`\xA0\x1B\x03\x93\x84\x16\x94\x92\x90\x93\x16\x92\x90\x91\x86V[a\x1Bz`@\x80Q``\x81\x01\x90\x91R\x80`\0\x81R` \x01``\x81R` \x01``\x81RP\x90V[a\x1B\x9B`\x98T`\x01a\x1B\x8C\x91\x90a>\x9AV[`\x01`\x97Ta\x04\xCE\x91\x90a;XV[\x90P\x90V[`\x9F\x81\x81T\x81\x10a\x1B\xB0W`\0\x80\xFD[`\0\x91\x82R` \x90\x91 \x01T\x90P\x81V[`\x02`\xD2T\x03a\x1B\xE3W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08\x03\x90a;\x0BV[`\x02`\xD2U`fT\x15a\x1C\x08W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08\x03\x90a:\xD4V[a\x1C\x12`\0a.3V[`\x01`\xD2UV[`\x9B` R`\0\x90\x81R`@\x90\x81\x90 \x81Q\x80\x83\x01\x90\x92R\x80T\x90\x91\x90\x82\x90\x82\x90`\xFF\x16`\x01\x81\x11\x15a\x1CNWa\x1CNa6\xDFV[`\x01\x81\x11\x15a\x1C_Wa\x1C_a6\xDFV[\x81R`\x01\x91\x90\x91\x01T` \x90\x91\x01R`\x02\x82\x01T`\x03\x83\x01T`\x04\x90\x93\x01T\x91\x92\x90\x91`\xFF\x90\x91\x16\x90\x84V[`\0`\x01`@Q` \x01a\x1C\x9F\x91\x90a<\xF3V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x82`@Q` \x01a\x19\x18\x91\x90a>\xB2V[`\x02`\xD2T\x03a\x1C\xE1W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08\x03\x90a;\x0BV[`\x02`\xD2U`fT\x15a\x1D\x06W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08\x03\x90a:\xD4V[a\x1D\x0F\x81a.3V[P`\x01`\xD2UV[`\0a\x1D$`\x02\x88a>\xE8V[c\xFF\xFF\xFF\xFF\x16`\0\x03a\x1D\xA7W\x81c\xFF\xFF\xFF\xFF\x16\x87c\xFF\xFF\xFF\xFF\x16\x03\x15a\x1E\x05W\x85\x85\x85\x85a\x1DR\x81a?\x0BV[\x96Pc\xFF\xFF\xFF\xFF\x16\x81\x81\x10a\x1DiWa\x1Dia<\xDDV[\x90P` \x02\x015`@Q` \x01a\x1D\x8A\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x95Pa\x1E\x05V[\x84\x84\x84a\x1D\xB3\x81a?\x0BV[\x95Pc\xFF\xFF\xFF\xFF\x16\x81\x81\x10a\x1D\xCAWa\x1D\xCAa<\xDDV[\x90P` \x02\x015\x86`@Q` \x01a\x1D\xEC\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x95P[\x87c\xFF\xFF\xFF\xFF\x16`\x01\x03a\x1E\x1AWP\x84a\x1EEV[a\x1EBa\x1E(`\x01\x8Aa>uV[a\x1E3`\x02\x8Aa>3V[\x88\x88\x88\x88a\x073`\x02\x8Aa>3V[\x90P[\x97\x96PPPPPPPV[a\x1EXa,\xA2V[`\x01`\x01`\xA0\x1B\x03\x81\x16a\x1E\xBDW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01Reddress`\xD0\x1B`d\x82\x01R`\x84\x01a\x08\x03V[a\x10\x02\x81a,\xFCV[`\0`\x99T\x82\x11\x15a\x1F\x0FW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x12`$\x82\x01Rq\x12[\x9D\x98[\x1AY\x08\x1C\x99\\]Y\\\xDD\x08\x1AY`r\x1B`D\x82\x01R`d\x01a\x08\x03V[`\x9FT`\0\x03a\x1FpW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7Fthere are no roots yet on the co`D\x82\x01Re\x1B\x9D\x1C\x98X\xDD`\xD2\x1B`d\x82\x01R`\x84\x01a\x08\x03V[`\x9FT`\0\x90a\x1F\x82\x90`\x01\x90a;XV[\x90P[`\x9D`\0`\x9F\x83\x81T\x81\x10a\x1F\x9CWa\x1F\x9Ca<\xDDV[\x90`\0R` `\0 \x01T\x81R` \x01\x90\x81R` \x01`\0 `\0\x01T\x83\x10\x15\x80\x15a\x1F\xFBWP`\x9D`\0`\x9F\x83\x81T\x81\x10a\x1F\xDAWa\x1F\xDAa<\xDDV[\x90`\0R` `\0 \x01T\x81R` \x01\x90\x81R` \x01`\0 `\x01\x01T\x83\x11\x15[\x15a &W`\x9F\x81\x81T\x81\x10a \x13Wa \x13a<\xDDV[\x90`\0R` `\0 \x01T\x91PP\x91\x90PV[\x80a 0\x81a?.V[\x91PPa\x1F\x85V[`e`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xEA\xB6mz`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a \x8BW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a \xAF\x91\x90a;\xDDV[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a \xDFW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08\x03\x90a;\xFAV[`fT\x19\x81\x19`fT\x19\x16\x14a!]W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`8`$\x82\x01R\x7FPausable.unpause: invalid attemp`D\x82\x01R\x7Ft to pause functionality\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x08\x03V[`f\x81\x90U`@Q\x81\x81R3\x90\x7F5\x82\xD1\x82\x8E&\xBFV\xBD\x80\x15\x02\xBC\x02\x1A\xC0\xBC\x8A\xFBW\xC8&\xE4\x98kEY<\x8F\xAD8\x9C\x90` \x01a\x119V[`\0Ta\x01\0\x90\x04`\xFF\x16\x15\x80\x80\x15a!\xB4WP`\0T`\x01`\xFF\x90\x91\x16\x10[\x80a!\xCEWP0;\x15\x80\x15a!\xCEWP`\0T`\xFF\x16`\x01\x14[a\"1W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01Rm\x19\x1EH\x1A[\x9A]\x1AX[\x1A^\x99Y`\x92\x1B`d\x82\x01R`\x84\x01a\x08\x03V[`\0\x80T`\xFF\x19\x16`\x01\x17\x90U\x80\x15a\"TW`\0\x80Ta\xFF\0\x19\x16a\x01\0\x17\x90U[a\"_\x85`\0a0\x0CV[a\"h\x84a,\xFCV[`\0`\x98\x81\x90U`\x01`\x97\x81\x90U`\x99\x91\x90\x91U`\x9A\x80T\x85\x92`\xFF\x19\x90\x91\x16\x90\x83\x81\x81\x11\x15a\"\x9AWa\"\x9Aa6\xDFV[\x02\x17\x90UP`\x9A\x80Ta\x01\0`\x01`\xA8\x1B\x03\x19\x16a\x01\0`\x01`\x01`\xA0\x1B\x03\x85\x16\x02\x17\x90U\x80\x15a#\x06W`\0\x80Ta\xFF\0\x19\x16\x90U`@Q`\x01\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01[`@Q\x80\x91\x03\x90\xA1[PPPPPV[`\0\x83\x81R`\x9D` \x90\x81R`@\x91\x82\x90 \x82Q\x80\x84\x01\x90\x93R\x80T\x80\x84R`\x01\x90\x91\x01T\x91\x83\x01\x91\x90\x91R\x15\x80\x15\x90a#JWP` \x81\x01Q\x15\x15[a#\x8CW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x13`$\x82\x01Rr\x15[\x9A\xDB\x9B\xDD\xDB\x88\x1BY\\\x9A\xDB\x19H\x1C\x9B\xDB\xDD`j\x1B`D\x82\x01R`d\x01a\x08\x03V[`\0\x85\x81R`\x9E` R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x16s\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x10\x19\x01a#\xFBW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x11`$\x82\x01Rp\x10[\x1C\x99XY\x1EH\x1C\x1C\x9B\xD8\xD9\\\xDC\xD9Y`z\x1B`D\x82\x01R`d\x01a\x08\x03V[\x80Q` \x82\x01Q\x10\x15a$[W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FInvalid request range, end < sta`D\x82\x01Ra\x1C\x9D`\xF2\x1B`d\x82\x01R`\x84\x01a\x08\x03V[\x80Q\x86\x10\x80a$mWP\x80` \x01Q\x86\x11[\x15a$\xBAW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1B`$\x82\x01R\x7FRequest id outside of range\0\0\0\0\0`D\x82\x01R`d\x01a\x08\x03V[\x80Q` \x82\x01Qc\xFF\xFF\xFF\xFF\x91a$\xD0\x91a;XV[a$\xDB\x90`\x01a>\x9AV[\x11\x15a%\x19W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\r`$\x82\x01RlRange too big`\x98\x1B`D\x82\x01R`d\x01a\x08\x03V[\x80Q` \x82\x01Q`\0\x91a%,\x91a;XV[a%7\x90`\x01a>\x9AV[\x82Q\x90\x91P`\0\x90a%I\x90\x89a;XV[\x90P\x85a%Y\x88\x83\x88\x88\x87a\x1A}V[\x14a%\x96W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\r`$\x82\x01Rl$\xB7;0\xB64\xB2\x10897\xB7\xB3`\x99\x1B`D\x82\x01R`d\x01a\x08\x03V[PPPPPPPPV[`\0`\x01`\x97Ta%\xB1\x91\x90a;XV[``\x84\x015\x11\x15a%\xC4WP`\x01a&\x11V[`\0a%\xD8`@\x85\x015``\x86\x015a\x14\xCBV[\x90P`\0\x81`@Q` \x01a%\xED\x91\x90a7\x85V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R\x80Q` \x90\x91\x01 `\x80\x86\x015\x14\x15\x92PPP[`@\x80Q`\xC0\x81\x01\x90\x91RB\x90`\0\x90\x80`\x80\x81\x01\x80\x84\x81R` \x01`\x97`\0\x81T\x80\x92\x91\x90a&@\x90a<\xAEV[\x90\x91UP\x90R\x81R` \x87\x81\x015\x81\x83\x01R\x85\x15\x15`@\x80\x84\x01\x91\x90\x91R``\x90\x92\x01\x85\x90R\x82Q\x81\x01Q`\0\x90\x81R`\x9B\x90\x91R \x81Q\x80Q\x82T\x93\x94P\x84\x93\x83\x91\x90\x82\x90`\xFF\x19\x16`\x01\x83\x81\x81\x11\x15a&\x9DWa&\x9Da6\xDFV[\x02\x17\x90UP` \x91\x82\x01Q`\x01\x91\x90\x91\x01U\x82\x81\x01Q`\x02\x83\x01U`@\x80\x84\x01Q`\x03\x84\x01\x80T`\xFF\x19\x16\x91\x15\x15\x91\x90\x91\x17\x90U``\x93\x84\x01Q`\x04\x90\x93\x01\x92\x90\x92U\x83\x81\x01Q\x84\x83\x01Q\x83Q\x91\x82R\x15\x15\x91\x81\x01\x91\x90\x91R\x90\x81\x01\x86\x90R\x7F\x9E\xF1\x13S\xAF\xD9}3\x9Aws(P\xB7\xC2'\x04eeX\xD9\xBAc\xCC~2\x1E\n\xC4\xC2\n\xA9\x91\x01a\"\xFDV[\x81\x81\x11\x15a'DW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08\x03\x90a;\x8DV[`\x01`\x01`\xA0\x1B\x03\x83\x16a'\x92W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x15`$\x82\x01RtInvalid token address`X\x1B`D\x82\x01R`d\x01a\x08\x03V[`\0\x82\x11a'\xB2W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08\x03\x90a?EV[3\x83a'\xC9`\x01`\x01`\xA0\x1B\x03\x82\x16\x830\x87a):V[`@\x80Qa\x01\0\x81\x01\x90\x91RB\x90`\0\x90\x80`\xC0\x81\x01\x80\x84\x81R` \x01`\x97`\0\x81T\x80\x92\x91\x90a'\xF9\x90a<\xAEV[\x90\x91UP\x90R\x81R`\x01`\x01`\xA0\x1B\x03\x80\x87\x16` \x80\x84\x01\x91\x90\x91R\x90\x8A\x16`@\x80\x84\x01\x91\x90\x91R``\x83\x01\x8A\x90R`\x80\x83\x01\x86\x90R`\xA0\x90\x92\x01\x88\x90R\x82Q\x81\x01Q`\0\x90\x81R`\x9C\x90\x91R \x81Q\x80Q\x82T\x93\x94P\x84\x93\x83\x91\x90\x82\x90`\xFF\x19\x16`\x01\x83\x81\x81\x11\x15a(nWa(na6\xDFV[\x02\x17\x90UP` \x91\x82\x01Q`\x01\x91\x90\x91\x01U\x82\x81\x01Q`\x02\x83\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x90\x81\x16`\x01`\x01`\xA0\x1B\x03\x93\x84\x16\x17\x90\x91U`@\x80\x86\x01Q`\x03\x86\x01\x80T\x90\x93\x16\x90\x84\x16\x17\x90\x91U``\x80\x86\x01Q`\x04\x86\x01U`\x80\x80\x87\x01Q`\x05\x87\x01U`\xA0\x96\x87\x01Q`\x06\x90\x96\x01\x95\x90\x95U\x86Q\x84\x01Q\x82Q\x90\x81R\x8A\x84\x16\x94\x81\x01\x94\x90\x94R\x91\x8C\x16\x90\x83\x01R\x81\x01\x89\x90R\x90\x81\x01\x87\x90R\x7F\"S\x05\xEC\xB6o\xA9\xB9\xB2\x9F\x8D\xEA\xD9\xBA\xEA6Zl\"]c\x9D\xFD\x86nx,\xCBc\xE2\xF0[\x91\x01`@Q\x80\x91\x03\x90\xA1PPPPPPPV[`@Q`\x01`\x01`\xA0\x1B\x03\x80\x85\x16`$\x83\x01R\x83\x16`D\x82\x01R`d\x81\x01\x82\x90Ra)\xA5\x90\x85\x90c#\xB8r\xDD`\xE0\x1B\x90`\x84\x01[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16`\x01`\x01`\xE0\x1B\x03\x19\x90\x93\x16\x92\x90\x92\x17\x90\x91Ra0\xF2V[PPPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16a*9W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`I`$\x82\x01R\x7FPausable._setPauserRegistry: new`D\x82\x01R\x7FPauserRegistry cannot be the zer`d\x82\x01Rho address`\xB8\x1B`\x84\x82\x01R`\xA4\x01a\x08\x03V[`eT`@\x80Q`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x81R\x91\x83\x16` \x83\x01R\x7Fn\x9F\xCDS\x98\x96\xFC\xA6\x0E\x8B\x0F\x01\xDDX\x023\xE4\x8Ak\x0F}\xF0\x13\xB8\x9B\xA7\xF5e\x86\x9A\xCD\xB6\x91\x01`@Q\x80\x91\x03\x90\xA1`e\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[01\x81\x11\x15a*\xF3W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1C`$\x82\x01R\x7FNot enough funds in contract\0\0\0\0`D\x82\x01R`d\x01a\x08\x03V[`\0\x81\x11a+\x13W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08\x03\x90a?EV[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x84\x16\x81R` \x81\x01\x83\x90R\x7F\xE0ISU\xC1\xE0LQ%\x84R\x18T\xD2\"\xD29\xA4\xB7\x82\xB3\x9A\xC8\xA7\xE85\xA3O^\xC7\xC1\xE1\x91\x01`@Q\x80\x91\x03\x90\xA1a+_\x82\x82a1\xC9V[PPV[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R\x82\x90\x82\x90`\x01`\x01`\xA0\x1B\x03\x83\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a+\xABW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a+\xCF\x91\x90a;\xC4V[\x10\x15a,\x1DW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1C`$\x82\x01R\x7FNot enough funds in contract\0\0\0\0`D\x82\x01R`d\x01a\x08\x03V[`\0\x82\x11a,=W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08\x03\x90a?EV[a,Q`\x01`\x01`\xA0\x1B\x03\x82\x16\x85\x84a2\xE2V[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x80\x87\x16\x82R\x85\x16` \x82\x01R\x90\x81\x01\x83\x90R~\xE7c\xF7w\x8B\x8C\xEE\xF7'\x0C\x89\xB7\xD1\xDF\x10\x08\xB0\xE4\x82\xDA9\xC481Aw3\xAF\x96\xFB\r\x90``\x01[`@Q\x80\x91\x03\x90\xA1PPPPV[`3T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x14\xC9W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\x08\x03V[`3\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x90\x93U`@Q\x91\x16\x91\x90\x82\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90`\0\x90\xA3PPV[`@\x80\x83\x015`\0\x90\x81R`\x9C` R\x90\x81 `\x02\x81\x01T\x90\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90a-\x84`\x80\x86\x01``\x87\x01a5\xF4V[`\x01`\x01`\xA0\x1B\x03\x16\x14a-\xA5Wa-\xA2`\x80\x85\x01``\x86\x01a5\xF4V[\x90P[`\x03\x82\x01T`\x01`\x01`\xA0\x1B\x03\x16`\0\x19\x01a-\xCEWa-\xC9\x81\x83`\x04\x01Ta*\xA2V[a-\xEFV[`\x03\x82\x01T`\x04\x83\x01Ta-\xEF\x91\x83\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90a+cV[`@\x80Q` \x86\x81\x015\x82R\x86\x83\x015\x90\x82\x01R\x90\x81\x01\x84\x90R\x7F\x13u\x0Cs\x1F\x87\xC1RB\x87L\xE7K\xF4d\x95\x02\xCC\x8E|\x82\x90g\xCE\x84e\x05\xAC\xDB\x96(\x9D\x90``\x01a,\x94V[4\x81\x11\x15a.SW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08\x03\x90a;\x8DV[`\x004\x11a.\xA3W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7Fmsg value must be greater that 0`D\x82\x01R`d\x01a\x08\x03V[`@\x80Qa\x01\0\x81\x01\x90\x91R3\x904\x90B\x90`\0\x90\x80`\xC0\x81\x01\x80\x84\x81R` \x01`\x97`\0\x81T\x80\x92\x91\x90a.\xD7\x90a<\xAEV[\x90\x91UP\x90R\x81R`\x01`\x01`\xA0\x1B\x03\x86\x16` \x80\x83\x01\x91\x90\x91R`\x01`@\x80\x84\x01\x82\x90R``\x84\x01\x88\x90R`\x80\x84\x01\x87\x90R`\xA0\x90\x93\x01\x89\x90R\x83Q\x82\x01Q`\0\x90\x81R`\x9C\x90\x92R\x91\x90 \x82Q\x80Q\x82T\x94\x95P\x85\x94\x92\x93\x91\x92\x84\x92\x83\x91`\xFF\x19\x16\x90\x83\x81\x81\x11\x15a/MWa/Ma6\xDFV[\x02\x17\x90UP` \x91\x82\x01Q`\x01\x91\x82\x01U\x83\x82\x01Q`\x02\x84\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x90\x81\x16`\x01`\x01`\xA0\x1B\x03\x93\x84\x16\x17\x90\x91U`@\x80\x87\x01Q`\x03\x87\x01\x80T\x90\x93\x16\x90\x84\x16\x17\x90\x91U``\x80\x87\x01Q`\x04\x87\x01U`\x80\x80\x88\x01Q`\x05\x88\x01U`\xA0\x97\x88\x01Q`\x06\x90\x97\x01\x96\x90\x96U\x87Q\x85\x01Q\x82Q\x90\x81R\x92\x8B\x16\x94\x83\x01\x94\x90\x94R\x81\x01\x91\x90\x91R\x90\x81\x01\x86\x90R\x90\x81\x01\x87\x90R\x7F\"S\x05\xEC\xB6o\xA9\xB9\xB2\x9F\x8D\xEA\xD9\xBA\xEA6Zl\"]c\x9D\xFD\x86nx,\xCBc\xE2\xF0[\x91\x01a\"\xFDV[`eT`\x01`\x01`\xA0\x1B\x03\x16\x15\x80\x15a0-WP`\x01`\x01`\xA0\x1B\x03\x82\x16\x15\x15[a0\xAFW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`G`$\x82\x01R\x7FPausable._initializePauser: _ini`D\x82\x01R\x7FtializePauser() can only be call`d\x82\x01Rfed once`\xC8\x1B`\x84\x82\x01R`\xA4\x01a\x08\x03V[`f\x81\x90U`@Q\x81\x81R3\x90\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=\x90` \x01`@Q\x80\x91\x03\x90\xA2a+_\x82a)\xABV[`\0a1G\x82`@Q\x80`@\x01`@R\x80` \x81R` \x01\x7FSafeERC20: low-level call failed\x81RP\x85`\x01`\x01`\xA0\x1B\x03\x16a3\x12\x90\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x80Q\x90\x91P\x15a1\xC4W\x80\x80` \x01\x90Q\x81\x01\x90a1e\x91\x90a<DV[a1\xC4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`*`$\x82\x01R\x7FSafeERC20: ERC20 operation did n`D\x82\x01Ri\x1B\xDD\x08\x1C\xDDX\xD8\xD9YY`\xB2\x1B`d\x82\x01R`\x84\x01a\x08\x03V[PPPV[\x80G\x10\x15a2\x19W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FAddress: insufficient balance\0\0\0`D\x82\x01R`d\x01a\x08\x03V[`\0\x82`\x01`\x01`\xA0\x1B\x03\x16\x82`@Q`\0`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80`\0\x81\x14a2fW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a2kV[``\x91P[PP\x90P\x80a1\xC4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`:`$\x82\x01R\x7FAddress: unable to send value, r`D\x82\x01R\x7Fecipient may have reverted\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x08\x03V[`@Q`\x01`\x01`\xA0\x1B\x03\x83\x16`$\x82\x01R`D\x81\x01\x82\x90Ra1\xC4\x90\x84\x90c\xA9\x05\x9C\xBB`\xE0\x1B\x90`d\x01a)nV[``a3!\x84\x84`\0\x85a3+V[\x90P[\x93\x92PPPV[``\x82G\x10\x15a3\x8CW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FAddress: insufficient balance fo`D\x82\x01Re\x1C\x88\x18\xD8[\x1B`\xD2\x1B`d\x82\x01R`\x84\x01a\x08\x03V[`\x01`\x01`\xA0\x1B\x03\x85\x16;a3\xE3W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FAddress: call to non-contract\0\0\0`D\x82\x01R`d\x01a\x08\x03V[`\0\x80\x86`\x01`\x01`\xA0\x1B\x03\x16\x85\x87`@Qa3\xFF\x91\x90a?zV[`\0`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80`\0\x81\x14a4<W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a4AV[``\x91P[P\x91P\x91Pa\x1EE\x82\x82\x86``\x83\x15a4[WP\x81a3$V[\x82Q\x15a4kW\x82Q\x80\x84` \x01\xFD[\x81`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08\x03\x91\x90a?\x96V[`\0`\xA0\x82\x84\x03\x12\x15a4\x97W`\0\x80\xFD[P\x91\x90PV[`\0\x80\x83`\x1F\x84\x01\x12a4\xAFW`\0\x80\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a4\xC7W`\0\x80\xFD[` \x83\x01\x91P\x83` \x82`\x05\x1B\x85\x01\x01\x11\x15a4\xE2W`\0\x80\xFD[\x92P\x92\x90PV[`\0\x80`\0\x80`\xE0\x85\x87\x03\x12\x15a4\xFFW`\0\x80\xFD[a5\t\x86\x86a4\x85V[\x93P`\xA0\x85\x015\x92P`\xC0\x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a5,W`\0\x80\xFD[a58\x87\x82\x88\x01a4\x9DV[\x95\x98\x94\x97P\x95PPPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x10\x02W`\0\x80\xFD[`\0\x80`\0``\x84\x86\x03\x12\x15a5nW`\0\x80\xFD[\x835a5y\x81a5DV[\x95` \x85\x015\x95P`@\x90\x94\x015\x93\x92PPPV[`\0\x80\x82\x84\x03``\x81\x12\x15a5\xA2W`\0\x80\xFD[\x835\x92P`@`\x1F\x19\x82\x01\x12\x15a5\xB8W`\0\x80\xFD[P` \x83\x01\x90P\x92P\x92\x90PV[`\0`\xC0\x82\x84\x03\x12\x15a4\x97W`\0\x80\xFD[`\0`\xC0\x82\x84\x03\x12\x15a5\xEAW`\0\x80\xFD[a3$\x83\x83a5\xC6V[`\0` \x82\x84\x03\x12\x15a6\x06W`\0\x80\xFD[\x815a3$\x81a5DV[`\0` \x82\x84\x03\x12\x15a6#W`\0\x80\xFD[P5\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a6=W`\0\x80\xFD[\x825a6H\x81a5DV[\x94` \x93\x90\x93\x015\x93PPPV[`\0\x80`\0\x80a\x01\0\x85\x87\x03\x12\x15a6mW`\0\x80\xFD[a6w\x86\x86a5\xC6V[\x93P`\xC0\x85\x015\x92P`\xE0\x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a5,W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a6\xACW`\0\x80\xFD[\x815`\xFF\x81\x16\x81\x14a3$W`\0\x80\xFD[`\0\x80`@\x83\x85\x03\x12\x15a6\xD0W`\0\x80\xFD[PP\x805\x92` \x90\x91\x015\x91PV[cNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[`\x02\x81\x10a\x10\x02Wa\x10\x02a6\xDFV[\x80Qa7\x10\x81a6\xF5V[\x82R` \x90\x81\x01Q\x91\x01RV[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15a7zW\x81Qa7F\x88\x82Qa7\x05V[\x80\x84\x01Q`@\x89\x81\x01\x91\x90\x91R\x81\x01Q\x15\x15``\x80\x8A\x01\x91\x90\x91R\x01Q`\x80\x88\x01R`\xA0\x90\x96\x01\x95\x90\x82\x01\x90`\x01\x01a71V[P\x94\x95\x94PPPPPV[`\0` \x80\x83R`\x80\x80\x84\x01\x85Qa7\x9C\x81a6\xF5V[\x85\x84\x01R\x85\x83\x01Q```@\x80\x88\x01\x82\x90R\x82Q\x93\x84\x90R`\xA0\x93\x92\x86\x01\x92\x84\x89\x01\x90`\0[\x81\x81\x10\x15a8\x1DW\x85Qa7\xD7\x84\x82Qa7\x05V[\x80\x8A\x01Q`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x85\x87\x01R\x85\x82\x01Q\x16\x86\x85\x01R\x85\x81\x01Q\x89\x85\x01R\x88\x81\x01Q\x88\x85\x01R\x87\x01Q`\xC0\x84\x01R\x94\x88\x01\x94`\xE0\x90\x92\x01\x91`\x01\x01a7\xC2V[PP\x89\x82\x01Q\x89\x82\x03`\x1F\x19\x01\x84\x8B\x01R\x96Pa8:\x81\x88a7\x1DV[\x9A\x99PPPPPPPPPPV[`\0`\x80\x82\x84\x03\x12\x15a4\x97W`\0\x80\xFD[`\0\x80`\0\x80`\xC0\x85\x87\x03\x12\x15a8pW`\0\x80\xFD[a8z\x86\x86a8HV[\x93P`\x80\x85\x015\x92P`\xA0\x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a5,W`\0\x80\xFD[`\0`\x80\x82\x84\x03\x12\x15a8\xAFW`\0\x80\xFD[a3$\x83\x83a8HV[\x805c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a8\xCDW`\0\x80\xFD[\x91\x90PV[`\0\x80`\0\x80`\0`\x80\x86\x88\x03\x12\x15a8\xEAW`\0\x80\xFD[\x855\x94Pa8\xFA` \x87\x01a8\xB9V[\x93P`@\x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a9\x16W`\0\x80\xFD[a9\"\x88\x82\x89\x01a4\x9DV[\x90\x94P\x92Pa95\x90P``\x87\x01a8\xB9V[\x90P\x92\x95P\x92\x95\x90\x93PV[`\xE0\x81\x01a9O\x82\x89a7\x05V[`\x01`\x01`\xA0\x1B\x03\x96\x87\x16`@\x83\x01R\x94\x90\x95\x16``\x86\x01R`\x80\x85\x01\x92\x90\x92R`\xA0\x84\x01R`\xC0\x90\x92\x01\x91\x90\x91R\x91\x90PV[` \x81\x01a9\x90\x83a6\xF5V[\x91\x90R\x90V[`\xA0\x81\x01a9\xA4\x82\x87a7\x05V[`@\x82\x01\x94\x90\x94R\x91\x15\x15``\x83\x01R`\x80\x90\x91\x01R\x91\x90PV[`\0`\xA0\x82\x84\x03\x12\x15a9\xD1W`\0\x80\xFD[a3$\x83\x83a4\x85V[`\0\x80`\0\x80`\0\x80`\0`\xC0\x88\x8A\x03\x12\x15a9\xF6W`\0\x80\xFD[a9\xFF\x88a8\xB9V[\x96Pa:\r` \x89\x01a8\xB9V[\x95P`@\x88\x015\x94P``\x88\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a:0W`\0\x80\xFD[a:<\x8A\x82\x8B\x01a4\x9DV[\x90\x95P\x93Pa:O\x90P`\x80\x89\x01a8\xB9V[\x91Pa:]`\xA0\x89\x01a8\xB9V[\x90P\x92\x95\x98\x91\x94\x97P\x92\x95PV[`\x02\x81\x10a\x10\x02W`\0\x80\xFD[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a:\x8EW`\0\x80\xFD[\x845a:\x99\x81a5DV[\x93P` \x85\x015a:\xA9\x81a5DV[\x92P`@\x85\x015a:\xB9\x81a:kV[\x91P``\x85\x015a:\xC9\x81a5DV[\x93\x96\x92\x95P\x90\x93PPV[` \x80\x82R`\x1C\x90\x82\x01R\x7FPausable: contract is paused\0\0\0\0`@\x82\x01R``\x01\x90V[` \x80\x82R`\x1F\x90\x82\x01R\x7FReentrancyGuard: reentrant call\0`@\x82\x01R``\x01\x90V[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0\x82\x82\x10\x15a;jWa;ja;BV[P\x03\x90V[\x82\x81R``\x81\x01a3$` \x83\x01\x84\x805\x82R` \x90\x81\x015\x91\x01RV[` \x80\x82R`\x1C\x90\x82\x01R\x7FTip exceeds deposited amount\0\0\0\0`@\x82\x01R``\x01\x90V[`\0` \x82\x84\x03\x12\x15a;\xD6W`\0\x80\xFD[PQ\x91\x90PV[`\0` \x82\x84\x03\x12\x15a;\xEFW`\0\x80\xFD[\x81Qa3$\x81a5DV[` \x80\x82R`*\x90\x82\x01R\x7Fmsg.sender is not permissioned a`@\x82\x01Ri9\x90:\xB780\xBA\xB9\xB2\xB9`\xB1\x1B``\x82\x01R`\x80\x01\x90V[`\0` \x82\x84\x03\x12\x15a<VW`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a3$W`\0\x80\xFD[` \x80\x82R`(\x90\x82\x01R\x7Fmsg.sender is not permissioned a`@\x82\x01Rg9\x9080\xBA\xB9\xB2\xB9`\xC1\x1B``\x82\x01R`\x80\x01\x90V[`\0`\x01\x82\x01a<\xC0Wa<\xC0a;BV[P`\x01\x01\x90V[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[` \x81\x01`\x03\x83\x10a9\x90Wa9\x90a6\xDFV[\x805a=\x12\x81a:kV[a=\x1B\x81a6\xF5V[\x82R` \x90\x81\x015\x91\x01RV[`\xC0\x81\x01a=6\x82\x84a=\x07V[`@\x83\x015a=D\x81a5DV[`\x01`\x01`\xA0\x1B\x03\x90\x81\x16`@\x84\x01R``\x84\x015\x90a=c\x82a5DV[\x16``\x83\x01R`\x80\x83\x81\x015\x90\x83\x01R`\xA0\x92\x83\x015\x92\x90\x91\x01\x91\x90\x91R\x90V[`\0[\x83\x81\x10\x15a=\x9FW\x81\x81\x01Q\x83\x82\x01R` \x01a=\x87V[\x83\x81\x11\x15a)\xA5WPP`\0\x91\x01RV[`\0\x83Qa=\xC2\x81\x84` \x88\x01a=\x84V[\x83Q\x90\x83\x01\x90a=\xD6\x81\x83` \x88\x01a=\x84V[\x01\x94\x93PPPPV[`\x80\x81\x01a=\xED\x82\x84a=\x07V[`@\x83\x015`@\x83\x01R``\x83\x015a>\x05\x81a5DV[`\x01`\x01`\xA0\x1B\x03\x16``\x92\x90\x92\x01\x91\x90\x91R\x91\x90PV[cNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[`\0c\xFF\xFF\xFF\xFF\x80\x84\x16\x80a>JWa>Ja>\x1DV[\x92\x16\x91\x90\x91\x04\x92\x91PPV[`\0c\xFF\xFF\xFF\xFF\x80\x83\x16\x81\x85\x16\x80\x83\x03\x82\x11\x15a=\xD6Wa=\xD6a;BV[`\0c\xFF\xFF\xFF\xFF\x83\x81\x16\x90\x83\x16\x81\x81\x10\x15a>\x92Wa>\x92a;BV[\x03\x93\x92PPPV[`\0\x82\x19\x82\x11\x15a>\xADWa>\xADa;BV[P\x01\x90V[`\xA0\x81\x01a>\xC0\x82\x84a=\x07V[a>\xDA`@\x83\x01`@\x85\x01\x805\x82R` \x90\x81\x015\x91\x01RV[`\x80\x92\x83\x015\x91\x90\x92\x01R\x90V[`\0c\xFF\xFF\xFF\xFF\x80\x84\x16\x80a>\xFFWa>\xFFa>\x1DV[\x92\x16\x91\x90\x91\x06\x92\x91PPV[`\0c\xFF\xFF\xFF\xFF\x80\x83\x16\x81\x81\x03a?$Wa?$a;BV[`\x01\x01\x93\x92PPPV[`\0\x81a?=Wa?=a;BV[P`\0\x19\x01\x90V[` \x80\x82R\x81\x81\x01R\x7FAmount must be greater than zero`@\x82\x01R``\x01\x90V[`\0\x82Qa?\x8C\x81\x84` \x87\x01a=\x84V[\x91\x90\x91\x01\x92\x91PPV[` \x81R`\0\x82Q\x80` \x84\x01Ra?\xB5\x81`@\x85\x01` \x87\x01a=\x84V[`\x1F\x01`\x1F\x19\x16\x91\x90\x91\x01`@\x01\x92\x91PPV\xFE\xA2dipfsX\"\x12 n\xE6\xC2\x9B\x88\xFBe\x99\xF0!\x19\x95z\"\xD7o'T\xFF\xF6-.:2\xF4\xAA\xD5\xAB\x96\x17\x11fdsolcC\0\x08\r\x003",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x6080604052600436106102675760003560e01c8063950ac48711610144578063d16544f0116100b6578063f26ee9d01161007a578063f26ee9d014610738578063f2fde38b1461074e578063f9ecd01e1461076e578063fabc1cbc1461078e578063ff2bae86146107ae578063ffea632b146107c357600080fd5b8063d16544f014610366578063de70e0b8146106ba578063df2ebdbb146106f0578063dffbdd9f14610705578063ef0ba5d01461071857600080fd5b8063b153870611610108578063b153870614610606578063c2b40ae41461061b578063c763e5a11461063b578063c87c222414610662578063ca9b21ae1461066a578063cc8c909f1461069a57600080fd5b8063950ac487146105545780639d54f41914610574578063ae46db1114610594578063af26c695146105b4578063b02c43d0146105d457600080fd5b8063595c6a67116101dd57806371c54461116101a157806371c544611461048e57806379e041f2146104b35780637fd4f845146104e0578063886f1195146104f6578063890e95ce146105165780638da5cb5b1461053657600080fd5b8063595c6a67146103ef5780635ac86ab7146104045780635c975abb1461044457806361bc221a14610463578063715018a61461047957600080fd5b80630efe6a8b1161022f5780630efe6a8b1461028e57806310d67a2f14610326578063136439dd1461034657806347e7ef24146103665780634bf5fec3146103865780634f48eedf146103a657600080fd5b806301ef69661461026c57806308aba1b21461028e57806308f42d40146102ae5780630cac57ab146102ce5780630e2636a3146102e1575b600080fd5b34801561027857600080fd5b5061028c6102873660046134e9565b6107e3565b005b34801561029a57600080fd5b5061028c6102a9366004613559565b610898565b3480156102ba57600080fd5b5061028c6102c936600461358e565b6108f4565b61028c6102dc3660046135d8565b610b47565b3480156102ed57600080fd5b5061030973111111111111111111111111111111111111111181565b6040516001600160a01b0390911681526020015b60405180910390f35b34801561033257600080fd5b5061028c6103413660046135f4565b610f52565b34801561035257600080fd5b5061028c610361366004613611565b611005565b34801561037257600080fd5b5061028c61038136600461362a565b611144565b34801561039257600080fd5b5061028c6103a1366004613656565b6111a0565b3480156103b257600080fd5b506103da6103c1366004613611565b609d602052600090815260409020805460019091015482565b6040805192835260208301919091520161031d565b3480156103fb57600080fd5b5061028c6113f0565b34801561041057600080fd5b5061043461041f36600461369a565b606654600160ff9092169190911b9081161490565b604051901515815260200161031d565b34801561045057600080fd5b506066545b60405190815260200161031d565b34801561046f57600080fd5b5061045560975481565b34801561048557600080fd5b5061028c6114b7565b34801561049a57600080fd5b50609a546103099061010090046001600160a01b031681565b3480156104bf57600080fd5b506104d36104ce3660046136bd565b6114cb565b60405161031d9190613785565b3480156104ec57600080fd5b5061045560985481565b34801561050257600080fd5b50606554610309906001600160a01b031681565b34801561052257600080fd5b506104556105313660046135d8565b6118e5565b34801561054257600080fd5b506033546001600160a01b0316610309565b34801561056057600080fd5b5061028c61056f36600461385a565b611953565b34801561058057600080fd5b5061028c61058f3660046135f4565b6119c2565b3480156105a057600080fd5b506104556105af36600461389d565b611a49565b3480156105c057600080fd5b506104556105cf3660046138d2565b611a7d565b3480156105e057600080fd5b506105f46105ef366004613611565b611ace565b60405161031d96959493929190613941565b34801561061257600080fd5b506104d3611b55565b34801561062757600080fd5b50610455610636366004613611565b611ba0565b34801561064757600080fd5b50609a546106559060ff1681565b60405161031d9190613983565b61028c611bc1565b34801561067657600080fd5b5061068a610685366004613611565b611c19565b60405161031d9493929190613996565b3480156106a657600080fd5b506104556106b53660046139bf565b611c8b565b3480156106c657600080fd5b506103096106d5366004613611565b609e602052600090815260409020546001600160a01b031681565b3480156106fc57600080fd5b50610309600181565b61028c610713366004613611565b611cbf565b34801561072457600080fd5b506104556107333660046139db565b611d17565b34801561074457600080fd5b5061045560995481565b34801561075a57600080fd5b5061028c6107693660046135f4565b611e50565b34801561077a57600080fd5b50610455610789366004613611565b611ec6565b34801561079a57600080fd5b5061028c6107a9366004613611565b612038565b3480156107ba57600080fd5b50609f54610455565b3480156107cf57600080fd5b5061028c6107de366004613a78565b612194565b6066541561080c5760405162461bcd60e51b815260040161080390613ad4565b60405180910390fd5b600260d2540361082e5760405162461bcd60e51b815260040161080390613b0b565b600260d255600061083e85611c8b565b905061085160208601358286868661230d565b61085b85826125a0565b6000908152609e6020526040902080546001600160a01b0319167311111111111111111111111111111111111111111790555050600160d2555050565b606654156108b85760405162461bcd60e51b815260040161080390613ad4565b600260d254036108da5760405162461bcd60e51b815260040161080390613b0b565b600260d2556108ea838383612724565b5050600160d25550565b606654156109145760405162461bcd60e51b815260040161080390613ad4565b609a5461010090046001600160a01b031633146109635760405162461bcd60e51b815260206004820152600d60248201526c2737ba103a34329037bbb732b960991b6044820152606401610803565b6099548160200135116109b85760405162461bcd60e51b815260206004820152601960248201527f557064617465206272696e6773206e6f206e65772064617461000000000000006044820152606401610803565b8035610a065760405162461bcd60e51b815260206004820152601f60248201527f72616e6765206964206d7573742062652067726561746572207468616e2030006044820152606401610803565b609954610a1560018335613b58565b1115610a635760405162461bcd60e51b815260206004820152601760248201527f50726576696f757320757064617465206d697373696e670000000000000000006044820152606401610803565b803560208201351015610aa85760405162461bcd60e51b815260206004820152600d60248201526c496e76616c69642072616e676560981b6044820152606401610803565b609f8054600181019091557f0bc14066c33013fe88f66e314e4cf150b0b2d4d6451a1a51dbbd1c27cd11de28018290556000828152609d602052604090208190610aff828281358155602082013560018201555050565b505060208101356099556040517f49c158d490db9e066f01b5d4f1a094485a6598cb6c5296b4c07e46c12a1dc11c90610b3b9084908490613b6f565b60405180910390a15050565b60665415610b675760405162461bcd60e51b815260040161080390613ad4565b600260d25403610b895760405162461bcd60e51b815260040161080390613b0b565b600260d255608081013560a08201351115610bb65760405162461bcd60e51b815260040161080390613b8d565b6000610bca60a08301356080840135613b58565b90506000610bd7836118e5565b6000818152609e60205260409020549091506001600160a01b031615610c315760405162461bcd60e51b815260206004820152600f60248201526e105b1c9958591e4819995c9c9a5959608a1b6044820152606401610803565b6000818152609e6020526040902080546001600160a01b031916331790556001610c6160808501606086016135f4565b6001600160a01b031603610df65760003411610cb75760405162461bcd60e51b815260206004820152601560248201527413985d1a5d99481d1bdad95b881b9bdd081cd95b9d605a1b6044820152606401610803565b813414610d3d5760405162461bcd60e51b815260206004820152604860248201527f53656e7420616d6f756e742073686f756c642065786163746c79206d6174636860448201527f207769746864726177616c2e616d6f756e74202d207769746864726177616c2e606482015267066657272795469760c41b608482015260a401610803565b610d4d60608401604085016135f4565b6001600160a01b03166108fc839081150290604051600060405180830381858888f19350505050158015610d85573d6000803e3d6000fd5b507f7a9abd9eb86bdbca89cba4069a632c37d93db82e3e14ad8119a3a7812814853e602084013583610dbd60608701604088016135f4565b6040805193845260208401929092526001600160a01b0316908201523360608201526080810183905260a00160405180910390a16108ea565b6000610e0860808501606086016135f4565b6040516370a0823160e01b815233600482015290915083906001600160a01b038316906370a0823190602401602060405180830381865afa158015610e51573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610e759190613bc4565b1015610eb65760405162461bcd60e51b815260206004820152601060248201526f4e6f7420656e6f7567682066756e647360801b6044820152606401610803565b610edc33610eca60608701604088016135f4565b6001600160a01b03841691908661293a565b7f7a9abd9eb86bdbca89cba4069a632c37d93db82e3e14ad8119a3a7812814853e602085013584610f1360608801604089016135f4565b6040805193845260208401929092526001600160a01b0316908201523360608201526080810184905260a00160405180910390a1505050600160d25550565b606560009054906101000a90046001600160a01b03166001600160a01b031663eab66d7a6040518163ffffffff1660e01b8152600401602060405180830381865afa158015610fa5573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610fc99190613bdd565b6001600160a01b0316336001600160a01b031614610ff95760405162461bcd60e51b815260040161080390613bfa565b611002816129ab565b50565b60655460405163237dfb4760e11b81523360048201526001600160a01b03909116906346fbf68e90602401602060405180830381865afa15801561104d573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906110719190613c44565b61108d5760405162461bcd60e51b815260040161080390613c66565b606654818116146111065760405162461bcd60e51b815260206004820152603860248201527f5061757361626c652e70617573653a20696e76616c696420617474656d70742060448201527f746f20756e70617573652066756e6374696f6e616c69747900000000000000006064820152608401610803565b606681905560405181815233907fab40a374bc51de372200a8bc981af8c9ecdc08dfdaef0bb6e09f88f3c616ef3d906020015b60405180910390a250565b606654156111645760405162461bcd60e51b815260040161080390613ad4565b600260d254036111865760405162461bcd60e51b815260040161080390613b0b565b600260d25561119782826000612724565b5050600160d255565b606654156111c05760405162461bcd60e51b815260040161080390613ad4565b600260d254036111e25760405162461bcd60e51b815260040161080390613b0b565b600260d25560006111f2856118e5565b905061120560208601358286868661230d565b6000818152609e6020526040902080546001600160a01b03198116731111111111111111111111111111111111111111179091556001600160a01b03168015158061135457600161125c6080890160608a016135f4565b6001600160a01b0316036112b15761129461127d6060890160408a016135f4565b61128f60a08a013560808b0135613b58565b612aa2565b60a0870135156112ac576112ac338860a00135612aa2565b611313565b6112eb6112c46060890160408a016135f4565b6112d460808a0160608b016135f4565b6112e660a08b013560808c0135613b58565b612b63565b60a087013515611313576113133361130960808a0160608b016135f4565b8960a00135612b63565b60408051602089810135825281018590527f935f26d94be31907080aa78b3e6e2ac6d48a072af096c20268388621bbc11789910160405180910390a16113e2565b60016113666080890160608a016135f4565b6001600160a01b03160361138757611382828860800135612aa2565b6113a5565b6113a58261139b60808a0160608b016135f4565b8960800135612b63565b60408051602089810135825281018590527f2996fd546c37d74c170466ea6aa4a308e3ca2d4aa689e6e9e32994db5039cc0e910160405180910390a15b5050600160d2555050505050565b60655460405163237dfb4760e11b81523360048201526001600160a01b03909116906346fbf68e90602401602060405180830381865afa158015611438573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061145c9190613c44565b6114785760405162461bcd60e51b815260040161080390613c66565b600019606681905560405190815233907fab40a374bc51de372200a8bc981af8c9ecdc08dfdaef0bb6e09f88f3c616ef3d9060200160405180910390a2565b6114bf612ca2565b6114c96000612cfc565b565b6114f06040805160608101909152806000815260200160608152602001606081525090565b6115156040805160608101909152806000815260200160608152602001606081525090565b609a54819060ff16600181111561152e5761152e6136df565b90816001811115611541576115416136df565b90525060008085158015611553575084155b15611563578293505050506118df565b855b858111611608576000818152609c602052604090206001015415611595578261158d81613cae565b9350506115f6565b6000818152609b6020526040902060010154156115be57816115b681613cae565b9250506115f6565b60405162461bcd60e51b815260206004820152600d60248201526c496e76616c69642072616e676560981b6044820152606401610803565b8061160081613cae565b915050611565565b508167ffffffffffffffff81111561162257611622613cc7565b60405190808252806020026020018201604052801561169057816020015b6040805161010081018252600060c0820181815260e0830182905282526020808301829052928201819052606082018190526080820181905260a082015282526000199092019101816116405790505b5060208401528067ffffffffffffffff8111156116af576116af613cc7565b60405190808252806020026020018201604052801561170e57816020015b6040805160c08101825260006080820181815260a0830182905282526020808301829052928201819052606082015282526000199092019101816116cd5790505b506040840152506000905080855b8581116118d8576000818152609c602052604090206001015415611806576000818152609c602052604090819020815161010081019092528054829060c08201908390829060ff166001811115611775576117756136df565b6001811115611786576117866136df565b81526001919091015460209182015290825260028301546001600160a01b03908116838301526003840154166040830152600483015460608301526005830154608083015260069092015460a090910152850151846117e481613cae565b9550815181106117f6576117f6613cdd565b60200260200101819052506118c6565b6000818152609b6020526040902060020154156118c1576000818152609b602052604090819020815160c081019092528054829060808201908390829060ff166001811115611857576118576136df565b6001811115611868576118686136df565b815260019190910154602091820152908252600283015490820152600382015460ff161515604080830191909152600490920154606090910152850151836118af81613cae565b9450815181106117f6576117f6613cdd565b6118d8565b806118d081613cae565b91505061171c565b5091925050505b92915050565b6000806040516020016118f89190613cf3565b604051602081830303815290604052826040516020016119189190613d28565b60408051601f19818403018152908290526119369291602001613db0565b604051602081830303815290604052805190602001209050919050565b606654156119735760405162461bcd60e51b815260040161080390613ad4565b600260d254036119955760405162461bcd60e51b815260040161080390613b0b565b600260d25560006119a585611a49565b90506119b860208601358286868661230d565b61085b8582612d4e565b6119ca612ca2565b606654156119ea5760405162461bcd60e51b815260040161080390613ad4565b609a8054610100600160a81b0319166101006001600160a01b038481168202929092179283905560405192041681527f1b0f2f500df596b42b73e80dbec6a1fb570f01978a586723f988a5fc54d773a19060200160405180910390a150565b60006002604051602001611a5d9190613cf3565b604051602081830303815290604052826040516020016119189190613ddf565b600080825b63ffffffff811615611aad57611a99600282613e33565b9050611aa6600183613e56565b9150611a82565b611ac282888a8989600061073360018c613e75565b98975050505050505050565b609c6020526000908152604090819020815180830190925280549091908290829060ff166001811115611b0357611b036136df565b6001811115611b1457611b146136df565b815260019190910154602090910152600282015460038301546004840154600585015460069095015493946001600160a01b03938416949290931692909186565b611b7a6040805160608101909152806000815260200160608152602001606081525090565b611b9b6098546001611b8c9190613e9a565b60016097546104ce9190613b58565b905090565b609f8181548110611bb057600080fd5b600091825260209091200154905081565b600260d25403611be35760405162461bcd60e51b815260040161080390613b0b565b600260d25560665415611c085760405162461bcd60e51b815260040161080390613ad4565b611c126000612e33565b600160d255565b609b6020526000908152604090819020815180830190925280549091908290829060ff166001811115611c4e57611c4e6136df565b6001811115611c5f57611c5f6136df565b815260019190910154602090910152600282015460038301546004909301549192909160ff9091169084565b60006001604051602001611c9f9190613cf3565b604051602081830303815290604052826040516020016119189190613eb2565b600260d25403611ce15760405162461bcd60e51b815260040161080390613b0b565b600260d25560665415611d065760405162461bcd60e51b815260040161080390613ad4565b611d0f81612e33565b50600160d255565b6000611d24600288613ee8565b63ffffffff16600003611da7578163ffffffff168763ffffffff160315611e055785858585611d5281613f0b565b965063ffffffff16818110611d6957611d69613cdd565b90506020020135604051602001611d8a929190918252602082015260400190565b604051602081830303815290604052805190602001209550611e05565b848484611db381613f0b565b955063ffffffff16818110611dca57611dca613cdd565b9050602002013586604051602001611dec929190918252602082015260400190565b6040516020818303038152906040528051906020012095505b8763ffffffff16600103611e1a575084611e45565b611e42611e2860018a613e75565b611e3360028a613e33565b8888888861073360028a613e33565b90505b979650505050505050565b611e58612ca2565b6001600160a01b038116611ebd5760405162461bcd60e51b815260206004820152602660248201527f4f776e61626c653a206e6577206f776e657220697320746865207a65726f206160448201526564647265737360d01b6064820152608401610803565b61100281612cfc565b6000609954821115611f0f5760405162461bcd60e51b8152602060048201526012602482015271125b9d985b1a59081c995c5d595cdd081a5960721b6044820152606401610803565b609f54600003611f705760405162461bcd60e51b815260206004820152602660248201527f746865726520617265206e6f20726f6f747320796574206f6e2074686520636f6044820152651b9d1c9858dd60d21b6064820152608401610803565b609f54600090611f8290600190613b58565b90505b609d6000609f8381548110611f9c57611f9c613cdd565b90600052602060002001548152602001908152602001600020600001548310158015611ffb5750609d6000609f8381548110611fda57611fda613cdd565b90600052602060002001548152602001908152602001600020600101548311155b1561202657609f818154811061201357612013613cdd565b9060005260206000200154915050919050565b8061203081613f2e565b915050611f85565b606560009054906101000a90046001600160a01b03166001600160a01b031663eab66d7a6040518163ffffffff1660e01b8152600401602060405180830381865afa15801561208b573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906120af9190613bdd565b6001600160a01b0316336001600160a01b0316146120df5760405162461bcd60e51b815260040161080390613bfa565b60665419811960665419161461215d5760405162461bcd60e51b815260206004820152603860248201527f5061757361626c652e756e70617573653a20696e76616c696420617474656d7060448201527f7420746f2070617573652066756e6374696f6e616c69747900000000000000006064820152608401610803565b606681905560405181815233907f3582d1828e26bf56bd801502bc021ac0bc8afb57c826e4986b45593c8fad389c90602001611139565b600054610100900460ff16158080156121b45750600054600160ff909116105b806121ce5750303b1580156121ce575060005460ff166001145b6122315760405162461bcd60e51b815260206004820152602e60248201527f496e697469616c697a61626c653a20636f6e747261637420697320616c72656160448201526d191e481a5b9a5d1a585b1a5e995960921b6064820152608401610803565b6000805460ff191660011790558015612254576000805461ff0019166101001790555b61225f85600061300c565b61226884612cfc565b6000609881905560016097819055609991909155609a8054859260ff1990911690838181111561229a5761229a6136df565b0217905550609a8054610100600160a81b0319166101006001600160a01b038516021790558015612306576000805461ff0019169055604051600181527f7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb3847402498906020015b60405180910390a15b5050505050565b6000838152609d602090815260409182902082518084019093528054808452600190910154918301919091521580159061234a5750602081015115155b61238c5760405162461bcd60e51b8152602060048201526013602482015272155b9adb9bdddb881b595c9adb19481c9bdbdd606a1b6044820152606401610803565b6000858152609e60205260409020546001600160a01b031673111111111111111111111111111111111111111019016123fb5760405162461bcd60e51b8152602060048201526011602482015270105b1c9958591e481c1c9bd8d95cdcd959607a1b6044820152606401610803565b80516020820151101561245b5760405162461bcd60e51b815260206004820152602260248201527f496e76616c696420726571756573742072616e67652c20656e64203c207374616044820152611c9d60f21b6064820152608401610803565b805186108061246d5750806020015186115b156124ba5760405162461bcd60e51b815260206004820152601b60248201527f52657175657374206964206f757473696465206f662072616e676500000000006044820152606401610803565b8051602082015163ffffffff916124d091613b58565b6124db906001613e9a565b11156125195760405162461bcd60e51b815260206004820152600d60248201526c52616e676520746f6f2062696760981b6044820152606401610803565b8051602082015160009161252c91613b58565b612537906001613e9a565b82519091506000906125499089613b58565b9050856125598883888887611a7d565b146125965760405162461bcd60e51b815260206004820152600d60248201526c24b73b30b634b210383937b7b360991b6044820152606401610803565b5050505050505050565b600060016097546125b19190613b58565b606084013511156125c457506001612611565b60006125d8604085013560608601356114cb565b90506000816040516020016125ed9190613785565b60408051601f19818403018152919052805160209091012060808601351415925050505b6040805160c0810190915242906000908060808101808481526020016097600081548092919061264090613cae565b909155509052815260208781013581830152851515604080840191909152606090920185905282518101516000908152609b909152208151805182549394508493839190829060ff19166001838181111561269d5761269d6136df565b02179055506020918201516001919091015582810151600283015560408084015160038401805460ff1916911515919091179055606093840151600490930192909255838101518483015183519182521515918101919091529081018690527f9ef11353afd97d339a77732850b7c22704656558d9ba63cc7e321e0ac4c20aa991016122fd565b818111156127445760405162461bcd60e51b815260040161080390613b8d565b6001600160a01b0383166127925760405162461bcd60e51b8152602060048201526015602482015274496e76616c696420746f6b656e206164647265737360581b6044820152606401610803565b600082116127b25760405162461bcd60e51b815260040161080390613f45565b33836127c96001600160a01b03821683308761293a565b60408051610100810190915242906000908060c0810180848152602001609760008154809291906127f990613cae565b90915550905281526001600160a01b03808716602080840191909152908a16604080840191909152606083018a90526080830186905260a090920188905282518101516000908152609c909152208151805182549394508493839190829060ff19166001838181111561286e5761286e6136df565b021790555060209182015160019190910155828101516002830180546001600160a01b03199081166001600160a01b0393841617909155604080860151600386018054909316908416179091556060808601516004860155608080870151600587015560a096870151600690960195909555865184015182519081528a841694810194909452918c169083015281018990529081018790527f225305ecb66fa9b9b29f8dead9baea365a6c225d639dfd866e782ccb63e2f05b910160405180910390a150505050505050565b6040516001600160a01b03808516602483015283166044820152606481018290526129a59085906323b872dd60e01b906084015b60408051601f198184030181529190526020810180516001600160e01b03166001600160e01b0319909316929092179091526130f2565b50505050565b6001600160a01b038116612a395760405162461bcd60e51b815260206004820152604960248201527f5061757361626c652e5f73657450617573657252656769737472793a206e657760448201527f50617573657252656769737472792063616e6e6f7420626520746865207a65726064820152686f206164647265737360b81b608482015260a401610803565b606554604080516001600160a01b03928316815291831660208301527f6e9fcd539896fca60e8b0f01dd580233e48a6b0f7df013b89ba7f565869acdb6910160405180910390a1606580546001600160a01b0319166001600160a01b0392909216919091179055565b3031811115612af35760405162461bcd60e51b815260206004820152601c60248201527f4e6f7420656e6f7567682066756e647320696e20636f6e7472616374000000006044820152606401610803565b60008111612b135760405162461bcd60e51b815260040161080390613f45565b604080516001600160a01b0384168152602081018390527fe0495355c1e04c512584521854d222d239a4b782b39ac8a7e835a34f5ec7c1e1910160405180910390a1612b5f82826131c9565b5050565b6040516370a0823160e01b8152306004820152829082906001600160a01b038316906370a0823190602401602060405180830381865afa158015612bab573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190612bcf9190613bc4565b1015612c1d5760405162461bcd60e51b815260206004820152601c60248201527f4e6f7420656e6f7567682066756e647320696e20636f6e7472616374000000006044820152606401610803565b60008211612c3d5760405162461bcd60e51b815260040161080390613f45565b612c516001600160a01b03821685846132e2565b604080516001600160a01b038087168252851660208201529081018390527ee763f7778b8ceef7270c89b7d1df1008b0e482da39c43831417733af96fb0d906060015b60405180910390a150505050565b6033546001600160a01b031633146114c95760405162461bcd60e51b815260206004820181905260248201527f4f776e61626c653a2063616c6c6572206973206e6f7420746865206f776e65726044820152606401610803565b603380546001600160a01b038381166001600160a01b0319831681179093556040519116919082907f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e090600090a35050565b6040808301356000908152609c602052908120600281015490916001600160a01b0390911690612d8460808601606087016135f4565b6001600160a01b031614612da557612da260808501606086016135f4565b90505b60038201546001600160a01b031660001901612dce57612dc9818360040154612aa2565b612def565b60038201546004830154612def9183916001600160a01b0390911690612b63565b60408051602086810135825286830135908201529081018490527f13750c731f87c15242874ce74bf4649502cc8e7c829067ce846505acdb96289d90606001612c94565b34811115612e535760405162461bcd60e51b815260040161080390613b8d565b60003411612ea35760405162461bcd60e51b815260206004820181905260248201527f6d73672076616c7565206d7573742062652067726561746572207468617420306044820152606401610803565b6040805161010081019091523390349042906000908060c081018084815260200160976000815480929190612ed790613cae565b90915550905281526001600160a01b03861660208083019190915260016040808401829052606084018890526080840187905260a090930189905283518201516000908152609c9092529190208251805182549495508594929391928492839160ff1916908381811115612f4d57612f4d6136df565b0217905550602091820151600191820155838201516002840180546001600160a01b03199081166001600160a01b0393841617909155604080870151600387018054909316908416179091556060808701516004870155608080880151600588015560a09788015160069097019690965587518501518251908152928b16948301949094528101919091529081018690529081018790527f225305ecb66fa9b9b29f8dead9baea365a6c225d639dfd866e782ccb63e2f05b91016122fd565b6065546001600160a01b031615801561302d57506001600160a01b03821615155b6130af5760405162461bcd60e51b815260206004820152604760248201527f5061757361626c652e5f696e697469616c697a655061757365723a205f696e6960448201527f7469616c697a6550617573657228292063616e206f6e6c792062652063616c6c6064820152666564206f6e636560c81b608482015260a401610803565b606681905560405181815233907fab40a374bc51de372200a8bc981af8c9ecdc08dfdaef0bb6e09f88f3c616ef3d9060200160405180910390a2612b5f826129ab565b6000613147826040518060400160405280602081526020017f5361666545524332303a206c6f772d6c6576656c2063616c6c206661696c6564815250856001600160a01b03166133129092919063ffffffff16565b8051909150156131c457808060200190518101906131659190613c44565b6131c45760405162461bcd60e51b815260206004820152602a60248201527f5361666545524332303a204552433230206f7065726174696f6e20646964206e6044820152691bdd081cdd58d8d9595960b21b6064820152608401610803565b505050565b804710156132195760405162461bcd60e51b815260206004820152601d60248201527f416464726573733a20696e73756666696369656e742062616c616e63650000006044820152606401610803565b6000826001600160a01b03168260405160006040518083038185875af1925050503d8060008114613266576040519150601f19603f3d011682016040523d82523d6000602084013e61326b565b606091505b50509050806131c45760405162461bcd60e51b815260206004820152603a60248201527f416464726573733a20756e61626c6520746f2073656e642076616c75652c207260448201527f6563697069656e74206d617920686176652072657665727465640000000000006064820152608401610803565b6040516001600160a01b0383166024820152604481018290526131c490849063a9059cbb60e01b9060640161296e565b6060613321848460008561332b565b90505b9392505050565b60608247101561338c5760405162461bcd60e51b815260206004820152602660248201527f416464726573733a20696e73756666696369656e742062616c616e636520666f6044820152651c8818d85b1b60d21b6064820152608401610803565b6001600160a01b0385163b6133e35760405162461bcd60e51b815260206004820152601d60248201527f416464726573733a2063616c6c20746f206e6f6e2d636f6e74726163740000006044820152606401610803565b600080866001600160a01b031685876040516133ff9190613f7a565b60006040518083038185875af1925050503d806000811461343c576040519150601f19603f3d011682016040523d82523d6000602084013e613441565b606091505b5091509150611e458282866060831561345b575081613324565b82511561346b5782518084602001fd5b8160405162461bcd60e51b81526004016108039190613f96565b600060a0828403121561349757600080fd5b50919050565b60008083601f8401126134af57600080fd5b50813567ffffffffffffffff8111156134c757600080fd5b6020830191508360208260051b85010111156134e257600080fd5b9250929050565b60008060008060e085870312156134ff57600080fd5b6135098686613485565b935060a0850135925060c085013567ffffffffffffffff81111561352c57600080fd5b6135388782880161349d565b95989497509550505050565b6001600160a01b038116811461100257600080fd5b60008060006060848603121561356e57600080fd5b833561357981613544565b95602085013595506040909401359392505050565b60008082840360608112156135a257600080fd5b833592506040601f19820112156135b857600080fd5b506020830190509250929050565b600060c0828403121561349757600080fd5b600060c082840312156135ea57600080fd5b61332483836135c6565b60006020828403121561360657600080fd5b813561332481613544565b60006020828403121561362357600080fd5b5035919050565b6000806040838503121561363d57600080fd5b823561364881613544565b946020939093013593505050565b600080600080610100858703121561366d57600080fd5b61367786866135c6565b935060c0850135925060e085013567ffffffffffffffff81111561352c57600080fd5b6000602082840312156136ac57600080fd5b813560ff8116811461332457600080fd5b600080604083850312156136d057600080fd5b50508035926020909101359150565b634e487b7160e01b600052602160045260246000fd5b60028110611002576110026136df565b8051613710816136f5565b8252602090810151910152565b600081518084526020808501945080840160005b8381101561377a578151613746888251613705565b8084015160408981019190915281015115156060808a01919091520151608088015260a09096019590820190600101613731565b509495945050505050565b600060208083526080808401855161379c816136f5565b85840152858301516060604080880182905282519384905260a093928601928489019060005b8181101561381d5785516137d7848251613705565b808a01516001600160a01b03908116858701528582015116868501528581015189850152888101518885015287015160c08401529488019460e0909201916001016137c2565b505089820151898203601f1901848b0152965061383a818861371d565b9a9950505050505050505050565b60006080828403121561349757600080fd5b60008060008060c0858703121561387057600080fd5b61387a8686613848565b93506080850135925060a085013567ffffffffffffffff81111561352c57600080fd5b6000608082840312156138af57600080fd5b6133248383613848565b803563ffffffff811681146138cd57600080fd5b919050565b6000806000806000608086880312156138ea57600080fd5b853594506138fa602087016138b9565b9350604086013567ffffffffffffffff81111561391657600080fd5b6139228882890161349d565b90945092506139359050606087016138b9565b90509295509295909350565b60e0810161394f8289613705565b6001600160a01b039687166040830152949095166060860152608085019290925260a084015260c090920191909152919050565b60208101613990836136f5565b91905290565b60a081016139a48287613705565b60408201949094529115156060830152608090910152919050565b600060a082840312156139d157600080fd5b6133248383613485565b600080600080600080600060c0888a0312156139f657600080fd5b6139ff886138b9565b9650613a0d602089016138b9565b955060408801359450606088013567ffffffffffffffff811115613a3057600080fd5b613a3c8a828b0161349d565b9095509350613a4f9050608089016138b9565b9150613a5d60a089016138b9565b905092959891949750929550565b6002811061100257600080fd5b60008060008060808587031215613a8e57600080fd5b8435613a9981613544565b93506020850135613aa981613544565b92506040850135613ab981613a6b565b91506060850135613ac981613544565b939692955090935050565b6020808252601c908201527f5061757361626c653a20636f6e74726163742069732070617573656400000000604082015260600190565b6020808252601f908201527f5265656e7472616e637947756172643a207265656e7472616e742063616c6c00604082015260600190565b634e487b7160e01b600052601160045260246000fd5b600082821015613b6a57613b6a613b42565b500390565b82815260608101613324602083018480358252602090810135910152565b6020808252601c908201527f5469702065786365656473206465706f736974656420616d6f756e7400000000604082015260600190565b600060208284031215613bd657600080fd5b5051919050565b600060208284031215613bef57600080fd5b815161332481613544565b6020808252602a908201527f6d73672e73656e646572206973206e6f74207065726d697373696f6e6564206160408201526939903ab73830bab9b2b960b11b606082015260800190565b600060208284031215613c5657600080fd5b8151801515811461332457600080fd5b60208082526028908201527f6d73672e73656e646572206973206e6f74207065726d697373696f6e6564206160408201526739903830bab9b2b960c11b606082015260800190565b600060018201613cc057613cc0613b42565b5060010190565b634e487b7160e01b600052604160045260246000fd5b634e487b7160e01b600052603260045260246000fd5b6020810160038310613990576139906136df565b8035613d1281613a6b565b613d1b816136f5565b8252602090810135910152565b60c08101613d368284613d07565b6040830135613d4481613544565b6001600160a01b039081166040840152606084013590613d6382613544565b1660608301526080838101359083015260a092830135929091019190915290565b60005b83811015613d9f578181015183820152602001613d87565b838111156129a55750506000910152565b60008351613dc2818460208801613d84565b835190830190613dd6818360208801613d84565b01949350505050565b60808101613ded8284613d07565b604083013560408301526060830135613e0581613544565b6001600160a01b031660609290920191909152919050565b634e487b7160e01b600052601260045260246000fd5b600063ffffffff80841680613e4a57613e4a613e1d565b92169190910492915050565b600063ffffffff808316818516808303821115613dd657613dd6613b42565b600063ffffffff83811690831681811015613e9257613e92613b42565b039392505050565b60008219821115613ead57613ead613b42565b500190565b60a08101613ec08284613d07565b613eda604083016040850180358252602090810135910152565b608092830135919092015290565b600063ffffffff80841680613eff57613eff613e1d565b92169190910692915050565b600063ffffffff808316818103613f2457613f24613b42565b6001019392505050565b600081613f3d57613f3d613b42565b506000190190565b6020808252818101527f416d6f756e74206d7573742062652067726561746572207468616e207a65726f604082015260600190565b60008251613f8c818460208701613d84565b9190910192915050565b6020815260008251806020840152613fb5816040850160208701613d84565b601f01601f1916919091016040019291505056fea26469706673582212206ee6c29b88fb6599f02119957a22d76f2754fff62d2e3a32f4aad5ab9617116664736f6c634300080d0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R`\x046\x10a\x02gW`\x005`\xE0\x1C\x80c\x95\n\xC4\x87\x11a\x01DW\x80c\xD1eD\xF0\x11a\0\xB6W\x80c\xF2n\xE9\xD0\x11a\0zW\x80c\xF2n\xE9\xD0\x14a\x078W\x80c\xF2\xFD\xE3\x8B\x14a\x07NW\x80c\xF9\xEC\xD0\x1E\x14a\x07nW\x80c\xFA\xBC\x1C\xBC\x14a\x07\x8EW\x80c\xFF+\xAE\x86\x14a\x07\xAEW\x80c\xFF\xEAc+\x14a\x07\xC3W`\0\x80\xFD[\x80c\xD1eD\xF0\x14a\x03fW\x80c\xDEp\xE0\xB8\x14a\x06\xBAW\x80c\xDF.\xBD\xBB\x14a\x06\xF0W\x80c\xDF\xFB\xDD\x9F\x14a\x07\x05W\x80c\xEF\x0B\xA5\xD0\x14a\x07\x18W`\0\x80\xFD[\x80c\xB1S\x87\x06\x11a\x01\x08W\x80c\xB1S\x87\x06\x14a\x06\x06W\x80c\xC2\xB4\n\xE4\x14a\x06\x1BW\x80c\xC7c\xE5\xA1\x14a\x06;W\x80c\xC8|\"$\x14a\x06bW\x80c\xCA\x9B!\xAE\x14a\x06jW\x80c\xCC\x8C\x90\x9F\x14a\x06\x9AW`\0\x80\xFD[\x80c\x95\n\xC4\x87\x14a\x05TW\x80c\x9DT\xF4\x19\x14a\x05tW\x80c\xAEF\xDB\x11\x14a\x05\x94W\x80c\xAF&\xC6\x95\x14a\x05\xB4W\x80c\xB0,C\xD0\x14a\x05\xD4W`\0\x80\xFD[\x80cY\\jg\x11a\x01\xDDW\x80cq\xC5Da\x11a\x01\xA1W\x80cq\xC5Da\x14a\x04\x8EW\x80cy\xE0A\xF2\x14a\x04\xB3W\x80c\x7F\xD4\xF8E\x14a\x04\xE0W\x80c\x88o\x11\x95\x14a\x04\xF6W\x80c\x89\x0E\x95\xCE\x14a\x05\x16W\x80c\x8D\xA5\xCB[\x14a\x056W`\0\x80\xFD[\x80cY\\jg\x14a\x03\xEFW\x80cZ\xC8j\xB7\x14a\x04\x04W\x80c\\\x97Z\xBB\x14a\x04DW\x80ca\xBC\"\x1A\x14a\x04cW\x80cqP\x18\xA6\x14a\x04yW`\0\x80\xFD[\x80c\x0E\xFEj\x8B\x11a\x02/W\x80c\x0E\xFEj\x8B\x14a\x02\x8EW\x80c\x10\xD6z/\x14a\x03&W\x80c\x13d9\xDD\x14a\x03FW\x80cG\xE7\xEF$\x14a\x03fW\x80cK\xF5\xFE\xC3\x14a\x03\x86W\x80cOH\xEE\xDF\x14a\x03\xA6W`\0\x80\xFD[\x80c\x01\xEFif\x14a\x02lW\x80c\x08\xAB\xA1\xB2\x14a\x02\x8EW\x80c\x08\xF4-@\x14a\x02\xAEW\x80c\x0C\xACW\xAB\x14a\x02\xCEW\x80c\x0E&6\xA3\x14a\x02\xE1W[`\0\x80\xFD[4\x80\x15a\x02xW`\0\x80\xFD[Pa\x02\x8Ca\x02\x876`\x04a4\xE9V[a\x07\xE3V[\0[4\x80\x15a\x02\x9AW`\0\x80\xFD[Pa\x02\x8Ca\x02\xA96`\x04a5YV[a\x08\x98V[4\x80\x15a\x02\xBAW`\0\x80\xFD[Pa\x02\x8Ca\x02\xC96`\x04a5\x8EV[a\x08\xF4V[a\x02\x8Ca\x02\xDC6`\x04a5\xD8V[a\x0BGV[4\x80\x15a\x02\xEDW`\0\x80\xFD[Pa\x03\ts\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x032W`\0\x80\xFD[Pa\x02\x8Ca\x03A6`\x04a5\xF4V[a\x0FRV[4\x80\x15a\x03RW`\0\x80\xFD[Pa\x02\x8Ca\x03a6`\x04a6\x11V[a\x10\x05V[4\x80\x15a\x03rW`\0\x80\xFD[Pa\x02\x8Ca\x03\x816`\x04a6*V[a\x11DV[4\x80\x15a\x03\x92W`\0\x80\xFD[Pa\x02\x8Ca\x03\xA16`\x04a6VV[a\x11\xA0V[4\x80\x15a\x03\xB2W`\0\x80\xFD[Pa\x03\xDAa\x03\xC16`\x04a6\x11V[`\x9D` R`\0\x90\x81R`@\x90 \x80T`\x01\x90\x91\x01T\x82V[`@\x80Q\x92\x83R` \x83\x01\x91\x90\x91R\x01a\x03\x1DV[4\x80\x15a\x03\xFBW`\0\x80\xFD[Pa\x02\x8Ca\x13\xF0V[4\x80\x15a\x04\x10W`\0\x80\xFD[Pa\x044a\x04\x1F6`\x04a6\x9AV[`fT`\x01`\xFF\x90\x92\x16\x91\x90\x91\x1B\x90\x81\x16\x14\x90V[`@Q\x90\x15\x15\x81R` \x01a\x03\x1DV[4\x80\x15a\x04PW`\0\x80\xFD[P`fT[`@Q\x90\x81R` \x01a\x03\x1DV[4\x80\x15a\x04oW`\0\x80\xFD[Pa\x04U`\x97T\x81V[4\x80\x15a\x04\x85W`\0\x80\xFD[Pa\x02\x8Ca\x14\xB7V[4\x80\x15a\x04\x9AW`\0\x80\xFD[P`\x9ATa\x03\t\x90a\x01\0\x90\x04`\x01`\x01`\xA0\x1B\x03\x16\x81V[4\x80\x15a\x04\xBFW`\0\x80\xFD[Pa\x04\xD3a\x04\xCE6`\x04a6\xBDV[a\x14\xCBV[`@Qa\x03\x1D\x91\x90a7\x85V[4\x80\x15a\x04\xECW`\0\x80\xFD[Pa\x04U`\x98T\x81V[4\x80\x15a\x05\x02W`\0\x80\xFD[P`eTa\x03\t\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[4\x80\x15a\x05\"W`\0\x80\xFD[Pa\x04Ua\x0516`\x04a5\xD8V[a\x18\xE5V[4\x80\x15a\x05BW`\0\x80\xFD[P`3T`\x01`\x01`\xA0\x1B\x03\x16a\x03\tV[4\x80\x15a\x05`W`\0\x80\xFD[Pa\x02\x8Ca\x05o6`\x04a8ZV[a\x19SV[4\x80\x15a\x05\x80W`\0\x80\xFD[Pa\x02\x8Ca\x05\x8F6`\x04a5\xF4V[a\x19\xC2V[4\x80\x15a\x05\xA0W`\0\x80\xFD[Pa\x04Ua\x05\xAF6`\x04a8\x9DV[a\x1AIV[4\x80\x15a\x05\xC0W`\0\x80\xFD[Pa\x04Ua\x05\xCF6`\x04a8\xD2V[a\x1A}V[4\x80\x15a\x05\xE0W`\0\x80\xFD[Pa\x05\xF4a\x05\xEF6`\x04a6\x11V[a\x1A\xCEV[`@Qa\x03\x1D\x96\x95\x94\x93\x92\x91\x90a9AV[4\x80\x15a\x06\x12W`\0\x80\xFD[Pa\x04\xD3a\x1BUV[4\x80\x15a\x06'W`\0\x80\xFD[Pa\x04Ua\x0666`\x04a6\x11V[a\x1B\xA0V[4\x80\x15a\x06GW`\0\x80\xFD[P`\x9ATa\x06U\x90`\xFF\x16\x81V[`@Qa\x03\x1D\x91\x90a9\x83V[a\x02\x8Ca\x1B\xC1V[4\x80\x15a\x06vW`\0\x80\xFD[Pa\x06\x8Aa\x06\x856`\x04a6\x11V[a\x1C\x19V[`@Qa\x03\x1D\x94\x93\x92\x91\x90a9\x96V[4\x80\x15a\x06\xA6W`\0\x80\xFD[Pa\x04Ua\x06\xB56`\x04a9\xBFV[a\x1C\x8BV[4\x80\x15a\x06\xC6W`\0\x80\xFD[Pa\x03\ta\x06\xD56`\x04a6\x11V[`\x9E` R`\0\x90\x81R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x16\x81V[4\x80\x15a\x06\xFCW`\0\x80\xFD[Pa\x03\t`\x01\x81V[a\x02\x8Ca\x07\x136`\x04a6\x11V[a\x1C\xBFV[4\x80\x15a\x07$W`\0\x80\xFD[Pa\x04Ua\x0736`\x04a9\xDBV[a\x1D\x17V[4\x80\x15a\x07DW`\0\x80\xFD[Pa\x04U`\x99T\x81V[4\x80\x15a\x07ZW`\0\x80\xFD[Pa\x02\x8Ca\x07i6`\x04a5\xF4V[a\x1EPV[4\x80\x15a\x07zW`\0\x80\xFD[Pa\x04Ua\x07\x896`\x04a6\x11V[a\x1E\xC6V[4\x80\x15a\x07\x9AW`\0\x80\xFD[Pa\x02\x8Ca\x07\xA96`\x04a6\x11V[a 8V[4\x80\x15a\x07\xBAW`\0\x80\xFD[P`\x9FTa\x04UV[4\x80\x15a\x07\xCFW`\0\x80\xFD[Pa\x02\x8Ca\x07\xDE6`\x04a:xV[a!\x94V[`fT\x15a\x08\x0CW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08\x03\x90a:\xD4V[`@Q\x80\x91\x03\x90\xFD[`\x02`\xD2T\x03a\x08.W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08\x03\x90a;\x0BV[`\x02`\xD2U`\0a\x08>\x85a\x1C\x8BV[\x90Pa\x08Q` \x86\x015\x82\x86\x86\x86a#\rV[a\x08[\x85\x82a%\xA0V[`\0\x90\x81R`\x9E` R`@\x90 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x16s\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x17\x90UPP`\x01`\xD2UPPV[`fT\x15a\x08\xB8W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08\x03\x90a:\xD4V[`\x02`\xD2T\x03a\x08\xDAW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08\x03\x90a;\x0BV[`\x02`\xD2Ua\x08\xEA\x83\x83\x83a'$V[PP`\x01`\xD2UPV[`fT\x15a\t\x14W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08\x03\x90a:\xD4V[`\x9ATa\x01\0\x90\x04`\x01`\x01`\xA0\x1B\x03\x163\x14a\tcW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\r`$\x82\x01Rl'7\xBA\x10:42\x907\xBB\xB72\xB9`\x99\x1B`D\x82\x01R`d\x01a\x08\x03V[`\x99T\x81` \x015\x11a\t\xB8W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x19`$\x82\x01R\x7FUpdate brings no new data\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x08\x03V[\x805a\n\x06W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7Frange id must be greater than 0\0`D\x82\x01R`d\x01a\x08\x03V[`\x99Ta\n\x15`\x01\x835a;XV[\x11\x15a\ncW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x17`$\x82\x01R\x7FPrevious update missing\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x08\x03V[\x805` \x82\x015\x10\x15a\n\xA8W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\r`$\x82\x01RlInvalid range`\x98\x1B`D\x82\x01R`d\x01a\x08\x03V[`\x9F\x80T`\x01\x81\x01\x90\x91U\x7F\x0B\xC1@f\xC30\x13\xFE\x88\xF6n1NL\xF1P\xB0\xB2\xD4\xD6E\x1A\x1AQ\xDB\xBD\x1C'\xCD\x11\xDE(\x01\x82\x90U`\0\x82\x81R`\x9D` R`@\x90 \x81\x90a\n\xFF\x82\x82\x815\x81U` \x82\x015`\x01\x82\x01UPPV[PP` \x81\x015`\x99U`@Q\x7FI\xC1X\xD4\x90\xDB\x9E\x06o\x01\xB5\xD4\xF1\xA0\x94HZe\x98\xCBlR\x96\xB4\xC0~F\xC1*\x1D\xC1\x1C\x90a\x0B;\x90\x84\x90\x84\x90a;oV[`@Q\x80\x91\x03\x90\xA1PPV[`fT\x15a\x0BgW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08\x03\x90a:\xD4V[`\x02`\xD2T\x03a\x0B\x89W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08\x03\x90a;\x0BV[`\x02`\xD2U`\x80\x81\x015`\xA0\x82\x015\x11\x15a\x0B\xB6W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08\x03\x90a;\x8DV[`\0a\x0B\xCA`\xA0\x83\x015`\x80\x84\x015a;XV[\x90P`\0a\x0B\xD7\x83a\x18\xE5V[`\0\x81\x81R`\x9E` R`@\x90 T\x90\x91P`\x01`\x01`\xA0\x1B\x03\x16\x15a\x0C1W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0F`$\x82\x01Rn\x10[\x1C\x99XY\x1EH\x19\x99\\\x9C\x9AYY`\x8A\x1B`D\x82\x01R`d\x01a\x08\x03V[`\0\x81\x81R`\x9E` R`@\x90 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x163\x17\x90U`\x01a\x0Ca`\x80\x85\x01``\x86\x01a5\xF4V[`\x01`\x01`\xA0\x1B\x03\x16\x03a\r\xF6W`\x004\x11a\x0C\xB7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x15`$\x82\x01Rt\x13\x98]\x1A]\x99H\x1D\x1B\xDA\xD9[\x88\x1B\x9B\xDD\x08\x1C\xD9[\x9D`Z\x1B`D\x82\x01R`d\x01a\x08\x03V[\x814\x14a\r=W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`H`$\x82\x01R\x7FSent amount should exactly match`D\x82\x01R\x7F withdrawal.amount - withdrawal.`d\x82\x01Rg\x06fW''\x95F\x97`\xC4\x1B`\x84\x82\x01R`\xA4\x01a\x08\x03V[a\rM``\x84\x01`@\x85\x01a5\xF4V[`\x01`\x01`\xA0\x1B\x03\x16a\x08\xFC\x83\x90\x81\x15\x02\x90`@Q`\0`@Q\x80\x83\x03\x81\x85\x88\x88\xF1\x93PPPP\x15\x80\x15a\r\x85W=`\0\x80>=`\0\xFD[P\x7Fz\x9A\xBD\x9E\xB8k\xDB\xCA\x89\xCB\xA4\x06\x9Ac,7\xD9=\xB8.>\x14\xAD\x81\x19\xA3\xA7\x81(\x14\x85>` \x84\x015\x83a\r\xBD``\x87\x01`@\x88\x01a5\xF4V[`@\x80Q\x93\x84R` \x84\x01\x92\x90\x92R`\x01`\x01`\xA0\x1B\x03\x16\x90\x82\x01R3``\x82\x01R`\x80\x81\x01\x83\x90R`\xA0\x01`@Q\x80\x91\x03\x90\xA1a\x08\xEAV[`\0a\x0E\x08`\x80\x85\x01``\x86\x01a5\xF4V[`@Qcp\xA0\x821`\xE0\x1B\x81R3`\x04\x82\x01R\x90\x91P\x83\x90`\x01`\x01`\xA0\x1B\x03\x83\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0EQW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0Eu\x91\x90a;\xC4V[\x10\x15a\x0E\xB6W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x10`$\x82\x01RoNot enough funds`\x80\x1B`D\x82\x01R`d\x01a\x08\x03V[a\x0E\xDC3a\x0E\xCA``\x87\x01`@\x88\x01a5\xF4V[`\x01`\x01`\xA0\x1B\x03\x84\x16\x91\x90\x86a):V[\x7Fz\x9A\xBD\x9E\xB8k\xDB\xCA\x89\xCB\xA4\x06\x9Ac,7\xD9=\xB8.>\x14\xAD\x81\x19\xA3\xA7\x81(\x14\x85>` \x85\x015\x84a\x0F\x13``\x88\x01`@\x89\x01a5\xF4V[`@\x80Q\x93\x84R` \x84\x01\x92\x90\x92R`\x01`\x01`\xA0\x1B\x03\x16\x90\x82\x01R3``\x82\x01R`\x80\x81\x01\x84\x90R`\xA0\x01`@Q\x80\x91\x03\x90\xA1PPP`\x01`\xD2UPV[`e`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xEA\xB6mz`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0F\xA5W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0F\xC9\x91\x90a;\xDDV[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x0F\xF9W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08\x03\x90a;\xFAV[a\x10\x02\x81a)\xABV[PV[`eT`@Qc#}\xFBG`\xE1\x1B\x81R3`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cF\xFB\xF6\x8E\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x10MW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x10q\x91\x90a<DV[a\x10\x8DW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08\x03\x90a<fV[`fT\x81\x81\x16\x14a\x11\x06W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`8`$\x82\x01R\x7FPausable.pause: invalid attempt `D\x82\x01R\x7Fto unpause functionality\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x08\x03V[`f\x81\x90U`@Q\x81\x81R3\x90\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=\x90` \x01[`@Q\x80\x91\x03\x90\xA2PV[`fT\x15a\x11dW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08\x03\x90a:\xD4V[`\x02`\xD2T\x03a\x11\x86W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08\x03\x90a;\x0BV[`\x02`\xD2Ua\x11\x97\x82\x82`\0a'$V[PP`\x01`\xD2UV[`fT\x15a\x11\xC0W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08\x03\x90a:\xD4V[`\x02`\xD2T\x03a\x11\xE2W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08\x03\x90a;\x0BV[`\x02`\xD2U`\0a\x11\xF2\x85a\x18\xE5V[\x90Pa\x12\x05` \x86\x015\x82\x86\x86\x86a#\rV[`\0\x81\x81R`\x9E` R`@\x90 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x81\x16s\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x17\x90\x91U`\x01`\x01`\xA0\x1B\x03\x16\x80\x15\x15\x80a\x13TW`\x01a\x12\\`\x80\x89\x01``\x8A\x01a5\xF4V[`\x01`\x01`\xA0\x1B\x03\x16\x03a\x12\xB1Wa\x12\x94a\x12}``\x89\x01`@\x8A\x01a5\xF4V[a\x12\x8F`\xA0\x8A\x015`\x80\x8B\x015a;XV[a*\xA2V[`\xA0\x87\x015\x15a\x12\xACWa\x12\xAC3\x88`\xA0\x015a*\xA2V[a\x13\x13V[a\x12\xEBa\x12\xC4``\x89\x01`@\x8A\x01a5\xF4V[a\x12\xD4`\x80\x8A\x01``\x8B\x01a5\xF4V[a\x12\xE6`\xA0\x8B\x015`\x80\x8C\x015a;XV[a+cV[`\xA0\x87\x015\x15a\x13\x13Wa\x13\x133a\x13\t`\x80\x8A\x01``\x8B\x01a5\xF4V[\x89`\xA0\x015a+cV[`@\x80Q` \x89\x81\x015\x82R\x81\x01\x85\x90R\x7F\x93_&\xD9K\xE3\x19\x07\x08\n\xA7\x8B>n*\xC6\xD4\x8A\x07*\xF0\x96\xC2\x02h8\x86!\xBB\xC1\x17\x89\x91\x01`@Q\x80\x91\x03\x90\xA1a\x13\xE2V[`\x01a\x13f`\x80\x89\x01``\x8A\x01a5\xF4V[`\x01`\x01`\xA0\x1B\x03\x16\x03a\x13\x87Wa\x13\x82\x82\x88`\x80\x015a*\xA2V[a\x13\xA5V[a\x13\xA5\x82a\x13\x9B`\x80\x8A\x01``\x8B\x01a5\xF4V[\x89`\x80\x015a+cV[`@\x80Q` \x89\x81\x015\x82R\x81\x01\x85\x90R\x7F)\x96\xFDTl7\xD7L\x17\x04f\xEAj\xA4\xA3\x08\xE3\xCA-J\xA6\x89\xE6\xE9\xE3)\x94\xDBP9\xCC\x0E\x91\x01`@Q\x80\x91\x03\x90\xA1[PP`\x01`\xD2UPPPPPV[`eT`@Qc#}\xFBG`\xE1\x1B\x81R3`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cF\xFB\xF6\x8E\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x148W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x14\\\x91\x90a<DV[a\x14xW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08\x03\x90a<fV[`\0\x19`f\x81\x90U`@Q\x90\x81R3\x90\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=\x90` \x01`@Q\x80\x91\x03\x90\xA2V[a\x14\xBFa,\xA2V[a\x14\xC9`\0a,\xFCV[V[a\x14\xF0`@\x80Q``\x81\x01\x90\x91R\x80`\0\x81R` \x01``\x81R` \x01``\x81RP\x90V[a\x15\x15`@\x80Q``\x81\x01\x90\x91R\x80`\0\x81R` \x01``\x81R` \x01``\x81RP\x90V[`\x9AT\x81\x90`\xFF\x16`\x01\x81\x11\x15a\x15.Wa\x15.a6\xDFV[\x90\x81`\x01\x81\x11\x15a\x15AWa\x15Aa6\xDFV[\x90RP`\0\x80\x85\x15\x80\x15a\x15SWP\x84\x15[\x15a\x15cW\x82\x93PPPPa\x18\xDFV[\x85[\x85\x81\x11a\x16\x08W`\0\x81\x81R`\x9C` R`@\x90 `\x01\x01T\x15a\x15\x95W\x82a\x15\x8D\x81a<\xAEV[\x93PPa\x15\xF6V[`\0\x81\x81R`\x9B` R`@\x90 `\x01\x01T\x15a\x15\xBEW\x81a\x15\xB6\x81a<\xAEV[\x92PPa\x15\xF6V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\r`$\x82\x01RlInvalid range`\x98\x1B`D\x82\x01R`d\x01a\x08\x03V[\x80a\x16\0\x81a<\xAEV[\x91PPa\x15eV[P\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x16\"Wa\x16\"a<\xC7V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x16\x90W\x81` \x01[`@\x80Qa\x01\0\x81\x01\x82R`\0`\xC0\x82\x01\x81\x81R`\xE0\x83\x01\x82\x90R\x82R` \x80\x83\x01\x82\x90R\x92\x82\x01\x81\x90R``\x82\x01\x81\x90R`\x80\x82\x01\x81\x90R`\xA0\x82\x01R\x82R`\0\x19\x90\x92\x01\x91\x01\x81a\x16@W\x90P[P` \x84\x01R\x80g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x16\xAFWa\x16\xAFa<\xC7V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x17\x0EW\x81` \x01[`@\x80Q`\xC0\x81\x01\x82R`\0`\x80\x82\x01\x81\x81R`\xA0\x83\x01\x82\x90R\x82R` \x80\x83\x01\x82\x90R\x92\x82\x01\x81\x90R``\x82\x01R\x82R`\0\x19\x90\x92\x01\x91\x01\x81a\x16\xCDW\x90P[P`@\x84\x01RP`\0\x90P\x80\x85[\x85\x81\x11a\x18\xD8W`\0\x81\x81R`\x9C` R`@\x90 `\x01\x01T\x15a\x18\x06W`\0\x81\x81R`\x9C` R`@\x90\x81\x90 \x81Qa\x01\0\x81\x01\x90\x92R\x80T\x82\x90`\xC0\x82\x01\x90\x83\x90\x82\x90`\xFF\x16`\x01\x81\x11\x15a\x17uWa\x17ua6\xDFV[`\x01\x81\x11\x15a\x17\x86Wa\x17\x86a6\xDFV[\x81R`\x01\x91\x90\x91\x01T` \x91\x82\x01R\x90\x82R`\x02\x83\x01T`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x83\x83\x01R`\x03\x84\x01T\x16`@\x83\x01R`\x04\x83\x01T``\x83\x01R`\x05\x83\x01T`\x80\x83\x01R`\x06\x90\x92\x01T`\xA0\x90\x91\x01R\x85\x01Q\x84a\x17\xE4\x81a<\xAEV[\x95P\x81Q\x81\x10a\x17\xF6Wa\x17\xF6a<\xDDV[` \x02` \x01\x01\x81\x90RPa\x18\xC6V[`\0\x81\x81R`\x9B` R`@\x90 `\x02\x01T\x15a\x18\xC1W`\0\x81\x81R`\x9B` R`@\x90\x81\x90 \x81Q`\xC0\x81\x01\x90\x92R\x80T\x82\x90`\x80\x82\x01\x90\x83\x90\x82\x90`\xFF\x16`\x01\x81\x11\x15a\x18WWa\x18Wa6\xDFV[`\x01\x81\x11\x15a\x18hWa\x18ha6\xDFV[\x81R`\x01\x91\x90\x91\x01T` \x91\x82\x01R\x90\x82R`\x02\x83\x01T\x90\x82\x01R`\x03\x82\x01T`\xFF\x16\x15\x15`@\x80\x83\x01\x91\x90\x91R`\x04\x90\x92\x01T``\x90\x91\x01R\x85\x01Q\x83a\x18\xAF\x81a<\xAEV[\x94P\x81Q\x81\x10a\x17\xF6Wa\x17\xF6a<\xDDV[a\x18\xD8V[\x80a\x18\xD0\x81a<\xAEV[\x91PPa\x17\x1CV[P\x91\x92PPP[\x92\x91PPV[`\0\x80`@Q` \x01a\x18\xF8\x91\x90a<\xF3V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x82`@Q` \x01a\x19\x18\x91\x90a=(V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra\x196\x92\x91` \x01a=\xB0V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x91\x90PV[`fT\x15a\x19sW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08\x03\x90a:\xD4V[`\x02`\xD2T\x03a\x19\x95W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08\x03\x90a;\x0BV[`\x02`\xD2U`\0a\x19\xA5\x85a\x1AIV[\x90Pa\x19\xB8` \x86\x015\x82\x86\x86\x86a#\rV[a\x08[\x85\x82a-NV[a\x19\xCAa,\xA2V[`fT\x15a\x19\xEAW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08\x03\x90a:\xD4V[`\x9A\x80Ta\x01\0`\x01`\xA8\x1B\x03\x19\x16a\x01\0`\x01`\x01`\xA0\x1B\x03\x84\x81\x16\x82\x02\x92\x90\x92\x17\x92\x83\x90U`@Q\x92\x04\x16\x81R\x7F\x1B\x0F/P\r\xF5\x96\xB4+s\xE8\r\xBE\xC6\xA1\xFBW\x0F\x01\x97\x8AXg#\xF9\x88\xA5\xFCT\xD7s\xA1\x90` \x01`@Q\x80\x91\x03\x90\xA1PV[`\0`\x02`@Q` \x01a\x1A]\x91\x90a<\xF3V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x82`@Q` \x01a\x19\x18\x91\x90a=\xDFV[`\0\x80\x82[c\xFF\xFF\xFF\xFF\x81\x16\x15a\x1A\xADWa\x1A\x99`\x02\x82a>3V[\x90Pa\x1A\xA6`\x01\x83a>VV[\x91Pa\x1A\x82V[a\x1A\xC2\x82\x88\x8A\x89\x89`\0a\x073`\x01\x8Ca>uV[\x98\x97PPPPPPPPV[`\x9C` R`\0\x90\x81R`@\x90\x81\x90 \x81Q\x80\x83\x01\x90\x92R\x80T\x90\x91\x90\x82\x90\x82\x90`\xFF\x16`\x01\x81\x11\x15a\x1B\x03Wa\x1B\x03a6\xDFV[`\x01\x81\x11\x15a\x1B\x14Wa\x1B\x14a6\xDFV[\x81R`\x01\x91\x90\x91\x01T` \x90\x91\x01R`\x02\x82\x01T`\x03\x83\x01T`\x04\x84\x01T`\x05\x85\x01T`\x06\x90\x95\x01T\x93\x94`\x01`\x01`\xA0\x1B\x03\x93\x84\x16\x94\x92\x90\x93\x16\x92\x90\x91\x86V[a\x1Bz`@\x80Q``\x81\x01\x90\x91R\x80`\0\x81R` \x01``\x81R` \x01``\x81RP\x90V[a\x1B\x9B`\x98T`\x01a\x1B\x8C\x91\x90a>\x9AV[`\x01`\x97Ta\x04\xCE\x91\x90a;XV[\x90P\x90V[`\x9F\x81\x81T\x81\x10a\x1B\xB0W`\0\x80\xFD[`\0\x91\x82R` \x90\x91 \x01T\x90P\x81V[`\x02`\xD2T\x03a\x1B\xE3W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08\x03\x90a;\x0BV[`\x02`\xD2U`fT\x15a\x1C\x08W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08\x03\x90a:\xD4V[a\x1C\x12`\0a.3V[`\x01`\xD2UV[`\x9B` R`\0\x90\x81R`@\x90\x81\x90 \x81Q\x80\x83\x01\x90\x92R\x80T\x90\x91\x90\x82\x90\x82\x90`\xFF\x16`\x01\x81\x11\x15a\x1CNWa\x1CNa6\xDFV[`\x01\x81\x11\x15a\x1C_Wa\x1C_a6\xDFV[\x81R`\x01\x91\x90\x91\x01T` \x90\x91\x01R`\x02\x82\x01T`\x03\x83\x01T`\x04\x90\x93\x01T\x91\x92\x90\x91`\xFF\x90\x91\x16\x90\x84V[`\0`\x01`@Q` \x01a\x1C\x9F\x91\x90a<\xF3V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x82`@Q` \x01a\x19\x18\x91\x90a>\xB2V[`\x02`\xD2T\x03a\x1C\xE1W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08\x03\x90a;\x0BV[`\x02`\xD2U`fT\x15a\x1D\x06W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08\x03\x90a:\xD4V[a\x1D\x0F\x81a.3V[P`\x01`\xD2UV[`\0a\x1D$`\x02\x88a>\xE8V[c\xFF\xFF\xFF\xFF\x16`\0\x03a\x1D\xA7W\x81c\xFF\xFF\xFF\xFF\x16\x87c\xFF\xFF\xFF\xFF\x16\x03\x15a\x1E\x05W\x85\x85\x85\x85a\x1DR\x81a?\x0BV[\x96Pc\xFF\xFF\xFF\xFF\x16\x81\x81\x10a\x1DiWa\x1Dia<\xDDV[\x90P` \x02\x015`@Q` \x01a\x1D\x8A\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x95Pa\x1E\x05V[\x84\x84\x84a\x1D\xB3\x81a?\x0BV[\x95Pc\xFF\xFF\xFF\xFF\x16\x81\x81\x10a\x1D\xCAWa\x1D\xCAa<\xDDV[\x90P` \x02\x015\x86`@Q` \x01a\x1D\xEC\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x95P[\x87c\xFF\xFF\xFF\xFF\x16`\x01\x03a\x1E\x1AWP\x84a\x1EEV[a\x1EBa\x1E(`\x01\x8Aa>uV[a\x1E3`\x02\x8Aa>3V[\x88\x88\x88\x88a\x073`\x02\x8Aa>3V[\x90P[\x97\x96PPPPPPPV[a\x1EXa,\xA2V[`\x01`\x01`\xA0\x1B\x03\x81\x16a\x1E\xBDW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01Reddress`\xD0\x1B`d\x82\x01R`\x84\x01a\x08\x03V[a\x10\x02\x81a,\xFCV[`\0`\x99T\x82\x11\x15a\x1F\x0FW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x12`$\x82\x01Rq\x12[\x9D\x98[\x1AY\x08\x1C\x99\\]Y\\\xDD\x08\x1AY`r\x1B`D\x82\x01R`d\x01a\x08\x03V[`\x9FT`\0\x03a\x1FpW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7Fthere are no roots yet on the co`D\x82\x01Re\x1B\x9D\x1C\x98X\xDD`\xD2\x1B`d\x82\x01R`\x84\x01a\x08\x03V[`\x9FT`\0\x90a\x1F\x82\x90`\x01\x90a;XV[\x90P[`\x9D`\0`\x9F\x83\x81T\x81\x10a\x1F\x9CWa\x1F\x9Ca<\xDDV[\x90`\0R` `\0 \x01T\x81R` \x01\x90\x81R` \x01`\0 `\0\x01T\x83\x10\x15\x80\x15a\x1F\xFBWP`\x9D`\0`\x9F\x83\x81T\x81\x10a\x1F\xDAWa\x1F\xDAa<\xDDV[\x90`\0R` `\0 \x01T\x81R` \x01\x90\x81R` \x01`\0 `\x01\x01T\x83\x11\x15[\x15a &W`\x9F\x81\x81T\x81\x10a \x13Wa \x13a<\xDDV[\x90`\0R` `\0 \x01T\x91PP\x91\x90PV[\x80a 0\x81a?.V[\x91PPa\x1F\x85V[`e`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xEA\xB6mz`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a \x8BW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a \xAF\x91\x90a;\xDDV[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a \xDFW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08\x03\x90a;\xFAV[`fT\x19\x81\x19`fT\x19\x16\x14a!]W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`8`$\x82\x01R\x7FPausable.unpause: invalid attemp`D\x82\x01R\x7Ft to pause functionality\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x08\x03V[`f\x81\x90U`@Q\x81\x81R3\x90\x7F5\x82\xD1\x82\x8E&\xBFV\xBD\x80\x15\x02\xBC\x02\x1A\xC0\xBC\x8A\xFBW\xC8&\xE4\x98kEY<\x8F\xAD8\x9C\x90` \x01a\x119V[`\0Ta\x01\0\x90\x04`\xFF\x16\x15\x80\x80\x15a!\xB4WP`\0T`\x01`\xFF\x90\x91\x16\x10[\x80a!\xCEWP0;\x15\x80\x15a!\xCEWP`\0T`\xFF\x16`\x01\x14[a\"1W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01Rm\x19\x1EH\x1A[\x9A]\x1AX[\x1A^\x99Y`\x92\x1B`d\x82\x01R`\x84\x01a\x08\x03V[`\0\x80T`\xFF\x19\x16`\x01\x17\x90U\x80\x15a\"TW`\0\x80Ta\xFF\0\x19\x16a\x01\0\x17\x90U[a\"_\x85`\0a0\x0CV[a\"h\x84a,\xFCV[`\0`\x98\x81\x90U`\x01`\x97\x81\x90U`\x99\x91\x90\x91U`\x9A\x80T\x85\x92`\xFF\x19\x90\x91\x16\x90\x83\x81\x81\x11\x15a\"\x9AWa\"\x9Aa6\xDFV[\x02\x17\x90UP`\x9A\x80Ta\x01\0`\x01`\xA8\x1B\x03\x19\x16a\x01\0`\x01`\x01`\xA0\x1B\x03\x85\x16\x02\x17\x90U\x80\x15a#\x06W`\0\x80Ta\xFF\0\x19\x16\x90U`@Q`\x01\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01[`@Q\x80\x91\x03\x90\xA1[PPPPPV[`\0\x83\x81R`\x9D` \x90\x81R`@\x91\x82\x90 \x82Q\x80\x84\x01\x90\x93R\x80T\x80\x84R`\x01\x90\x91\x01T\x91\x83\x01\x91\x90\x91R\x15\x80\x15\x90a#JWP` \x81\x01Q\x15\x15[a#\x8CW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x13`$\x82\x01Rr\x15[\x9A\xDB\x9B\xDD\xDB\x88\x1BY\\\x9A\xDB\x19H\x1C\x9B\xDB\xDD`j\x1B`D\x82\x01R`d\x01a\x08\x03V[`\0\x85\x81R`\x9E` R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x16s\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x10\x19\x01a#\xFBW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x11`$\x82\x01Rp\x10[\x1C\x99XY\x1EH\x1C\x1C\x9B\xD8\xD9\\\xDC\xD9Y`z\x1B`D\x82\x01R`d\x01a\x08\x03V[\x80Q` \x82\x01Q\x10\x15a$[W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FInvalid request range, end < sta`D\x82\x01Ra\x1C\x9D`\xF2\x1B`d\x82\x01R`\x84\x01a\x08\x03V[\x80Q\x86\x10\x80a$mWP\x80` \x01Q\x86\x11[\x15a$\xBAW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1B`$\x82\x01R\x7FRequest id outside of range\0\0\0\0\0`D\x82\x01R`d\x01a\x08\x03V[\x80Q` \x82\x01Qc\xFF\xFF\xFF\xFF\x91a$\xD0\x91a;XV[a$\xDB\x90`\x01a>\x9AV[\x11\x15a%\x19W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\r`$\x82\x01RlRange too big`\x98\x1B`D\x82\x01R`d\x01a\x08\x03V[\x80Q` \x82\x01Q`\0\x91a%,\x91a;XV[a%7\x90`\x01a>\x9AV[\x82Q\x90\x91P`\0\x90a%I\x90\x89a;XV[\x90P\x85a%Y\x88\x83\x88\x88\x87a\x1A}V[\x14a%\x96W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\r`$\x82\x01Rl$\xB7;0\xB64\xB2\x10897\xB7\xB3`\x99\x1B`D\x82\x01R`d\x01a\x08\x03V[PPPPPPPPV[`\0`\x01`\x97Ta%\xB1\x91\x90a;XV[``\x84\x015\x11\x15a%\xC4WP`\x01a&\x11V[`\0a%\xD8`@\x85\x015``\x86\x015a\x14\xCBV[\x90P`\0\x81`@Q` \x01a%\xED\x91\x90a7\x85V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R\x80Q` \x90\x91\x01 `\x80\x86\x015\x14\x15\x92PPP[`@\x80Q`\xC0\x81\x01\x90\x91RB\x90`\0\x90\x80`\x80\x81\x01\x80\x84\x81R` \x01`\x97`\0\x81T\x80\x92\x91\x90a&@\x90a<\xAEV[\x90\x91UP\x90R\x81R` \x87\x81\x015\x81\x83\x01R\x85\x15\x15`@\x80\x84\x01\x91\x90\x91R``\x90\x92\x01\x85\x90R\x82Q\x81\x01Q`\0\x90\x81R`\x9B\x90\x91R \x81Q\x80Q\x82T\x93\x94P\x84\x93\x83\x91\x90\x82\x90`\xFF\x19\x16`\x01\x83\x81\x81\x11\x15a&\x9DWa&\x9Da6\xDFV[\x02\x17\x90UP` \x91\x82\x01Q`\x01\x91\x90\x91\x01U\x82\x81\x01Q`\x02\x83\x01U`@\x80\x84\x01Q`\x03\x84\x01\x80T`\xFF\x19\x16\x91\x15\x15\x91\x90\x91\x17\x90U``\x93\x84\x01Q`\x04\x90\x93\x01\x92\x90\x92U\x83\x81\x01Q\x84\x83\x01Q\x83Q\x91\x82R\x15\x15\x91\x81\x01\x91\x90\x91R\x90\x81\x01\x86\x90R\x7F\x9E\xF1\x13S\xAF\xD9}3\x9Aws(P\xB7\xC2'\x04eeX\xD9\xBAc\xCC~2\x1E\n\xC4\xC2\n\xA9\x91\x01a\"\xFDV[\x81\x81\x11\x15a'DW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08\x03\x90a;\x8DV[`\x01`\x01`\xA0\x1B\x03\x83\x16a'\x92W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x15`$\x82\x01RtInvalid token address`X\x1B`D\x82\x01R`d\x01a\x08\x03V[`\0\x82\x11a'\xB2W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08\x03\x90a?EV[3\x83a'\xC9`\x01`\x01`\xA0\x1B\x03\x82\x16\x830\x87a):V[`@\x80Qa\x01\0\x81\x01\x90\x91RB\x90`\0\x90\x80`\xC0\x81\x01\x80\x84\x81R` \x01`\x97`\0\x81T\x80\x92\x91\x90a'\xF9\x90a<\xAEV[\x90\x91UP\x90R\x81R`\x01`\x01`\xA0\x1B\x03\x80\x87\x16` \x80\x84\x01\x91\x90\x91R\x90\x8A\x16`@\x80\x84\x01\x91\x90\x91R``\x83\x01\x8A\x90R`\x80\x83\x01\x86\x90R`\xA0\x90\x92\x01\x88\x90R\x82Q\x81\x01Q`\0\x90\x81R`\x9C\x90\x91R \x81Q\x80Q\x82T\x93\x94P\x84\x93\x83\x91\x90\x82\x90`\xFF\x19\x16`\x01\x83\x81\x81\x11\x15a(nWa(na6\xDFV[\x02\x17\x90UP` \x91\x82\x01Q`\x01\x91\x90\x91\x01U\x82\x81\x01Q`\x02\x83\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x90\x81\x16`\x01`\x01`\xA0\x1B\x03\x93\x84\x16\x17\x90\x91U`@\x80\x86\x01Q`\x03\x86\x01\x80T\x90\x93\x16\x90\x84\x16\x17\x90\x91U``\x80\x86\x01Q`\x04\x86\x01U`\x80\x80\x87\x01Q`\x05\x87\x01U`\xA0\x96\x87\x01Q`\x06\x90\x96\x01\x95\x90\x95U\x86Q\x84\x01Q\x82Q\x90\x81R\x8A\x84\x16\x94\x81\x01\x94\x90\x94R\x91\x8C\x16\x90\x83\x01R\x81\x01\x89\x90R\x90\x81\x01\x87\x90R\x7F\"S\x05\xEC\xB6o\xA9\xB9\xB2\x9F\x8D\xEA\xD9\xBA\xEA6Zl\"]c\x9D\xFD\x86nx,\xCBc\xE2\xF0[\x91\x01`@Q\x80\x91\x03\x90\xA1PPPPPPPV[`@Q`\x01`\x01`\xA0\x1B\x03\x80\x85\x16`$\x83\x01R\x83\x16`D\x82\x01R`d\x81\x01\x82\x90Ra)\xA5\x90\x85\x90c#\xB8r\xDD`\xE0\x1B\x90`\x84\x01[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16`\x01`\x01`\xE0\x1B\x03\x19\x90\x93\x16\x92\x90\x92\x17\x90\x91Ra0\xF2V[PPPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16a*9W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`I`$\x82\x01R\x7FPausable._setPauserRegistry: new`D\x82\x01R\x7FPauserRegistry cannot be the zer`d\x82\x01Rho address`\xB8\x1B`\x84\x82\x01R`\xA4\x01a\x08\x03V[`eT`@\x80Q`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x81R\x91\x83\x16` \x83\x01R\x7Fn\x9F\xCDS\x98\x96\xFC\xA6\x0E\x8B\x0F\x01\xDDX\x023\xE4\x8Ak\x0F}\xF0\x13\xB8\x9B\xA7\xF5e\x86\x9A\xCD\xB6\x91\x01`@Q\x80\x91\x03\x90\xA1`e\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[01\x81\x11\x15a*\xF3W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1C`$\x82\x01R\x7FNot enough funds in contract\0\0\0\0`D\x82\x01R`d\x01a\x08\x03V[`\0\x81\x11a+\x13W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08\x03\x90a?EV[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x84\x16\x81R` \x81\x01\x83\x90R\x7F\xE0ISU\xC1\xE0LQ%\x84R\x18T\xD2\"\xD29\xA4\xB7\x82\xB3\x9A\xC8\xA7\xE85\xA3O^\xC7\xC1\xE1\x91\x01`@Q\x80\x91\x03\x90\xA1a+_\x82\x82a1\xC9V[PPV[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R\x82\x90\x82\x90`\x01`\x01`\xA0\x1B\x03\x83\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a+\xABW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a+\xCF\x91\x90a;\xC4V[\x10\x15a,\x1DW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1C`$\x82\x01R\x7FNot enough funds in contract\0\0\0\0`D\x82\x01R`d\x01a\x08\x03V[`\0\x82\x11a,=W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08\x03\x90a?EV[a,Q`\x01`\x01`\xA0\x1B\x03\x82\x16\x85\x84a2\xE2V[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x80\x87\x16\x82R\x85\x16` \x82\x01R\x90\x81\x01\x83\x90R~\xE7c\xF7w\x8B\x8C\xEE\xF7'\x0C\x89\xB7\xD1\xDF\x10\x08\xB0\xE4\x82\xDA9\xC481Aw3\xAF\x96\xFB\r\x90``\x01[`@Q\x80\x91\x03\x90\xA1PPPPV[`3T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x14\xC9W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\x08\x03V[`3\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x90\x93U`@Q\x91\x16\x91\x90\x82\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90`\0\x90\xA3PPV[`@\x80\x83\x015`\0\x90\x81R`\x9C` R\x90\x81 `\x02\x81\x01T\x90\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90a-\x84`\x80\x86\x01``\x87\x01a5\xF4V[`\x01`\x01`\xA0\x1B\x03\x16\x14a-\xA5Wa-\xA2`\x80\x85\x01``\x86\x01a5\xF4V[\x90P[`\x03\x82\x01T`\x01`\x01`\xA0\x1B\x03\x16`\0\x19\x01a-\xCEWa-\xC9\x81\x83`\x04\x01Ta*\xA2V[a-\xEFV[`\x03\x82\x01T`\x04\x83\x01Ta-\xEF\x91\x83\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90a+cV[`@\x80Q` \x86\x81\x015\x82R\x86\x83\x015\x90\x82\x01R\x90\x81\x01\x84\x90R\x7F\x13u\x0Cs\x1F\x87\xC1RB\x87L\xE7K\xF4d\x95\x02\xCC\x8E|\x82\x90g\xCE\x84e\x05\xAC\xDB\x96(\x9D\x90``\x01a,\x94V[4\x81\x11\x15a.SW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08\x03\x90a;\x8DV[`\x004\x11a.\xA3W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7Fmsg value must be greater that 0`D\x82\x01R`d\x01a\x08\x03V[`@\x80Qa\x01\0\x81\x01\x90\x91R3\x904\x90B\x90`\0\x90\x80`\xC0\x81\x01\x80\x84\x81R` \x01`\x97`\0\x81T\x80\x92\x91\x90a.\xD7\x90a<\xAEV[\x90\x91UP\x90R\x81R`\x01`\x01`\xA0\x1B\x03\x86\x16` \x80\x83\x01\x91\x90\x91R`\x01`@\x80\x84\x01\x82\x90R``\x84\x01\x88\x90R`\x80\x84\x01\x87\x90R`\xA0\x90\x93\x01\x89\x90R\x83Q\x82\x01Q`\0\x90\x81R`\x9C\x90\x92R\x91\x90 \x82Q\x80Q\x82T\x94\x95P\x85\x94\x92\x93\x91\x92\x84\x92\x83\x91`\xFF\x19\x16\x90\x83\x81\x81\x11\x15a/MWa/Ma6\xDFV[\x02\x17\x90UP` \x91\x82\x01Q`\x01\x91\x82\x01U\x83\x82\x01Q`\x02\x84\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x90\x81\x16`\x01`\x01`\xA0\x1B\x03\x93\x84\x16\x17\x90\x91U`@\x80\x87\x01Q`\x03\x87\x01\x80T\x90\x93\x16\x90\x84\x16\x17\x90\x91U``\x80\x87\x01Q`\x04\x87\x01U`\x80\x80\x88\x01Q`\x05\x88\x01U`\xA0\x97\x88\x01Q`\x06\x90\x97\x01\x96\x90\x96U\x87Q\x85\x01Q\x82Q\x90\x81R\x92\x8B\x16\x94\x83\x01\x94\x90\x94R\x81\x01\x91\x90\x91R\x90\x81\x01\x86\x90R\x90\x81\x01\x87\x90R\x7F\"S\x05\xEC\xB6o\xA9\xB9\xB2\x9F\x8D\xEA\xD9\xBA\xEA6Zl\"]c\x9D\xFD\x86nx,\xCBc\xE2\xF0[\x91\x01a\"\xFDV[`eT`\x01`\x01`\xA0\x1B\x03\x16\x15\x80\x15a0-WP`\x01`\x01`\xA0\x1B\x03\x82\x16\x15\x15[a0\xAFW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`G`$\x82\x01R\x7FPausable._initializePauser: _ini`D\x82\x01R\x7FtializePauser() can only be call`d\x82\x01Rfed once`\xC8\x1B`\x84\x82\x01R`\xA4\x01a\x08\x03V[`f\x81\x90U`@Q\x81\x81R3\x90\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=\x90` \x01`@Q\x80\x91\x03\x90\xA2a+_\x82a)\xABV[`\0a1G\x82`@Q\x80`@\x01`@R\x80` \x81R` \x01\x7FSafeERC20: low-level call failed\x81RP\x85`\x01`\x01`\xA0\x1B\x03\x16a3\x12\x90\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x80Q\x90\x91P\x15a1\xC4W\x80\x80` \x01\x90Q\x81\x01\x90a1e\x91\x90a<DV[a1\xC4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`*`$\x82\x01R\x7FSafeERC20: ERC20 operation did n`D\x82\x01Ri\x1B\xDD\x08\x1C\xDDX\xD8\xD9YY`\xB2\x1B`d\x82\x01R`\x84\x01a\x08\x03V[PPPV[\x80G\x10\x15a2\x19W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FAddress: insufficient balance\0\0\0`D\x82\x01R`d\x01a\x08\x03V[`\0\x82`\x01`\x01`\xA0\x1B\x03\x16\x82`@Q`\0`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80`\0\x81\x14a2fW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a2kV[``\x91P[PP\x90P\x80a1\xC4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`:`$\x82\x01R\x7FAddress: unable to send value, r`D\x82\x01R\x7Fecipient may have reverted\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x08\x03V[`@Q`\x01`\x01`\xA0\x1B\x03\x83\x16`$\x82\x01R`D\x81\x01\x82\x90Ra1\xC4\x90\x84\x90c\xA9\x05\x9C\xBB`\xE0\x1B\x90`d\x01a)nV[``a3!\x84\x84`\0\x85a3+V[\x90P[\x93\x92PPPV[``\x82G\x10\x15a3\x8CW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FAddress: insufficient balance fo`D\x82\x01Re\x1C\x88\x18\xD8[\x1B`\xD2\x1B`d\x82\x01R`\x84\x01a\x08\x03V[`\x01`\x01`\xA0\x1B\x03\x85\x16;a3\xE3W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FAddress: call to non-contract\0\0\0`D\x82\x01R`d\x01a\x08\x03V[`\0\x80\x86`\x01`\x01`\xA0\x1B\x03\x16\x85\x87`@Qa3\xFF\x91\x90a?zV[`\0`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80`\0\x81\x14a4<W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a4AV[``\x91P[P\x91P\x91Pa\x1EE\x82\x82\x86``\x83\x15a4[WP\x81a3$V[\x82Q\x15a4kW\x82Q\x80\x84` \x01\xFD[\x81`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08\x03\x91\x90a?\x96V[`\0`\xA0\x82\x84\x03\x12\x15a4\x97W`\0\x80\xFD[P\x91\x90PV[`\0\x80\x83`\x1F\x84\x01\x12a4\xAFW`\0\x80\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a4\xC7W`\0\x80\xFD[` \x83\x01\x91P\x83` \x82`\x05\x1B\x85\x01\x01\x11\x15a4\xE2W`\0\x80\xFD[\x92P\x92\x90PV[`\0\x80`\0\x80`\xE0\x85\x87\x03\x12\x15a4\xFFW`\0\x80\xFD[a5\t\x86\x86a4\x85V[\x93P`\xA0\x85\x015\x92P`\xC0\x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a5,W`\0\x80\xFD[a58\x87\x82\x88\x01a4\x9DV[\x95\x98\x94\x97P\x95PPPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x10\x02W`\0\x80\xFD[`\0\x80`\0``\x84\x86\x03\x12\x15a5nW`\0\x80\xFD[\x835a5y\x81a5DV[\x95` \x85\x015\x95P`@\x90\x94\x015\x93\x92PPPV[`\0\x80\x82\x84\x03``\x81\x12\x15a5\xA2W`\0\x80\xFD[\x835\x92P`@`\x1F\x19\x82\x01\x12\x15a5\xB8W`\0\x80\xFD[P` \x83\x01\x90P\x92P\x92\x90PV[`\0`\xC0\x82\x84\x03\x12\x15a4\x97W`\0\x80\xFD[`\0`\xC0\x82\x84\x03\x12\x15a5\xEAW`\0\x80\xFD[a3$\x83\x83a5\xC6V[`\0` \x82\x84\x03\x12\x15a6\x06W`\0\x80\xFD[\x815a3$\x81a5DV[`\0` \x82\x84\x03\x12\x15a6#W`\0\x80\xFD[P5\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a6=W`\0\x80\xFD[\x825a6H\x81a5DV[\x94` \x93\x90\x93\x015\x93PPPV[`\0\x80`\0\x80a\x01\0\x85\x87\x03\x12\x15a6mW`\0\x80\xFD[a6w\x86\x86a5\xC6V[\x93P`\xC0\x85\x015\x92P`\xE0\x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a5,W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a6\xACW`\0\x80\xFD[\x815`\xFF\x81\x16\x81\x14a3$W`\0\x80\xFD[`\0\x80`@\x83\x85\x03\x12\x15a6\xD0W`\0\x80\xFD[PP\x805\x92` \x90\x91\x015\x91PV[cNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[`\x02\x81\x10a\x10\x02Wa\x10\x02a6\xDFV[\x80Qa7\x10\x81a6\xF5V[\x82R` \x90\x81\x01Q\x91\x01RV[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15a7zW\x81Qa7F\x88\x82Qa7\x05V[\x80\x84\x01Q`@\x89\x81\x01\x91\x90\x91R\x81\x01Q\x15\x15``\x80\x8A\x01\x91\x90\x91R\x01Q`\x80\x88\x01R`\xA0\x90\x96\x01\x95\x90\x82\x01\x90`\x01\x01a71V[P\x94\x95\x94PPPPPV[`\0` \x80\x83R`\x80\x80\x84\x01\x85Qa7\x9C\x81a6\xF5V[\x85\x84\x01R\x85\x83\x01Q```@\x80\x88\x01\x82\x90R\x82Q\x93\x84\x90R`\xA0\x93\x92\x86\x01\x92\x84\x89\x01\x90`\0[\x81\x81\x10\x15a8\x1DW\x85Qa7\xD7\x84\x82Qa7\x05V[\x80\x8A\x01Q`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x85\x87\x01R\x85\x82\x01Q\x16\x86\x85\x01R\x85\x81\x01Q\x89\x85\x01R\x88\x81\x01Q\x88\x85\x01R\x87\x01Q`\xC0\x84\x01R\x94\x88\x01\x94`\xE0\x90\x92\x01\x91`\x01\x01a7\xC2V[PP\x89\x82\x01Q\x89\x82\x03`\x1F\x19\x01\x84\x8B\x01R\x96Pa8:\x81\x88a7\x1DV[\x9A\x99PPPPPPPPPPV[`\0`\x80\x82\x84\x03\x12\x15a4\x97W`\0\x80\xFD[`\0\x80`\0\x80`\xC0\x85\x87\x03\x12\x15a8pW`\0\x80\xFD[a8z\x86\x86a8HV[\x93P`\x80\x85\x015\x92P`\xA0\x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a5,W`\0\x80\xFD[`\0`\x80\x82\x84\x03\x12\x15a8\xAFW`\0\x80\xFD[a3$\x83\x83a8HV[\x805c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a8\xCDW`\0\x80\xFD[\x91\x90PV[`\0\x80`\0\x80`\0`\x80\x86\x88\x03\x12\x15a8\xEAW`\0\x80\xFD[\x855\x94Pa8\xFA` \x87\x01a8\xB9V[\x93P`@\x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a9\x16W`\0\x80\xFD[a9\"\x88\x82\x89\x01a4\x9DV[\x90\x94P\x92Pa95\x90P``\x87\x01a8\xB9V[\x90P\x92\x95P\x92\x95\x90\x93PV[`\xE0\x81\x01a9O\x82\x89a7\x05V[`\x01`\x01`\xA0\x1B\x03\x96\x87\x16`@\x83\x01R\x94\x90\x95\x16``\x86\x01R`\x80\x85\x01\x92\x90\x92R`\xA0\x84\x01R`\xC0\x90\x92\x01\x91\x90\x91R\x91\x90PV[` \x81\x01a9\x90\x83a6\xF5V[\x91\x90R\x90V[`\xA0\x81\x01a9\xA4\x82\x87a7\x05V[`@\x82\x01\x94\x90\x94R\x91\x15\x15``\x83\x01R`\x80\x90\x91\x01R\x91\x90PV[`\0`\xA0\x82\x84\x03\x12\x15a9\xD1W`\0\x80\xFD[a3$\x83\x83a4\x85V[`\0\x80`\0\x80`\0\x80`\0`\xC0\x88\x8A\x03\x12\x15a9\xF6W`\0\x80\xFD[a9\xFF\x88a8\xB9V[\x96Pa:\r` \x89\x01a8\xB9V[\x95P`@\x88\x015\x94P``\x88\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a:0W`\0\x80\xFD[a:<\x8A\x82\x8B\x01a4\x9DV[\x90\x95P\x93Pa:O\x90P`\x80\x89\x01a8\xB9V[\x91Pa:]`\xA0\x89\x01a8\xB9V[\x90P\x92\x95\x98\x91\x94\x97P\x92\x95PV[`\x02\x81\x10a\x10\x02W`\0\x80\xFD[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a:\x8EW`\0\x80\xFD[\x845a:\x99\x81a5DV[\x93P` \x85\x015a:\xA9\x81a5DV[\x92P`@\x85\x015a:\xB9\x81a:kV[\x91P``\x85\x015a:\xC9\x81a5DV[\x93\x96\x92\x95P\x90\x93PPV[` \x80\x82R`\x1C\x90\x82\x01R\x7FPausable: contract is paused\0\0\0\0`@\x82\x01R``\x01\x90V[` \x80\x82R`\x1F\x90\x82\x01R\x7FReentrancyGuard: reentrant call\0`@\x82\x01R``\x01\x90V[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0\x82\x82\x10\x15a;jWa;ja;BV[P\x03\x90V[\x82\x81R``\x81\x01a3$` \x83\x01\x84\x805\x82R` \x90\x81\x015\x91\x01RV[` \x80\x82R`\x1C\x90\x82\x01R\x7FTip exceeds deposited amount\0\0\0\0`@\x82\x01R``\x01\x90V[`\0` \x82\x84\x03\x12\x15a;\xD6W`\0\x80\xFD[PQ\x91\x90PV[`\0` \x82\x84\x03\x12\x15a;\xEFW`\0\x80\xFD[\x81Qa3$\x81a5DV[` \x80\x82R`*\x90\x82\x01R\x7Fmsg.sender is not permissioned a`@\x82\x01Ri9\x90:\xB780\xBA\xB9\xB2\xB9`\xB1\x1B``\x82\x01R`\x80\x01\x90V[`\0` \x82\x84\x03\x12\x15a<VW`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a3$W`\0\x80\xFD[` \x80\x82R`(\x90\x82\x01R\x7Fmsg.sender is not permissioned a`@\x82\x01Rg9\x9080\xBA\xB9\xB2\xB9`\xC1\x1B``\x82\x01R`\x80\x01\x90V[`\0`\x01\x82\x01a<\xC0Wa<\xC0a;BV[P`\x01\x01\x90V[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[` \x81\x01`\x03\x83\x10a9\x90Wa9\x90a6\xDFV[\x805a=\x12\x81a:kV[a=\x1B\x81a6\xF5V[\x82R` \x90\x81\x015\x91\x01RV[`\xC0\x81\x01a=6\x82\x84a=\x07V[`@\x83\x015a=D\x81a5DV[`\x01`\x01`\xA0\x1B\x03\x90\x81\x16`@\x84\x01R``\x84\x015\x90a=c\x82a5DV[\x16``\x83\x01R`\x80\x83\x81\x015\x90\x83\x01R`\xA0\x92\x83\x015\x92\x90\x91\x01\x91\x90\x91R\x90V[`\0[\x83\x81\x10\x15a=\x9FW\x81\x81\x01Q\x83\x82\x01R` \x01a=\x87V[\x83\x81\x11\x15a)\xA5WPP`\0\x91\x01RV[`\0\x83Qa=\xC2\x81\x84` \x88\x01a=\x84V[\x83Q\x90\x83\x01\x90a=\xD6\x81\x83` \x88\x01a=\x84V[\x01\x94\x93PPPPV[`\x80\x81\x01a=\xED\x82\x84a=\x07V[`@\x83\x015`@\x83\x01R``\x83\x015a>\x05\x81a5DV[`\x01`\x01`\xA0\x1B\x03\x16``\x92\x90\x92\x01\x91\x90\x91R\x91\x90PV[cNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[`\0c\xFF\xFF\xFF\xFF\x80\x84\x16\x80a>JWa>Ja>\x1DV[\x92\x16\x91\x90\x91\x04\x92\x91PPV[`\0c\xFF\xFF\xFF\xFF\x80\x83\x16\x81\x85\x16\x80\x83\x03\x82\x11\x15a=\xD6Wa=\xD6a;BV[`\0c\xFF\xFF\xFF\xFF\x83\x81\x16\x90\x83\x16\x81\x81\x10\x15a>\x92Wa>\x92a;BV[\x03\x93\x92PPPV[`\0\x82\x19\x82\x11\x15a>\xADWa>\xADa;BV[P\x01\x90V[`\xA0\x81\x01a>\xC0\x82\x84a=\x07V[a>\xDA`@\x83\x01`@\x85\x01\x805\x82R` \x90\x81\x015\x91\x01RV[`\x80\x92\x83\x015\x91\x90\x92\x01R\x90V[`\0c\xFF\xFF\xFF\xFF\x80\x84\x16\x80a>\xFFWa>\xFFa>\x1DV[\x92\x16\x91\x90\x91\x06\x92\x91PPV[`\0c\xFF\xFF\xFF\xFF\x80\x83\x16\x81\x81\x03a?$Wa?$a;BV[`\x01\x01\x93\x92PPPV[`\0\x81a?=Wa?=a;BV[P`\0\x19\x01\x90V[` \x80\x82R\x81\x81\x01R\x7FAmount must be greater than zero`@\x82\x01R``\x01\x90V[`\0\x82Qa?\x8C\x81\x84` \x87\x01a=\x84V[\x91\x90\x91\x01\x92\x91PPV[` \x81R`\0\x82Q\x80` \x84\x01Ra?\xB5\x81`@\x85\x01` \x87\x01a=\x84V[`\x1F\x01`\x1F\x19\x16\x91\x90\x91\x01`@\x01\x92\x91PPV\xFE\xA2dipfsX\"\x12 n\xE6\xC2\x9B\x88\xFBe\x99\xF0!\x19\x95z\"\xD7o'T\xFF\xF6-.:2\xF4\xAA\xD5\xAB\x96\x17\x11fdsolcC\0\x08\r\x003",
    );
    /**Event with signature `DepositAcceptedIntoQueue(uint256,address,address,uint256,uint256)` and selector `0x225305ecb66fa9b9b29f8dead9baea365a6c225d639dfd866e782ccb63e2f05b`.
```solidity
event DepositAcceptedIntoQueue(uint256 requestId, address depositRecipient, address tokenAddress, uint256 amount, uint256 ferryTip);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct DepositAcceptedIntoQueue {
        #[allow(missing_docs)]
        pub requestId: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub depositRecipient: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub tokenAddress: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub amount: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub ferryTip: alloy::sol_types::private::primitives::aliases::U256,
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
        impl alloy_sol_types::SolEvent for DepositAcceptedIntoQueue {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "DepositAcceptedIntoQueue(uint256,address,address,uint256,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                34u8,
                83u8,
                5u8,
                236u8,
                182u8,
                111u8,
                169u8,
                185u8,
                178u8,
                159u8,
                141u8,
                234u8,
                217u8,
                186u8,
                234u8,
                54u8,
                90u8,
                108u8,
                34u8,
                93u8,
                99u8,
                157u8,
                253u8,
                134u8,
                110u8,
                120u8,
                44u8,
                203u8,
                99u8,
                226u8,
                240u8,
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
                    requestId: data.0,
                    depositRecipient: data.1,
                    tokenAddress: data.2,
                    amount: data.3,
                    ferryTip: data.4,
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
                    > as alloy_sol_types::SolType>::tokenize(&self.requestId),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.depositRecipient,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.tokenAddress,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.amount),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.ferryTip),
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
        impl alloy_sol_types::private::IntoLogData for DepositAcceptedIntoQueue {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&DepositAcceptedIntoQueue> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(
                this: &DepositAcceptedIntoQueue,
            ) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `DisputeResolutionAcceptedIntoQueue(uint256,bool,bytes32)` and selector `0x9ef11353afd97d339a77732850b7c22704656558d9ba63cc7e321e0ac4c20aa9`.
```solidity
event DisputeResolutionAcceptedIntoQueue(uint256 requestId, bool cancelJustified, bytes32 cancelResolutionHash);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct DisputeResolutionAcceptedIntoQueue {
        #[allow(missing_docs)]
        pub requestId: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub cancelJustified: bool,
        #[allow(missing_docs)]
        pub cancelResolutionHash: alloy::sol_types::private::FixedBytes<32>,
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
        impl alloy_sol_types::SolEvent for DisputeResolutionAcceptedIntoQueue {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Bool,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "DisputeResolutionAcceptedIntoQueue(uint256,bool,bytes32)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                158u8,
                241u8,
                19u8,
                83u8,
                175u8,
                217u8,
                125u8,
                51u8,
                154u8,
                119u8,
                115u8,
                40u8,
                80u8,
                183u8,
                194u8,
                39u8,
                4u8,
                101u8,
                101u8,
                88u8,
                217u8,
                186u8,
                99u8,
                204u8,
                126u8,
                50u8,
                30u8,
                10u8,
                196u8,
                194u8,
                10u8,
                169u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    requestId: data.0,
                    cancelJustified: data.1,
                    cancelResolutionHash: data.2,
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
                    > as alloy_sol_types::SolType>::tokenize(&self.requestId),
                    <alloy::sol_types::sol_data::Bool as alloy_sol_types::SolType>::tokenize(
                        &self.cancelJustified,
                    ),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.cancelResolutionHash),
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
        impl alloy_sol_types::private::IntoLogData
        for DisputeResolutionAcceptedIntoQueue {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&DisputeResolutionAcceptedIntoQueue>
        for alloy_sol_types::private::LogData {
            #[inline]
            fn from(
                this: &DisputeResolutionAcceptedIntoQueue,
            ) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `ERC20TokensWithdrawn(address,address,uint256)` and selector `0x00e763f7778b8ceef7270c89b7d1df1008b0e482da39c43831417733af96fb0d`.
```solidity
event ERC20TokensWithdrawn(address sender, address token_address, uint256 amount);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct ERC20TokensWithdrawn {
        #[allow(missing_docs)]
        pub sender: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub token_address: alloy::sol_types::private::Address,
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
        impl alloy_sol_types::SolEvent for ERC20TokensWithdrawn {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "ERC20TokensWithdrawn(address,address,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                0u8,
                231u8,
                99u8,
                247u8,
                119u8,
                139u8,
                140u8,
                238u8,
                247u8,
                39u8,
                12u8,
                137u8,
                183u8,
                209u8,
                223u8,
                16u8,
                8u8,
                176u8,
                228u8,
                130u8,
                218u8,
                57u8,
                196u8,
                56u8,
                49u8,
                65u8,
                119u8,
                51u8,
                175u8,
                150u8,
                251u8,
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
                    sender: data.0,
                    token_address: data.1,
                    amount: data.2,
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
                        &self.sender,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.token_address,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.amount),
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
        impl alloy_sol_types::private::IntoLogData for ERC20TokensWithdrawn {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&ERC20TokensWithdrawn> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &ERC20TokensWithdrawn) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `FailedDepositResolutionClosed(uint256,uint256,bytes32)` and selector `0x13750c731f87c15242874ce74bf4649502cc8e7c829067ce846505acdb96289d`.
```solidity
event FailedDepositResolutionClosed(uint256 requestId, uint256 originDepositId, bytes32 failedDespotiResolutionHash);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct FailedDepositResolutionClosed {
        #[allow(missing_docs)]
        pub requestId: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub originDepositId: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub failedDespotiResolutionHash: alloy::sol_types::private::FixedBytes<32>,
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
        impl alloy_sol_types::SolEvent for FailedDepositResolutionClosed {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "FailedDepositResolutionClosed(uint256,uint256,bytes32)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                19u8,
                117u8,
                12u8,
                115u8,
                31u8,
                135u8,
                193u8,
                82u8,
                66u8,
                135u8,
                76u8,
                231u8,
                75u8,
                244u8,
                100u8,
                149u8,
                2u8,
                204u8,
                142u8,
                124u8,
                130u8,
                144u8,
                103u8,
                206u8,
                132u8,
                101u8,
                5u8,
                172u8,
                219u8,
                150u8,
                40u8,
                157u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    requestId: data.0,
                    originDepositId: data.1,
                    failedDespotiResolutionHash: data.2,
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
                    > as alloy_sol_types::SolType>::tokenize(&self.requestId),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.originDepositId),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(
                        &self.failedDespotiResolutionHash,
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
        impl alloy_sol_types::private::IntoLogData for FailedDepositResolutionClosed {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&FailedDepositResolutionClosed> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(
                this: &FailedDepositResolutionClosed,
            ) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `FerriedWithdrawalClosed(uint256,bytes32)` and selector `0x2996fd546c37d74c170466ea6aa4a308e3ca2d4aa689e6e9e32994db5039cc0e`.
```solidity
event FerriedWithdrawalClosed(uint256 requestId, bytes32 withdrawalHash);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct FerriedWithdrawalClosed {
        #[allow(missing_docs)]
        pub requestId: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub withdrawalHash: alloy::sol_types::private::FixedBytes<32>,
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
        impl alloy_sol_types::SolEvent for FerriedWithdrawalClosed {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "FerriedWithdrawalClosed(uint256,bytes32)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                41u8,
                150u8,
                253u8,
                84u8,
                108u8,
                55u8,
                215u8,
                76u8,
                23u8,
                4u8,
                102u8,
                234u8,
                106u8,
                164u8,
                163u8,
                8u8,
                227u8,
                202u8,
                45u8,
                74u8,
                166u8,
                137u8,
                230u8,
                233u8,
                227u8,
                41u8,
                148u8,
                219u8,
                80u8,
                57u8,
                204u8,
                14u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    requestId: data.0,
                    withdrawalHash: data.1,
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
                    > as alloy_sol_types::SolType>::tokenize(&self.requestId),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.withdrawalHash),
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
        impl alloy_sol_types::private::IntoLogData for FerriedWithdrawalClosed {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&FerriedWithdrawalClosed> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(
                this: &FerriedWithdrawalClosed,
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
    /**Event with signature `L2UpdateAccepted(bytes32,(uint256,uint256))` and selector `0x49c158d490db9e066f01b5d4f1a094485a6598cb6c5296b4c07e46c12a1dc11c`.
```solidity
event L2UpdateAccepted(bytes32 root, IRolldownPrimitives.Range range);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct L2UpdateAccepted {
        #[allow(missing_docs)]
        pub root: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub range: <IRolldownPrimitives::Range as alloy::sol_types::SolType>::RustType,
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
        impl alloy_sol_types::SolEvent for L2UpdateAccepted {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                IRolldownPrimitives::Range,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "L2UpdateAccepted(bytes32,(uint256,uint256))";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                73u8,
                193u8,
                88u8,
                212u8,
                144u8,
                219u8,
                158u8,
                6u8,
                111u8,
                1u8,
                181u8,
                212u8,
                241u8,
                160u8,
                148u8,
                72u8,
                90u8,
                101u8,
                152u8,
                203u8,
                108u8,
                82u8,
                150u8,
                180u8,
                192u8,
                126u8,
                70u8,
                193u8,
                42u8,
                29u8,
                193u8,
                28u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    root: data.0,
                    range: data.1,
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
                    > as alloy_sol_types::SolType>::tokenize(&self.root),
                    <IRolldownPrimitives::Range as alloy_sol_types::SolType>::tokenize(
                        &self.range,
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
        impl alloy_sol_types::private::IntoLogData for L2UpdateAccepted {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&L2UpdateAccepted> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &L2UpdateAccepted) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `NativeTokensWithdrawn(address,uint256)` and selector `0xe0495355c1e04c512584521854d222d239a4b782b39ac8a7e835a34f5ec7c1e1`.
```solidity
event NativeTokensWithdrawn(address sender, uint256 amount);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct NativeTokensWithdrawn {
        #[allow(missing_docs)]
        pub sender: alloy::sol_types::private::Address,
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
        impl alloy_sol_types::SolEvent for NativeTokensWithdrawn {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "NativeTokensWithdrawn(address,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                224u8,
                73u8,
                83u8,
                85u8,
                193u8,
                224u8,
                76u8,
                81u8,
                37u8,
                132u8,
                82u8,
                24u8,
                84u8,
                210u8,
                34u8,
                210u8,
                57u8,
                164u8,
                183u8,
                130u8,
                179u8,
                154u8,
                200u8,
                167u8,
                232u8,
                53u8,
                163u8,
                79u8,
                94u8,
                199u8,
                193u8,
                225u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    sender: data.0,
                    amount: data.1,
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
                        &self.sender,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.amount),
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
        impl alloy_sol_types::private::IntoLogData for NativeTokensWithdrawn {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&NativeTokensWithdrawn> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &NativeTokensWithdrawn) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `NewUpdaterSet(address)` and selector `0x1b0f2f500df596b42b73e80dbec6a1fb570f01978a586723f988a5fc54d773a1`.
```solidity
event NewUpdaterSet(address updater);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct NewUpdaterSet {
        #[allow(missing_docs)]
        pub updater: alloy::sol_types::private::Address,
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
        impl alloy_sol_types::SolEvent for NewUpdaterSet {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "NewUpdaterSet(address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                27u8,
                15u8,
                47u8,
                80u8,
                13u8,
                245u8,
                150u8,
                180u8,
                43u8,
                115u8,
                232u8,
                13u8,
                190u8,
                198u8,
                161u8,
                251u8,
                87u8,
                15u8,
                1u8,
                151u8,
                138u8,
                88u8,
                103u8,
                35u8,
                249u8,
                136u8,
                165u8,
                252u8,
                84u8,
                215u8,
                115u8,
                161u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { updater: data.0 }
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
                        &self.updater,
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
        impl alloy_sol_types::private::IntoLogData for NewUpdaterSet {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&NewUpdaterSet> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &NewUpdaterSet) -> alloy_sol_types::private::LogData {
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
    /**Event with signature `PauserRegistrySet(address,address)` and selector `0x6e9fcd539896fca60e8b0f01dd580233e48a6b0f7df013b89ba7f565869acdb6`.
```solidity
event PauserRegistrySet(address pauserRegistry, address newPauserRegistry);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct PauserRegistrySet {
        #[allow(missing_docs)]
        pub pauserRegistry: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub newPauserRegistry: alloy::sol_types::private::Address,
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
        impl alloy_sol_types::SolEvent for PauserRegistrySet {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "PauserRegistrySet(address,address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                110u8,
                159u8,
                205u8,
                83u8,
                152u8,
                150u8,
                252u8,
                166u8,
                14u8,
                139u8,
                15u8,
                1u8,
                221u8,
                88u8,
                2u8,
                51u8,
                228u8,
                138u8,
                107u8,
                15u8,
                125u8,
                240u8,
                19u8,
                184u8,
                155u8,
                167u8,
                245u8,
                101u8,
                134u8,
                154u8,
                205u8,
                182u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    pauserRegistry: data.0,
                    newPauserRegistry: data.1,
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
                        &self.pauserRegistry,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.newPauserRegistry,
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
        impl alloy_sol_types::private::IntoLogData for PauserRegistrySet {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&PauserRegistrySet> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &PauserRegistrySet) -> alloy_sol_types::private::LogData {
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
    /**Event with signature `WithdrawalClosed(uint256,bytes32)` and selector `0x935f26d94be31907080aa78b3e6e2ac6d48a072af096c20268388621bbc11789`.
```solidity
event WithdrawalClosed(uint256 requestId, bytes32 withdrawalHash);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct WithdrawalClosed {
        #[allow(missing_docs)]
        pub requestId: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub withdrawalHash: alloy::sol_types::private::FixedBytes<32>,
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
        impl alloy_sol_types::SolEvent for WithdrawalClosed {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "WithdrawalClosed(uint256,bytes32)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                147u8,
                95u8,
                38u8,
                217u8,
                75u8,
                227u8,
                25u8,
                7u8,
                8u8,
                10u8,
                167u8,
                139u8,
                62u8,
                110u8,
                42u8,
                198u8,
                212u8,
                138u8,
                7u8,
                42u8,
                240u8,
                150u8,
                194u8,
                2u8,
                104u8,
                56u8,
                134u8,
                33u8,
                187u8,
                193u8,
                23u8,
                137u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    requestId: data.0,
                    withdrawalHash: data.1,
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
                    > as alloy_sol_types::SolType>::tokenize(&self.requestId),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.withdrawalHash),
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
        impl alloy_sol_types::private::IntoLogData for WithdrawalClosed {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&WithdrawalClosed> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &WithdrawalClosed) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `WithdrawalFerried(uint256,uint256,address,address,bytes32)` and selector `0x7a9abd9eb86bdbca89cba4069a632c37d93db82e3e14ad8119a3a7812814853e`.
```solidity
event WithdrawalFerried(uint256 requestId, uint256 amount, address recipient, address ferry, bytes32 withdrawalHash);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct WithdrawalFerried {
        #[allow(missing_docs)]
        pub requestId: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub amount: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub recipient: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub ferry: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub withdrawalHash: alloy::sol_types::private::FixedBytes<32>,
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
        impl alloy_sol_types::SolEvent for WithdrawalFerried {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "WithdrawalFerried(uint256,uint256,address,address,bytes32)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                122u8,
                154u8,
                189u8,
                158u8,
                184u8,
                107u8,
                219u8,
                202u8,
                137u8,
                203u8,
                164u8,
                6u8,
                154u8,
                99u8,
                44u8,
                55u8,
                217u8,
                61u8,
                184u8,
                46u8,
                62u8,
                20u8,
                173u8,
                129u8,
                25u8,
                163u8,
                167u8,
                129u8,
                40u8,
                20u8,
                133u8,
                62u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    requestId: data.0,
                    amount: data.1,
                    recipient: data.2,
                    ferry: data.3,
                    withdrawalHash: data.4,
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
                    > as alloy_sol_types::SolType>::tokenize(&self.requestId),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.amount),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.recipient,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.ferry,
                    ),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.withdrawalHash),
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
        impl alloy_sol_types::private::IntoLogData for WithdrawalFerried {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&WithdrawalFerried> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &WithdrawalFerried) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Function with signature `CLOSED()` and selector `0x0e2636a3`.
```solidity
function CLOSED() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct CLOSEDCall {}
    ///Container type for the return parameters of the [`CLOSED()`](CLOSEDCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct CLOSEDReturn {
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
            impl ::core::convert::From<CLOSEDCall> for UnderlyingRustTuple<'_> {
                fn from(value: CLOSEDCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for CLOSEDCall {
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
            impl ::core::convert::From<CLOSEDReturn> for UnderlyingRustTuple<'_> {
                fn from(value: CLOSEDReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for CLOSEDReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for CLOSEDCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = CLOSEDReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "CLOSED()";
            const SELECTOR: [u8; 4] = [14u8, 38u8, 54u8, 163u8];
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
    /**Function with signature `NATIVE_TOKEN_ADDRESS()` and selector `0xdf2ebdbb`.
```solidity
function NATIVE_TOKEN_ADDRESS() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct NATIVE_TOKEN_ADDRESSCall {}
    ///Container type for the return parameters of the [`NATIVE_TOKEN_ADDRESS()`](NATIVE_TOKEN_ADDRESSCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct NATIVE_TOKEN_ADDRESSReturn {
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
            impl ::core::convert::From<NATIVE_TOKEN_ADDRESSCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: NATIVE_TOKEN_ADDRESSCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for NATIVE_TOKEN_ADDRESSCall {
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
            impl ::core::convert::From<NATIVE_TOKEN_ADDRESSReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: NATIVE_TOKEN_ADDRESSReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for NATIVE_TOKEN_ADDRESSReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for NATIVE_TOKEN_ADDRESSCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = NATIVE_TOKEN_ADDRESSReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "NATIVE_TOKEN_ADDRESS()";
            const SELECTOR: [u8; 4] = [223u8, 46u8, 189u8, 187u8];
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
    /**Function with signature `calculate_root(bytes32,uint32,bytes32[],uint32)` and selector `0xaf26c695`.
```solidity
function calculate_root(bytes32 leave_hash, uint32 leave_idx, bytes32[] memory proof, uint32 leaves_count) external pure returns (bytes32);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct calculate_rootCall {
        pub leave_hash: alloy::sol_types::private::FixedBytes<32>,
        pub leave_idx: u32,
        pub proof: alloy::sol_types::private::Vec<
            alloy::sol_types::private::FixedBytes<32>,
        >,
        pub leaves_count: u32,
    }
    ///Container type for the return parameters of the [`calculate_root(bytes32,uint32,bytes32[],uint32)`](calculate_rootCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct calculate_rootReturn {
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
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<32>,
                alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::FixedBytes<32>,
                >,
                alloy::sol_types::sol_data::Uint<32>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::FixedBytes<32>,
                u32,
                alloy::sol_types::private::Vec<
                    alloy::sol_types::private::FixedBytes<32>,
                >,
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
            impl ::core::convert::From<calculate_rootCall> for UnderlyingRustTuple<'_> {
                fn from(value: calculate_rootCall) -> Self {
                    (value.leave_hash, value.leave_idx, value.proof, value.leaves_count)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for calculate_rootCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        leave_hash: tuple.0,
                        leave_idx: tuple.1,
                        proof: tuple.2,
                        leaves_count: tuple.3,
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
            impl ::core::convert::From<calculate_rootReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: calculate_rootReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for calculate_rootReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for calculate_rootCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<32>,
                alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::FixedBytes<32>,
                >,
                alloy::sol_types::sol_data::Uint<32>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = calculate_rootReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "calculate_root(bytes32,uint32,bytes32[],uint32)";
            const SELECTOR: [u8; 4] = [175u8, 38u8, 198u8, 149u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.leave_hash),
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.leave_idx),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::FixedBytes<32>,
                    > as alloy_sol_types::SolType>::tokenize(&self.proof),
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.leaves_count),
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
    /**Function with signature `calculate_root_impl(uint32,uint32,bytes32,bytes32[],uint32,uint32)` and selector `0xef0ba5d0`.
```solidity
function calculate_root_impl(uint32 level, uint32 pos, bytes32 hash, bytes32[] memory proofs, uint32 proof_idx, uint32 max_index) external pure returns (bytes32);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct calculate_root_implCall {
        pub level: u32,
        pub pos: u32,
        pub hash: alloy::sol_types::private::FixedBytes<32>,
        pub proofs: alloy::sol_types::private::Vec<
            alloy::sol_types::private::FixedBytes<32>,
        >,
        pub proof_idx: u32,
        pub max_index: u32,
    }
    ///Container type for the return parameters of the [`calculate_root_impl(uint32,uint32,bytes32,bytes32[],uint32,uint32)`](calculate_root_implCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct calculate_root_implReturn {
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
                alloy::sol_types::sol_data::Uint<32>,
                alloy::sol_types::sol_data::Uint<32>,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::FixedBytes<32>,
                >,
                alloy::sol_types::sol_data::Uint<32>,
                alloy::sol_types::sol_data::Uint<32>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                u32,
                u32,
                alloy::sol_types::private::FixedBytes<32>,
                alloy::sol_types::private::Vec<
                    alloy::sol_types::private::FixedBytes<32>,
                >,
                u32,
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
            impl ::core::convert::From<calculate_root_implCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: calculate_root_implCall) -> Self {
                    (
                        value.level,
                        value.pos,
                        value.hash,
                        value.proofs,
                        value.proof_idx,
                        value.max_index,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for calculate_root_implCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        level: tuple.0,
                        pos: tuple.1,
                        hash: tuple.2,
                        proofs: tuple.3,
                        proof_idx: tuple.4,
                        max_index: tuple.5,
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
            impl ::core::convert::From<calculate_root_implReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: calculate_root_implReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for calculate_root_implReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for calculate_root_implCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<32>,
                alloy::sol_types::sol_data::Uint<32>,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::FixedBytes<32>,
                >,
                alloy::sol_types::sol_data::Uint<32>,
                alloy::sol_types::sol_data::Uint<32>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = calculate_root_implReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "calculate_root_impl(uint32,uint32,bytes32,bytes32[],uint32,uint32)";
            const SELECTOR: [u8; 4] = [239u8, 11u8, 165u8, 208u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.level),
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.pos),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.hash),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::FixedBytes<32>,
                    > as alloy_sol_types::SolType>::tokenize(&self.proofs),
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.proof_idx),
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.max_index),
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
    /**Function with signature `cancelResolutions(uint256)` and selector `0xca9b21ae`.
```solidity
function cancelResolutions(uint256) external view returns (IRolldownPrimitives.RequestId memory requestId, uint256 l2RequestId, bool cancelJustified, uint256 timeStamp);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct cancelResolutionsCall {
        pub _0: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`cancelResolutions(uint256)`](cancelResolutionsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct cancelResolutionsReturn {
        pub requestId: <IRolldownPrimitives::RequestId as alloy::sol_types::SolType>::RustType,
        pub l2RequestId: alloy::sol_types::private::primitives::aliases::U256,
        pub cancelJustified: bool,
        pub timeStamp: alloy::sol_types::private::primitives::aliases::U256,
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
            impl ::core::convert::From<cancelResolutionsCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: cancelResolutionsCall) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for cancelResolutionsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                IRolldownPrimitives::RequestId,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Bool,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <IRolldownPrimitives::RequestId as alloy::sol_types::SolType>::RustType,
                alloy::sol_types::private::primitives::aliases::U256,
                bool,
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
            impl ::core::convert::From<cancelResolutionsReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: cancelResolutionsReturn) -> Self {
                    (
                        value.requestId,
                        value.l2RequestId,
                        value.cancelJustified,
                        value.timeStamp,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for cancelResolutionsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        requestId: tuple.0,
                        l2RequestId: tuple.1,
                        cancelJustified: tuple.2,
                        timeStamp: tuple.3,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for cancelResolutionsCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = cancelResolutionsReturn;
            type ReturnTuple<'a> = (
                IRolldownPrimitives::RequestId,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Bool,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "cancelResolutions(uint256)";
            const SELECTOR: [u8; 4] = [202u8, 155u8, 33u8, 174u8];
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
    /**Function with signature `chain()` and selector `0xc763e5a1`.
```solidity
function chain() external view returns (IRolldownPrimitives.ChainId);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct chainCall {}
    ///Container type for the return parameters of the [`chain()`](chainCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct chainReturn {
        pub _0: <IRolldownPrimitives::ChainId as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<chainCall> for UnderlyingRustTuple<'_> {
                fn from(value: chainCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for chainCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (IRolldownPrimitives::ChainId,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <IRolldownPrimitives::ChainId as alloy::sol_types::SolType>::RustType,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<chainReturn> for UnderlyingRustTuple<'_> {
                fn from(value: chainReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for chainReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for chainCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = chainReturn;
            type ReturnTuple<'a> = (IRolldownPrimitives::ChainId,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "chain()";
            const SELECTOR: [u8; 4] = [199u8, 99u8, 229u8, 161u8];
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
    /**Function with signature `close_cancel(((uint8,uint256),(uint256,uint256),bytes32),bytes32,bytes32[])` and selector `0x01ef6966`.
```solidity
function close_cancel(IRolldownPrimitives.Cancel memory cancel, bytes32 merkle_root, bytes32[] memory proof) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct close_cancelCall {
        pub cancel: <IRolldownPrimitives::Cancel as alloy::sol_types::SolType>::RustType,
        pub merkle_root: alloy::sol_types::private::FixedBytes<32>,
        pub proof: alloy::sol_types::private::Vec<
            alloy::sol_types::private::FixedBytes<32>,
        >,
    }
    ///Container type for the return parameters of the [`close_cancel(((uint8,uint256),(uint256,uint256),bytes32),bytes32,bytes32[])`](close_cancelCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct close_cancelReturn {}
    #[allow(
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
                IRolldownPrimitives::Cancel,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::FixedBytes<32>,
                >,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <IRolldownPrimitives::Cancel as alloy::sol_types::SolType>::RustType,
                alloy::sol_types::private::FixedBytes<32>,
                alloy::sol_types::private::Vec<alloy::sol_types::private::FixedBytes<32>>,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<close_cancelCall> for UnderlyingRustTuple<'_> {
                fn from(value: close_cancelCall) -> Self {
                    (value.cancel, value.merkle_root, value.proof)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for close_cancelCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        cancel: tuple.0,
                        merkle_root: tuple.1,
                        proof: tuple.2,
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
            impl ::core::convert::From<close_cancelReturn> for UnderlyingRustTuple<'_> {
                fn from(value: close_cancelReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for close_cancelReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for close_cancelCall {
            type Parameters<'a> = (
                IRolldownPrimitives::Cancel,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::FixedBytes<32>,
                >,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = close_cancelReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "close_cancel(((uint8,uint256),(uint256,uint256),bytes32),bytes32,bytes32[])";
            const SELECTOR: [u8; 4] = [1u8, 239u8, 105u8, 102u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <IRolldownPrimitives::Cancel as alloy_sol_types::SolType>::tokenize(
                        &self.cancel,
                    ),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.merkle_root),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::FixedBytes<32>,
                    > as alloy_sol_types::SolType>::tokenize(&self.proof),
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
    /**Function with signature `close_deposit_refund(((uint8,uint256),uint256,address),bytes32,bytes32[])` and selector `0x950ac487`.
```solidity
function close_deposit_refund(IRolldownPrimitives.FailedDepositResolution memory failedDeposit, bytes32 merkle_root, bytes32[] memory proof) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct close_deposit_refundCall {
        pub failedDeposit: <IRolldownPrimitives::FailedDepositResolution as alloy::sol_types::SolType>::RustType,
        pub merkle_root: alloy::sol_types::private::FixedBytes<32>,
        pub proof: alloy::sol_types::private::Vec<
            alloy::sol_types::private::FixedBytes<32>,
        >,
    }
    ///Container type for the return parameters of the [`close_deposit_refund(((uint8,uint256),uint256,address),bytes32,bytes32[])`](close_deposit_refundCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct close_deposit_refundReturn {}
    #[allow(
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
                IRolldownPrimitives::FailedDepositResolution,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::FixedBytes<32>,
                >,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <IRolldownPrimitives::FailedDepositResolution as alloy::sol_types::SolType>::RustType,
                alloy::sol_types::private::FixedBytes<32>,
                alloy::sol_types::private::Vec<alloy::sol_types::private::FixedBytes<32>>,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<close_deposit_refundCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: close_deposit_refundCall) -> Self {
                    (value.failedDeposit, value.merkle_root, value.proof)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for close_deposit_refundCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        failedDeposit: tuple.0,
                        merkle_root: tuple.1,
                        proof: tuple.2,
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
            impl ::core::convert::From<close_deposit_refundReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: close_deposit_refundReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for close_deposit_refundReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for close_deposit_refundCall {
            type Parameters<'a> = (
                IRolldownPrimitives::FailedDepositResolution,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::FixedBytes<32>,
                >,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = close_deposit_refundReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "close_deposit_refund(((uint8,uint256),uint256,address),bytes32,bytes32[])";
            const SELECTOR: [u8; 4] = [149u8, 10u8, 196u8, 135u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <IRolldownPrimitives::FailedDepositResolution as alloy_sol_types::SolType>::tokenize(
                        &self.failedDeposit,
                    ),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.merkle_root),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::FixedBytes<32>,
                    > as alloy_sol_types::SolType>::tokenize(&self.proof),
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
    /**Function with signature `close_withdrawal(((uint8,uint256),address,address,uint256,uint256),bytes32,bytes32[])` and selector `0x4bf5fec3`.
```solidity
function close_withdrawal(IRolldownPrimitives.Withdrawal memory withdrawal, bytes32 merkle_root, bytes32[] memory proof) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct close_withdrawalCall {
        pub withdrawal: <IRolldownPrimitives::Withdrawal as alloy::sol_types::SolType>::RustType,
        pub merkle_root: alloy::sol_types::private::FixedBytes<32>,
        pub proof: alloy::sol_types::private::Vec<
            alloy::sol_types::private::FixedBytes<32>,
        >,
    }
    ///Container type for the return parameters of the [`close_withdrawal(((uint8,uint256),address,address,uint256,uint256),bytes32,bytes32[])`](close_withdrawalCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct close_withdrawalReturn {}
    #[allow(
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
                IRolldownPrimitives::Withdrawal,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::FixedBytes<32>,
                >,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <IRolldownPrimitives::Withdrawal as alloy::sol_types::SolType>::RustType,
                alloy::sol_types::private::FixedBytes<32>,
                alloy::sol_types::private::Vec<alloy::sol_types::private::FixedBytes<32>>,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<close_withdrawalCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: close_withdrawalCall) -> Self {
                    (value.withdrawal, value.merkle_root, value.proof)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for close_withdrawalCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        withdrawal: tuple.0,
                        merkle_root: tuple.1,
                        proof: tuple.2,
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
            impl ::core::convert::From<close_withdrawalReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: close_withdrawalReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for close_withdrawalReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for close_withdrawalCall {
            type Parameters<'a> = (
                IRolldownPrimitives::Withdrawal,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::FixedBytes<32>,
                >,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = close_withdrawalReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "close_withdrawal(((uint8,uint256),address,address,uint256,uint256),bytes32,bytes32[])";
            const SELECTOR: [u8; 4] = [75u8, 245u8, 254u8, 195u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <IRolldownPrimitives::Withdrawal as alloy_sol_types::SolType>::tokenize(
                        &self.withdrawal,
                    ),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.merkle_root),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::FixedBytes<32>,
                    > as alloy_sol_types::SolType>::tokenize(&self.proof),
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
    /**Function with signature `counter()` and selector `0x61bc221a`.
```solidity
function counter() external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct counterCall {}
    ///Container type for the return parameters of the [`counter()`](counterCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct counterReturn {
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
            impl ::core::convert::From<counterCall> for UnderlyingRustTuple<'_> {
                fn from(value: counterCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for counterCall {
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
            impl ::core::convert::From<counterReturn> for UnderlyingRustTuple<'_> {
                fn from(value: counterReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for counterReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for counterCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = counterReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "counter()";
            const SELECTOR: [u8; 4] = [97u8, 188u8, 34u8, 26u8];
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
    /**Function with signature `deposit(address,uint256,uint256)` and selector `0x0efe6a8b`.
```solidity
function deposit(address tokenAddress, uint256 amount, uint256 ferryTip) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct deposit_0Call {
        pub tokenAddress: alloy::sol_types::private::Address,
        pub amount: alloy::sol_types::private::primitives::aliases::U256,
        pub ferryTip: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`deposit(address,uint256,uint256)`](deposit_0Call) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct deposit_0Return {}
    #[allow(
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
            impl ::core::convert::From<deposit_0Call> for UnderlyingRustTuple<'_> {
                fn from(value: deposit_0Call) -> Self {
                    (value.tokenAddress, value.amount, value.ferryTip)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for deposit_0Call {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        tokenAddress: tuple.0,
                        amount: tuple.1,
                        ferryTip: tuple.2,
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
            impl ::core::convert::From<deposit_0Return> for UnderlyingRustTuple<'_> {
                fn from(value: deposit_0Return) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for deposit_0Return {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for deposit_0Call {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = deposit_0Return;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "deposit(address,uint256,uint256)";
            const SELECTOR: [u8; 4] = [14u8, 254u8, 106u8, 139u8];
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
                        &self.tokenAddress,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.amount),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.ferryTip),
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
    /**Function with signature `deposit(address,uint256)` and selector `0x47e7ef24`.
```solidity
function deposit(address tokenAddress, uint256 amount) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct deposit_1Call {
        pub tokenAddress: alloy::sol_types::private::Address,
        pub amount: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`deposit(address,uint256)`](deposit_1Call) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct deposit_1Return {}
    #[allow(
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
            impl ::core::convert::From<deposit_1Call> for UnderlyingRustTuple<'_> {
                fn from(value: deposit_1Call) -> Self {
                    (value.tokenAddress, value.amount)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for deposit_1Call {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        tokenAddress: tuple.0,
                        amount: tuple.1,
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
            impl ::core::convert::From<deposit_1Return> for UnderlyingRustTuple<'_> {
                fn from(value: deposit_1Return) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for deposit_1Return {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for deposit_1Call {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = deposit_1Return;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "deposit(address,uint256)";
            const SELECTOR: [u8; 4] = [71u8, 231u8, 239u8, 36u8];
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
                        &self.tokenAddress,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.amount),
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
    /**Function with signature `deposit_erc20(address,uint256,uint256)` and selector `0x08aba1b2`.
```solidity
function deposit_erc20(address tokenAddress, uint256 amount, uint256 ferryTip) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct deposit_erc20_0Call {
        pub tokenAddress: alloy::sol_types::private::Address,
        pub amount: alloy::sol_types::private::primitives::aliases::U256,
        pub ferryTip: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`deposit_erc20(address,uint256,uint256)`](deposit_erc20_0Call) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct deposit_erc20_0Return {}
    #[allow(
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
            impl ::core::convert::From<deposit_erc20_0Call> for UnderlyingRustTuple<'_> {
                fn from(value: deposit_erc20_0Call) -> Self {
                    (value.tokenAddress, value.amount, value.ferryTip)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for deposit_erc20_0Call {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        tokenAddress: tuple.0,
                        amount: tuple.1,
                        ferryTip: tuple.2,
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
            impl ::core::convert::From<deposit_erc20_0Return>
            for UnderlyingRustTuple<'_> {
                fn from(value: deposit_erc20_0Return) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for deposit_erc20_0Return {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for deposit_erc20_0Call {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = deposit_erc20_0Return;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "deposit_erc20(address,uint256,uint256)";
            const SELECTOR: [u8; 4] = [8u8, 171u8, 161u8, 178u8];
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
                        &self.tokenAddress,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.amount),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.ferryTip),
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
    /**Function with signature `deposit_erc20(address,uint256)` and selector `0xd16544f0`.
```solidity
function deposit_erc20(address tokenAddress, uint256 amount) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct deposit_erc20_1Call {
        pub tokenAddress: alloy::sol_types::private::Address,
        pub amount: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`deposit_erc20(address,uint256)`](deposit_erc20_1Call) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct deposit_erc20_1Return {}
    #[allow(
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
            impl ::core::convert::From<deposit_erc20_1Call> for UnderlyingRustTuple<'_> {
                fn from(value: deposit_erc20_1Call) -> Self {
                    (value.tokenAddress, value.amount)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for deposit_erc20_1Call {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        tokenAddress: tuple.0,
                        amount: tuple.1,
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
            impl ::core::convert::From<deposit_erc20_1Return>
            for UnderlyingRustTuple<'_> {
                fn from(value: deposit_erc20_1Return) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for deposit_erc20_1Return {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for deposit_erc20_1Call {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = deposit_erc20_1Return;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "deposit_erc20(address,uint256)";
            const SELECTOR: [u8; 4] = [209u8, 101u8, 68u8, 240u8];
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
                        &self.tokenAddress,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.amount),
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
    /**Function with signature `deposit_native()` and selector `0xc87c2224`.
```solidity
function deposit_native() external payable;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct deposit_native_0Call {}
    ///Container type for the return parameters of the [`deposit_native()`](deposit_native_0Call) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct deposit_native_0Return {}
    #[allow(
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
            impl ::core::convert::From<deposit_native_0Call>
            for UnderlyingRustTuple<'_> {
                fn from(value: deposit_native_0Call) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for deposit_native_0Call {
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
            impl ::core::convert::From<deposit_native_0Return>
            for UnderlyingRustTuple<'_> {
                fn from(value: deposit_native_0Return) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for deposit_native_0Return {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for deposit_native_0Call {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = deposit_native_0Return;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "deposit_native()";
            const SELECTOR: [u8; 4] = [200u8, 124u8, 34u8, 36u8];
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
    /**Function with signature `deposit_native(uint256)` and selector `0xdffbdd9f`.
```solidity
function deposit_native(uint256 ferryTip) external payable;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct deposit_native_1Call {
        pub ferryTip: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`deposit_native(uint256)`](deposit_native_1Call) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct deposit_native_1Return {}
    #[allow(
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
            impl ::core::convert::From<deposit_native_1Call>
            for UnderlyingRustTuple<'_> {
                fn from(value: deposit_native_1Call) -> Self {
                    (value.ferryTip,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for deposit_native_1Call {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { ferryTip: tuple.0 }
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
            impl ::core::convert::From<deposit_native_1Return>
            for UnderlyingRustTuple<'_> {
                fn from(value: deposit_native_1Return) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for deposit_native_1Return {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for deposit_native_1Call {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = deposit_native_1Return;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "deposit_native(uint256)";
            const SELECTOR: [u8; 4] = [223u8, 251u8, 221u8, 159u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.ferryTip),
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
    /**Function with signature `deposits(uint256)` and selector `0xb02c43d0`.
```solidity
function deposits(uint256) external view returns (IRolldownPrimitives.RequestId memory requestId, address depositRecipient, address tokenAddress, uint256 amount, uint256 timeStamp, uint256 ferryTip);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct depositsCall {
        pub _0: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`deposits(uint256)`](depositsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct depositsReturn {
        pub requestId: <IRolldownPrimitives::RequestId as alloy::sol_types::SolType>::RustType,
        pub depositRecipient: alloy::sol_types::private::Address,
        pub tokenAddress: alloy::sol_types::private::Address,
        pub amount: alloy::sol_types::private::primitives::aliases::U256,
        pub timeStamp: alloy::sol_types::private::primitives::aliases::U256,
        pub ferryTip: alloy::sol_types::private::primitives::aliases::U256,
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
            impl ::core::convert::From<depositsCall> for UnderlyingRustTuple<'_> {
                fn from(value: depositsCall) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for depositsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                IRolldownPrimitives::RequestId,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <IRolldownPrimitives::RequestId as alloy::sol_types::SolType>::RustType,
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Address,
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
            impl ::core::convert::From<depositsReturn> for UnderlyingRustTuple<'_> {
                fn from(value: depositsReturn) -> Self {
                    (
                        value.requestId,
                        value.depositRecipient,
                        value.tokenAddress,
                        value.amount,
                        value.timeStamp,
                        value.ferryTip,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for depositsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        requestId: tuple.0,
                        depositRecipient: tuple.1,
                        tokenAddress: tuple.2,
                        amount: tuple.3,
                        timeStamp: tuple.4,
                        ferryTip: tuple.5,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for depositsCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = depositsReturn;
            type ReturnTuple<'a> = (
                IRolldownPrimitives::RequestId,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "deposits(uint256)";
            const SELECTOR: [u8; 4] = [176u8, 44u8, 67u8, 208u8];
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
    /**Function with signature `ferry_withdrawal(((uint8,uint256),address,address,uint256,uint256))` and selector `0x0cac57ab`.
```solidity
function ferry_withdrawal(IRolldownPrimitives.Withdrawal memory withdrawal) external payable;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ferry_withdrawalCall {
        pub withdrawal: <IRolldownPrimitives::Withdrawal as alloy::sol_types::SolType>::RustType,
    }
    ///Container type for the return parameters of the [`ferry_withdrawal(((uint8,uint256),address,address,uint256,uint256))`](ferry_withdrawalCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ferry_withdrawalReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (IRolldownPrimitives::Withdrawal,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <IRolldownPrimitives::Withdrawal as alloy::sol_types::SolType>::RustType,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<ferry_withdrawalCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: ferry_withdrawalCall) -> Self {
                    (value.withdrawal,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for ferry_withdrawalCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { withdrawal: tuple.0 }
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
            impl ::core::convert::From<ferry_withdrawalReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: ferry_withdrawalReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for ferry_withdrawalReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for ferry_withdrawalCall {
            type Parameters<'a> = (IRolldownPrimitives::Withdrawal,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = ferry_withdrawalReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ferry_withdrawal(((uint8,uint256),address,address,uint256,uint256))";
            const SELECTOR: [u8; 4] = [12u8, 172u8, 87u8, 171u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <IRolldownPrimitives::Withdrawal as alloy_sol_types::SolType>::tokenize(
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
    /**Function with signature `find_l2_batch(uint256)` and selector `0xf9ecd01e`.
```solidity
function find_l2_batch(uint256 requestId) external view returns (bytes32);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct find_l2_batchCall {
        pub requestId: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`find_l2_batch(uint256)`](find_l2_batchCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct find_l2_batchReturn {
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
            impl ::core::convert::From<find_l2_batchCall> for UnderlyingRustTuple<'_> {
                fn from(value: find_l2_batchCall) -> Self {
                    (value.requestId,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for find_l2_batchCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { requestId: tuple.0 }
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
            impl ::core::convert::From<find_l2_batchReturn> for UnderlyingRustTuple<'_> {
                fn from(value: find_l2_batchReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for find_l2_batchReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for find_l2_batchCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = find_l2_batchReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "find_l2_batch(uint256)";
            const SELECTOR: [u8; 4] = [249u8, 236u8, 208u8, 30u8];
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
    /**Function with signature `getMerkleRootsLength()` and selector `0xff2bae86`.
```solidity
function getMerkleRootsLength() external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getMerkleRootsLengthCall {}
    ///Container type for the return parameters of the [`getMerkleRootsLength()`](getMerkleRootsLengthCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getMerkleRootsLengthReturn {
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
            impl ::core::convert::From<getMerkleRootsLengthCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getMerkleRootsLengthCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getMerkleRootsLengthCall {
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
            impl ::core::convert::From<getMerkleRootsLengthReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getMerkleRootsLengthReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getMerkleRootsLengthReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getMerkleRootsLengthCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getMerkleRootsLengthReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getMerkleRootsLength()";
            const SELECTOR: [u8; 4] = [255u8, 43u8, 174u8, 134u8];
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
    /**Function with signature `getPendingRequests(uint256,uint256)` and selector `0x79e041f2`.
```solidity
function getPendingRequests(uint256 start, uint256 end) external view returns (IRolldownPrimitives.L1Update memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getPendingRequestsCall {
        pub start: alloy::sol_types::private::primitives::aliases::U256,
        pub end: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`getPendingRequests(uint256,uint256)`](getPendingRequestsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getPendingRequestsReturn {
        pub _0: <IRolldownPrimitives::L1Update as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<getPendingRequestsCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getPendingRequestsCall) -> Self {
                    (value.start, value.end)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getPendingRequestsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        start: tuple.0,
                        end: tuple.1,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (IRolldownPrimitives::L1Update,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <IRolldownPrimitives::L1Update as alloy::sol_types::SolType>::RustType,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getPendingRequestsReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getPendingRequestsReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getPendingRequestsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getPendingRequestsCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getPendingRequestsReturn;
            type ReturnTuple<'a> = (IRolldownPrimitives::L1Update,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getPendingRequests(uint256,uint256)";
            const SELECTOR: [u8; 4] = [121u8, 224u8, 65u8, 242u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.start),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.end),
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
    /**Function with signature `getUpdateForL2()` and selector `0xb1538706`.
```solidity
function getUpdateForL2() external view returns (IRolldownPrimitives.L1Update memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getUpdateForL2Call {}
    ///Container type for the return parameters of the [`getUpdateForL2()`](getUpdateForL2Call) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getUpdateForL2Return {
        pub _0: <IRolldownPrimitives::L1Update as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<getUpdateForL2Call> for UnderlyingRustTuple<'_> {
                fn from(value: getUpdateForL2Call) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getUpdateForL2Call {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (IRolldownPrimitives::L1Update,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <IRolldownPrimitives::L1Update as alloy::sol_types::SolType>::RustType,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getUpdateForL2Return>
            for UnderlyingRustTuple<'_> {
                fn from(value: getUpdateForL2Return) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getUpdateForL2Return {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getUpdateForL2Call {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getUpdateForL2Return;
            type ReturnTuple<'a> = (IRolldownPrimitives::L1Update,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getUpdateForL2()";
            const SELECTOR: [u8; 4] = [177u8, 83u8, 135u8, 6u8];
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
    /**Function with signature `hashCancel(((uint8,uint256),(uint256,uint256),bytes32))` and selector `0xcc8c909f`.
```solidity
function hashCancel(IRolldownPrimitives.Cancel memory cancel) external pure returns (bytes32);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct hashCancelCall {
        pub cancel: <IRolldownPrimitives::Cancel as alloy::sol_types::SolType>::RustType,
    }
    ///Container type for the return parameters of the [`hashCancel(((uint8,uint256),(uint256,uint256),bytes32))`](hashCancelCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct hashCancelReturn {
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
            type UnderlyingSolTuple<'a> = (IRolldownPrimitives::Cancel,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <IRolldownPrimitives::Cancel as alloy::sol_types::SolType>::RustType,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<hashCancelCall> for UnderlyingRustTuple<'_> {
                fn from(value: hashCancelCall) -> Self {
                    (value.cancel,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for hashCancelCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { cancel: tuple.0 }
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
            impl ::core::convert::From<hashCancelReturn> for UnderlyingRustTuple<'_> {
                fn from(value: hashCancelReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for hashCancelReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for hashCancelCall {
            type Parameters<'a> = (IRolldownPrimitives::Cancel,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = hashCancelReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "hashCancel(((uint8,uint256),(uint256,uint256),bytes32))";
            const SELECTOR: [u8; 4] = [204u8, 140u8, 144u8, 159u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <IRolldownPrimitives::Cancel as alloy_sol_types::SolType>::tokenize(
                        &self.cancel,
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
    /**Function with signature `hashFailedDepositResolution(((uint8,uint256),uint256,address))` and selector `0xae46db11`.
```solidity
function hashFailedDepositResolution(IRolldownPrimitives.FailedDepositResolution memory failedDeposit) external pure returns (bytes32);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct hashFailedDepositResolutionCall {
        pub failedDeposit: <IRolldownPrimitives::FailedDepositResolution as alloy::sol_types::SolType>::RustType,
    }
    ///Container type for the return parameters of the [`hashFailedDepositResolution(((uint8,uint256),uint256,address))`](hashFailedDepositResolutionCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct hashFailedDepositResolutionReturn {
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
                IRolldownPrimitives::FailedDepositResolution,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <IRolldownPrimitives::FailedDepositResolution as alloy::sol_types::SolType>::RustType,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<hashFailedDepositResolutionCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: hashFailedDepositResolutionCall) -> Self {
                    (value.failedDeposit,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for hashFailedDepositResolutionCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { failedDeposit: tuple.0 }
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
            impl ::core::convert::From<hashFailedDepositResolutionReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: hashFailedDepositResolutionReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for hashFailedDepositResolutionReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for hashFailedDepositResolutionCall {
            type Parameters<'a> = (IRolldownPrimitives::FailedDepositResolution,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = hashFailedDepositResolutionReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "hashFailedDepositResolution(((uint8,uint256),uint256,address))";
            const SELECTOR: [u8; 4] = [174u8, 70u8, 219u8, 17u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <IRolldownPrimitives::FailedDepositResolution as alloy_sol_types::SolType>::tokenize(
                        &self.failedDeposit,
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
    /**Function with signature `hashWithdrawal(((uint8,uint256),address,address,uint256,uint256))` and selector `0x890e95ce`.
```solidity
function hashWithdrawal(IRolldownPrimitives.Withdrawal memory withdrawal) external pure returns (bytes32);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct hashWithdrawalCall {
        pub withdrawal: <IRolldownPrimitives::Withdrawal as alloy::sol_types::SolType>::RustType,
    }
    ///Container type for the return parameters of the [`hashWithdrawal(((uint8,uint256),address,address,uint256,uint256))`](hashWithdrawalCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct hashWithdrawalReturn {
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
            type UnderlyingSolTuple<'a> = (IRolldownPrimitives::Withdrawal,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <IRolldownPrimitives::Withdrawal as alloy::sol_types::SolType>::RustType,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<hashWithdrawalCall> for UnderlyingRustTuple<'_> {
                fn from(value: hashWithdrawalCall) -> Self {
                    (value.withdrawal,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for hashWithdrawalCall {
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
            impl ::core::convert::From<hashWithdrawalReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: hashWithdrawalReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for hashWithdrawalReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for hashWithdrawalCall {
            type Parameters<'a> = (IRolldownPrimitives::Withdrawal,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = hashWithdrawalReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "hashWithdrawal(((uint8,uint256),address,address,uint256,uint256))";
            const SELECTOR: [u8; 4] = [137u8, 14u8, 149u8, 206u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <IRolldownPrimitives::Withdrawal as alloy_sol_types::SolType>::tokenize(
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
    /**Function with signature `initialize(address,address,uint8,address)` and selector `0xffea632b`.
```solidity
function initialize(address _pauserRegistry, address initialOwner, IRolldownPrimitives.ChainId chainId, address updater) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct initializeCall {
        pub _pauserRegistry: alloy::sol_types::private::Address,
        pub initialOwner: alloy::sol_types::private::Address,
        pub chainId: <IRolldownPrimitives::ChainId as alloy::sol_types::SolType>::RustType,
        pub updater: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`initialize(address,address,uint8,address)`](initializeCall) function.
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
                IRolldownPrimitives::ChainId,
                alloy::sol_types::sol_data::Address,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Address,
                <IRolldownPrimitives::ChainId as alloy::sol_types::SolType>::RustType,
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
                    (
                        value._pauserRegistry,
                        value.initialOwner,
                        value.chainId,
                        value.updater,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for initializeCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _pauserRegistry: tuple.0,
                        initialOwner: tuple.1,
                        chainId: tuple.2,
                        updater: tuple.3,
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
                IRolldownPrimitives::ChainId,
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
            const SIGNATURE: &'static str = "initialize(address,address,uint8,address)";
            const SELECTOR: [u8; 4] = [255u8, 234u8, 99u8, 43u8];
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
                        &self._pauserRegistry,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.initialOwner,
                    ),
                    <IRolldownPrimitives::ChainId as alloy_sol_types::SolType>::tokenize(
                        &self.chainId,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.updater,
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
    /**Function with signature `lastProcessedUpdate_origin_l1()` and selector `0x7fd4f845`.
```solidity
function lastProcessedUpdate_origin_l1() external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct lastProcessedUpdate_origin_l1Call {}
    ///Container type for the return parameters of the [`lastProcessedUpdate_origin_l1()`](lastProcessedUpdate_origin_l1Call) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct lastProcessedUpdate_origin_l1Return {
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
            impl ::core::convert::From<lastProcessedUpdate_origin_l1Call>
            for UnderlyingRustTuple<'_> {
                fn from(value: lastProcessedUpdate_origin_l1Call) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for lastProcessedUpdate_origin_l1Call {
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
            impl ::core::convert::From<lastProcessedUpdate_origin_l1Return>
            for UnderlyingRustTuple<'_> {
                fn from(value: lastProcessedUpdate_origin_l1Return) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for lastProcessedUpdate_origin_l1Return {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for lastProcessedUpdate_origin_l1Call {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = lastProcessedUpdate_origin_l1Return;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "lastProcessedUpdate_origin_l1()";
            const SELECTOR: [u8; 4] = [127u8, 212u8, 248u8, 69u8];
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
    /**Function with signature `lastProcessedUpdate_origin_l2()` and selector `0xf26ee9d0`.
```solidity
function lastProcessedUpdate_origin_l2() external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct lastProcessedUpdate_origin_l2Call {}
    ///Container type for the return parameters of the [`lastProcessedUpdate_origin_l2()`](lastProcessedUpdate_origin_l2Call) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct lastProcessedUpdate_origin_l2Return {
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
            impl ::core::convert::From<lastProcessedUpdate_origin_l2Call>
            for UnderlyingRustTuple<'_> {
                fn from(value: lastProcessedUpdate_origin_l2Call) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for lastProcessedUpdate_origin_l2Call {
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
            impl ::core::convert::From<lastProcessedUpdate_origin_l2Return>
            for UnderlyingRustTuple<'_> {
                fn from(value: lastProcessedUpdate_origin_l2Return) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for lastProcessedUpdate_origin_l2Return {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for lastProcessedUpdate_origin_l2Call {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = lastProcessedUpdate_origin_l2Return;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "lastProcessedUpdate_origin_l2()";
            const SELECTOR: [u8; 4] = [242u8, 110u8, 233u8, 208u8];
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
    /**Function with signature `merkleRootRange(bytes32)` and selector `0x4f48eedf`.
```solidity
function merkleRootRange(bytes32) external view returns (uint256 start, uint256 end);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct merkleRootRangeCall {
        pub _0: alloy::sol_types::private::FixedBytes<32>,
    }
    ///Container type for the return parameters of the [`merkleRootRange(bytes32)`](merkleRootRangeCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct merkleRootRangeReturn {
        pub start: alloy::sol_types::private::primitives::aliases::U256,
        pub end: alloy::sol_types::private::primitives::aliases::U256,
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
            impl ::core::convert::From<merkleRootRangeCall> for UnderlyingRustTuple<'_> {
                fn from(value: merkleRootRangeCall) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for merkleRootRangeCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        {
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
            impl ::core::convert::From<merkleRootRangeReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: merkleRootRangeReturn) -> Self {
                    (value.start, value.end)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for merkleRootRangeReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        start: tuple.0,
                        end: tuple.1,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for merkleRootRangeCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = merkleRootRangeReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "merkleRootRange(bytes32)";
            const SELECTOR: [u8; 4] = [79u8, 72u8, 238u8, 223u8];
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
        pub index: u8,
    }
    ///Container type for the return parameters of the [`paused(uint8)`](paused_0Call) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct paused_0Return {
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
    /**Function with signature `processedL2Requests(bytes32)` and selector `0xde70e0b8`.
```solidity
function processedL2Requests(bytes32) external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct processedL2RequestsCall {
        pub _0: alloy::sol_types::private::FixedBytes<32>,
    }
    ///Container type for the return parameters of the [`processedL2Requests(bytes32)`](processedL2RequestsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct processedL2RequestsReturn {
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
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<processedL2RequestsCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: processedL2RequestsCall) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for processedL2RequestsCall {
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
            impl ::core::convert::From<processedL2RequestsReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: processedL2RequestsReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for processedL2RequestsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for processedL2RequestsCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = processedL2RequestsReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "processedL2Requests(bytes32)";
            const SELECTOR: [u8; 4] = [222u8, 112u8, 224u8, 184u8];
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
    /**Function with signature `roots(uint256)` and selector `0xc2b40ae4`.
```solidity
function roots(uint256) external view returns (bytes32);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct rootsCall {
        pub _0: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`roots(uint256)`](rootsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct rootsReturn {
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
            impl ::core::convert::From<rootsCall> for UnderlyingRustTuple<'_> {
                fn from(value: rootsCall) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for rootsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
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
            impl ::core::convert::From<rootsReturn> for UnderlyingRustTuple<'_> {
                fn from(value: rootsReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for rootsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for rootsCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = rootsReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "roots(uint256)";
            const SELECTOR: [u8; 4] = [194u8, 180u8, 10u8, 228u8];
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
    /**Function with signature `setPauserRegistry(address)` and selector `0x10d67a2f`.
```solidity
function setPauserRegistry(address newPauserRegistry) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setPauserRegistryCall {
        pub newPauserRegistry: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`setPauserRegistry(address)`](setPauserRegistryCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setPauserRegistryReturn {}
    #[allow(
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
            impl ::core::convert::From<setPauserRegistryCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: setPauserRegistryCall) -> Self {
                    (value.newPauserRegistry,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for setPauserRegistryCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { newPauserRegistry: tuple.0 }
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
            impl ::core::convert::From<setPauserRegistryReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: setPauserRegistryReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for setPauserRegistryReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for setPauserRegistryCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = setPauserRegistryReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "setPauserRegistry(address)";
            const SELECTOR: [u8; 4] = [16u8, 214u8, 122u8, 47u8];
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
                        &self.newPauserRegistry,
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
    /**Function with signature `setUpdater(address)` and selector `0x9d54f419`.
```solidity
function setUpdater(address updater) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setUpdaterCall {
        pub updater: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`setUpdater(address)`](setUpdaterCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setUpdaterReturn {}
    #[allow(
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
            impl ::core::convert::From<setUpdaterCall> for UnderlyingRustTuple<'_> {
                fn from(value: setUpdaterCall) -> Self {
                    (value.updater,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for setUpdaterCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { updater: tuple.0 }
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
            impl ::core::convert::From<setUpdaterReturn> for UnderlyingRustTuple<'_> {
                fn from(value: setUpdaterReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for setUpdaterReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for setUpdaterCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = setUpdaterReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "setUpdater(address)";
            const SELECTOR: [u8; 4] = [157u8, 84u8, 244u8, 25u8];
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
                        &self.updater,
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
    /**Function with signature `update_l1_from_l2(bytes32,(uint256,uint256))` and selector `0x08f42d40`.
```solidity
function update_l1_from_l2(bytes32 merkle_root, IRolldownPrimitives.Range memory range) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct update_l1_from_l2Call {
        pub merkle_root: alloy::sol_types::private::FixedBytes<32>,
        pub range: <IRolldownPrimitives::Range as alloy::sol_types::SolType>::RustType,
    }
    ///Container type for the return parameters of the [`update_l1_from_l2(bytes32,(uint256,uint256))`](update_l1_from_l2Call) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct update_l1_from_l2Return {}
    #[allow(
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
                IRolldownPrimitives::Range,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::FixedBytes<32>,
                <IRolldownPrimitives::Range as alloy::sol_types::SolType>::RustType,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<update_l1_from_l2Call>
            for UnderlyingRustTuple<'_> {
                fn from(value: update_l1_from_l2Call) -> Self {
                    (value.merkle_root, value.range)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for update_l1_from_l2Call {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        merkle_root: tuple.0,
                        range: tuple.1,
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
            impl ::core::convert::From<update_l1_from_l2Return>
            for UnderlyingRustTuple<'_> {
                fn from(value: update_l1_from_l2Return) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for update_l1_from_l2Return {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for update_l1_from_l2Call {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                IRolldownPrimitives::Range,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = update_l1_from_l2Return;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "update_l1_from_l2(bytes32,(uint256,uint256))";
            const SELECTOR: [u8; 4] = [8u8, 244u8, 45u8, 64u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.merkle_root),
                    <IRolldownPrimitives::Range as alloy_sol_types::SolType>::tokenize(
                        &self.range,
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
    /**Function with signature `updaterAccount()` and selector `0x71c54461`.
```solidity
function updaterAccount() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct updaterAccountCall {}
    ///Container type for the return parameters of the [`updaterAccount()`](updaterAccountCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct updaterAccountReturn {
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
            impl ::core::convert::From<updaterAccountCall> for UnderlyingRustTuple<'_> {
                fn from(value: updaterAccountCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for updaterAccountCall {
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
            impl ::core::convert::From<updaterAccountReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: updaterAccountReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for updaterAccountReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for updaterAccountCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = updaterAccountReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "updaterAccount()";
            const SELECTOR: [u8; 4] = [113u8, 197u8, 68u8, 97u8];
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
    ///Container for all the [`Rolldown`](self) function calls.
    pub enum RolldownCalls {
        CLOSED(CLOSEDCall),
        NATIVE_TOKEN_ADDRESS(NATIVE_TOKEN_ADDRESSCall),
        calculate_root(calculate_rootCall),
        calculate_root_impl(calculate_root_implCall),
        cancelResolutions(cancelResolutionsCall),
        chain(chainCall),
        close_cancel(close_cancelCall),
        close_deposit_refund(close_deposit_refundCall),
        close_withdrawal(close_withdrawalCall),
        counter(counterCall),
        deposit_0(deposit_0Call),
        deposit_1(deposit_1Call),
        deposit_erc20_0(deposit_erc20_0Call),
        deposit_erc20_1(deposit_erc20_1Call),
        deposit_native_0(deposit_native_0Call),
        deposit_native_1(deposit_native_1Call),
        deposits(depositsCall),
        ferry_withdrawal(ferry_withdrawalCall),
        find_l2_batch(find_l2_batchCall),
        getMerkleRootsLength(getMerkleRootsLengthCall),
        getPendingRequests(getPendingRequestsCall),
        getUpdateForL2(getUpdateForL2Call),
        hashCancel(hashCancelCall),
        hashFailedDepositResolution(hashFailedDepositResolutionCall),
        hashWithdrawal(hashWithdrawalCall),
        initialize(initializeCall),
        lastProcessedUpdate_origin_l1(lastProcessedUpdate_origin_l1Call),
        lastProcessedUpdate_origin_l2(lastProcessedUpdate_origin_l2Call),
        merkleRootRange(merkleRootRangeCall),
        owner(ownerCall),
        pause(pauseCall),
        pauseAll(pauseAllCall),
        paused_0(paused_0Call),
        paused_1(paused_1Call),
        pauserRegistry(pauserRegistryCall),
        processedL2Requests(processedL2RequestsCall),
        renounceOwnership(renounceOwnershipCall),
        roots(rootsCall),
        setPauserRegistry(setPauserRegistryCall),
        setUpdater(setUpdaterCall),
        transferOwnership(transferOwnershipCall),
        unpause(unpauseCall),
        update_l1_from_l2(update_l1_from_l2Call),
        updaterAccount(updaterAccountCall),
    }
    #[automatically_derived]
    impl RolldownCalls {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [1u8, 239u8, 105u8, 102u8],
            [8u8, 171u8, 161u8, 178u8],
            [8u8, 244u8, 45u8, 64u8],
            [12u8, 172u8, 87u8, 171u8],
            [14u8, 38u8, 54u8, 163u8],
            [14u8, 254u8, 106u8, 139u8],
            [16u8, 214u8, 122u8, 47u8],
            [19u8, 100u8, 57u8, 221u8],
            [71u8, 231u8, 239u8, 36u8],
            [75u8, 245u8, 254u8, 195u8],
            [79u8, 72u8, 238u8, 223u8],
            [89u8, 92u8, 106u8, 103u8],
            [90u8, 200u8, 106u8, 183u8],
            [92u8, 151u8, 90u8, 187u8],
            [97u8, 188u8, 34u8, 26u8],
            [113u8, 80u8, 24u8, 166u8],
            [113u8, 197u8, 68u8, 97u8],
            [121u8, 224u8, 65u8, 242u8],
            [127u8, 212u8, 248u8, 69u8],
            [136u8, 111u8, 17u8, 149u8],
            [137u8, 14u8, 149u8, 206u8],
            [141u8, 165u8, 203u8, 91u8],
            [149u8, 10u8, 196u8, 135u8],
            [157u8, 84u8, 244u8, 25u8],
            [174u8, 70u8, 219u8, 17u8],
            [175u8, 38u8, 198u8, 149u8],
            [176u8, 44u8, 67u8, 208u8],
            [177u8, 83u8, 135u8, 6u8],
            [194u8, 180u8, 10u8, 228u8],
            [199u8, 99u8, 229u8, 161u8],
            [200u8, 124u8, 34u8, 36u8],
            [202u8, 155u8, 33u8, 174u8],
            [204u8, 140u8, 144u8, 159u8],
            [209u8, 101u8, 68u8, 240u8],
            [222u8, 112u8, 224u8, 184u8],
            [223u8, 46u8, 189u8, 187u8],
            [223u8, 251u8, 221u8, 159u8],
            [239u8, 11u8, 165u8, 208u8],
            [242u8, 110u8, 233u8, 208u8],
            [242u8, 253u8, 227u8, 139u8],
            [249u8, 236u8, 208u8, 30u8],
            [250u8, 188u8, 28u8, 188u8],
            [255u8, 43u8, 174u8, 134u8],
            [255u8, 234u8, 99u8, 43u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for RolldownCalls {
        const NAME: &'static str = "RolldownCalls";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 44usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::CLOSED(_) => <CLOSEDCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::NATIVE_TOKEN_ADDRESS(_) => {
                    <NATIVE_TOKEN_ADDRESSCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::calculate_root(_) => {
                    <calculate_rootCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::calculate_root_impl(_) => {
                    <calculate_root_implCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::cancelResolutions(_) => {
                    <cancelResolutionsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::chain(_) => <chainCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::close_cancel(_) => {
                    <close_cancelCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::close_deposit_refund(_) => {
                    <close_deposit_refundCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::close_withdrawal(_) => {
                    <close_withdrawalCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::counter(_) => <counterCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::deposit_0(_) => {
                    <deposit_0Call as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::deposit_1(_) => {
                    <deposit_1Call as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::deposit_erc20_0(_) => {
                    <deposit_erc20_0Call as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::deposit_erc20_1(_) => {
                    <deposit_erc20_1Call as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::deposit_native_0(_) => {
                    <deposit_native_0Call as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::deposit_native_1(_) => {
                    <deposit_native_1Call as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::deposits(_) => <depositsCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::ferry_withdrawal(_) => {
                    <ferry_withdrawalCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::find_l2_batch(_) => {
                    <find_l2_batchCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getMerkleRootsLength(_) => {
                    <getMerkleRootsLengthCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getPendingRequests(_) => {
                    <getPendingRequestsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getUpdateForL2(_) => {
                    <getUpdateForL2Call as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::hashCancel(_) => {
                    <hashCancelCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::hashFailedDepositResolution(_) => {
                    <hashFailedDepositResolutionCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::hashWithdrawal(_) => {
                    <hashWithdrawalCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::initialize(_) => {
                    <initializeCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::lastProcessedUpdate_origin_l1(_) => {
                    <lastProcessedUpdate_origin_l1Call as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::lastProcessedUpdate_origin_l2(_) => {
                    <lastProcessedUpdate_origin_l2Call as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::merkleRootRange(_) => {
                    <merkleRootRangeCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::owner(_) => <ownerCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::pause(_) => <pauseCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::pauseAll(_) => <pauseAllCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::paused_0(_) => <paused_0Call as alloy_sol_types::SolCall>::SELECTOR,
                Self::paused_1(_) => <paused_1Call as alloy_sol_types::SolCall>::SELECTOR,
                Self::pauserRegistry(_) => {
                    <pauserRegistryCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::processedL2Requests(_) => {
                    <processedL2RequestsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::renounceOwnership(_) => {
                    <renounceOwnershipCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::roots(_) => <rootsCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::setPauserRegistry(_) => {
                    <setPauserRegistryCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::setUpdater(_) => {
                    <setUpdaterCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::transferOwnership(_) => {
                    <transferOwnershipCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::unpause(_) => <unpauseCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::update_l1_from_l2(_) => {
                    <update_l1_from_l2Call as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::updaterAccount(_) => {
                    <updaterAccountCall as alloy_sol_types::SolCall>::SELECTOR
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
        #[allow(unsafe_code, non_snake_case)]
        fn abi_decode_raw(
            selector: [u8; 4],
            data: &[u8],
            validate: bool,
        ) -> alloy_sol_types::Result<Self> {
            static DECODE_SHIMS: &[fn(
                &[u8],
                bool,
            ) -> alloy_sol_types::Result<RolldownCalls>] = &[
                {
                    fn close_cancel(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RolldownCalls> {
                        <close_cancelCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(RolldownCalls::close_cancel)
                    }
                    close_cancel
                },
                {
                    fn deposit_erc20_0(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RolldownCalls> {
                        <deposit_erc20_0Call as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(RolldownCalls::deposit_erc20_0)
                    }
                    deposit_erc20_0
                },
                {
                    fn update_l1_from_l2(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RolldownCalls> {
                        <update_l1_from_l2Call as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(RolldownCalls::update_l1_from_l2)
                    }
                    update_l1_from_l2
                },
                {
                    fn ferry_withdrawal(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RolldownCalls> {
                        <ferry_withdrawalCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(RolldownCalls::ferry_withdrawal)
                    }
                    ferry_withdrawal
                },
                {
                    fn CLOSED(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RolldownCalls> {
                        <CLOSEDCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(RolldownCalls::CLOSED)
                    }
                    CLOSED
                },
                {
                    fn deposit_0(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RolldownCalls> {
                        <deposit_0Call as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(RolldownCalls::deposit_0)
                    }
                    deposit_0
                },
                {
                    fn setPauserRegistry(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RolldownCalls> {
                        <setPauserRegistryCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(RolldownCalls::setPauserRegistry)
                    }
                    setPauserRegistry
                },
                {
                    fn pause(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RolldownCalls> {
                        <pauseCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(RolldownCalls::pause)
                    }
                    pause
                },
                {
                    fn deposit_1(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RolldownCalls> {
                        <deposit_1Call as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(RolldownCalls::deposit_1)
                    }
                    deposit_1
                },
                {
                    fn close_withdrawal(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RolldownCalls> {
                        <close_withdrawalCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(RolldownCalls::close_withdrawal)
                    }
                    close_withdrawal
                },
                {
                    fn merkleRootRange(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RolldownCalls> {
                        <merkleRootRangeCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(RolldownCalls::merkleRootRange)
                    }
                    merkleRootRange
                },
                {
                    fn pauseAll(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RolldownCalls> {
                        <pauseAllCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(RolldownCalls::pauseAll)
                    }
                    pauseAll
                },
                {
                    fn paused_0(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RolldownCalls> {
                        <paused_0Call as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(RolldownCalls::paused_0)
                    }
                    paused_0
                },
                {
                    fn paused_1(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RolldownCalls> {
                        <paused_1Call as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(RolldownCalls::paused_1)
                    }
                    paused_1
                },
                {
                    fn counter(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RolldownCalls> {
                        <counterCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(RolldownCalls::counter)
                    }
                    counter
                },
                {
                    fn renounceOwnership(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RolldownCalls> {
                        <renounceOwnershipCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(RolldownCalls::renounceOwnership)
                    }
                    renounceOwnership
                },
                {
                    fn updaterAccount(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RolldownCalls> {
                        <updaterAccountCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(RolldownCalls::updaterAccount)
                    }
                    updaterAccount
                },
                {
                    fn getPendingRequests(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RolldownCalls> {
                        <getPendingRequestsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(RolldownCalls::getPendingRequests)
                    }
                    getPendingRequests
                },
                {
                    fn lastProcessedUpdate_origin_l1(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RolldownCalls> {
                        <lastProcessedUpdate_origin_l1Call as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(RolldownCalls::lastProcessedUpdate_origin_l1)
                    }
                    lastProcessedUpdate_origin_l1
                },
                {
                    fn pauserRegistry(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RolldownCalls> {
                        <pauserRegistryCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(RolldownCalls::pauserRegistry)
                    }
                    pauserRegistry
                },
                {
                    fn hashWithdrawal(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RolldownCalls> {
                        <hashWithdrawalCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(RolldownCalls::hashWithdrawal)
                    }
                    hashWithdrawal
                },
                {
                    fn owner(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RolldownCalls> {
                        <ownerCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(RolldownCalls::owner)
                    }
                    owner
                },
                {
                    fn close_deposit_refund(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RolldownCalls> {
                        <close_deposit_refundCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(RolldownCalls::close_deposit_refund)
                    }
                    close_deposit_refund
                },
                {
                    fn setUpdater(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RolldownCalls> {
                        <setUpdaterCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(RolldownCalls::setUpdater)
                    }
                    setUpdater
                },
                {
                    fn hashFailedDepositResolution(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RolldownCalls> {
                        <hashFailedDepositResolutionCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(RolldownCalls::hashFailedDepositResolution)
                    }
                    hashFailedDepositResolution
                },
                {
                    fn calculate_root(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RolldownCalls> {
                        <calculate_rootCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(RolldownCalls::calculate_root)
                    }
                    calculate_root
                },
                {
                    fn deposits(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RolldownCalls> {
                        <depositsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(RolldownCalls::deposits)
                    }
                    deposits
                },
                {
                    fn getUpdateForL2(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RolldownCalls> {
                        <getUpdateForL2Call as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(RolldownCalls::getUpdateForL2)
                    }
                    getUpdateForL2
                },
                {
                    fn roots(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RolldownCalls> {
                        <rootsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(RolldownCalls::roots)
                    }
                    roots
                },
                {
                    fn chain(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RolldownCalls> {
                        <chainCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(RolldownCalls::chain)
                    }
                    chain
                },
                {
                    fn deposit_native_0(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RolldownCalls> {
                        <deposit_native_0Call as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(RolldownCalls::deposit_native_0)
                    }
                    deposit_native_0
                },
                {
                    fn cancelResolutions(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RolldownCalls> {
                        <cancelResolutionsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(RolldownCalls::cancelResolutions)
                    }
                    cancelResolutions
                },
                {
                    fn hashCancel(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RolldownCalls> {
                        <hashCancelCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(RolldownCalls::hashCancel)
                    }
                    hashCancel
                },
                {
                    fn deposit_erc20_1(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RolldownCalls> {
                        <deposit_erc20_1Call as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(RolldownCalls::deposit_erc20_1)
                    }
                    deposit_erc20_1
                },
                {
                    fn processedL2Requests(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RolldownCalls> {
                        <processedL2RequestsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(RolldownCalls::processedL2Requests)
                    }
                    processedL2Requests
                },
                {
                    fn NATIVE_TOKEN_ADDRESS(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RolldownCalls> {
                        <NATIVE_TOKEN_ADDRESSCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(RolldownCalls::NATIVE_TOKEN_ADDRESS)
                    }
                    NATIVE_TOKEN_ADDRESS
                },
                {
                    fn deposit_native_1(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RolldownCalls> {
                        <deposit_native_1Call as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(RolldownCalls::deposit_native_1)
                    }
                    deposit_native_1
                },
                {
                    fn calculate_root_impl(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RolldownCalls> {
                        <calculate_root_implCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(RolldownCalls::calculate_root_impl)
                    }
                    calculate_root_impl
                },
                {
                    fn lastProcessedUpdate_origin_l2(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RolldownCalls> {
                        <lastProcessedUpdate_origin_l2Call as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(RolldownCalls::lastProcessedUpdate_origin_l2)
                    }
                    lastProcessedUpdate_origin_l2
                },
                {
                    fn transferOwnership(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RolldownCalls> {
                        <transferOwnershipCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(RolldownCalls::transferOwnership)
                    }
                    transferOwnership
                },
                {
                    fn find_l2_batch(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RolldownCalls> {
                        <find_l2_batchCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(RolldownCalls::find_l2_batch)
                    }
                    find_l2_batch
                },
                {
                    fn unpause(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RolldownCalls> {
                        <unpauseCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(RolldownCalls::unpause)
                    }
                    unpause
                },
                {
                    fn getMerkleRootsLength(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RolldownCalls> {
                        <getMerkleRootsLengthCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(RolldownCalls::getMerkleRootsLength)
                    }
                    getMerkleRootsLength
                },
                {
                    fn initialize(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RolldownCalls> {
                        <initializeCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(RolldownCalls::initialize)
                    }
                    initialize
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
            (unsafe { DECODE_SHIMS.get_unchecked(idx) })(data, validate)
        }
        #[inline]
        fn abi_encoded_size(&self) -> usize {
            match self {
                Self::CLOSED(inner) => {
                    <CLOSEDCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::NATIVE_TOKEN_ADDRESS(inner) => {
                    <NATIVE_TOKEN_ADDRESSCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::calculate_root(inner) => {
                    <calculate_rootCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::calculate_root_impl(inner) => {
                    <calculate_root_implCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::cancelResolutions(inner) => {
                    <cancelResolutionsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::chain(inner) => {
                    <chainCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::close_cancel(inner) => {
                    <close_cancelCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::close_deposit_refund(inner) => {
                    <close_deposit_refundCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::close_withdrawal(inner) => {
                    <close_withdrawalCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::counter(inner) => {
                    <counterCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::deposit_0(inner) => {
                    <deposit_0Call as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::deposit_1(inner) => {
                    <deposit_1Call as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::deposit_erc20_0(inner) => {
                    <deposit_erc20_0Call as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::deposit_erc20_1(inner) => {
                    <deposit_erc20_1Call as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::deposit_native_0(inner) => {
                    <deposit_native_0Call as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::deposit_native_1(inner) => {
                    <deposit_native_1Call as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::deposits(inner) => {
                    <depositsCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::ferry_withdrawal(inner) => {
                    <ferry_withdrawalCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::find_l2_batch(inner) => {
                    <find_l2_batchCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getMerkleRootsLength(inner) => {
                    <getMerkleRootsLengthCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getPendingRequests(inner) => {
                    <getPendingRequestsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getUpdateForL2(inner) => {
                    <getUpdateForL2Call as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::hashCancel(inner) => {
                    <hashCancelCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::hashFailedDepositResolution(inner) => {
                    <hashFailedDepositResolutionCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::hashWithdrawal(inner) => {
                    <hashWithdrawalCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::initialize(inner) => {
                    <initializeCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::lastProcessedUpdate_origin_l1(inner) => {
                    <lastProcessedUpdate_origin_l1Call as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::lastProcessedUpdate_origin_l2(inner) => {
                    <lastProcessedUpdate_origin_l2Call as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::merkleRootRange(inner) => {
                    <merkleRootRangeCall as alloy_sol_types::SolCall>::abi_encoded_size(
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
                Self::processedL2Requests(inner) => {
                    <processedL2RequestsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::renounceOwnership(inner) => {
                    <renounceOwnershipCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::roots(inner) => {
                    <rootsCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::setPauserRegistry(inner) => {
                    <setPauserRegistryCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::setUpdater(inner) => {
                    <setUpdaterCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::transferOwnership(inner) => {
                    <transferOwnershipCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::unpause(inner) => {
                    <unpauseCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::update_l1_from_l2(inner) => {
                    <update_l1_from_l2Call as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::updaterAccount(inner) => {
                    <updaterAccountCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
            }
        }
        #[inline]
        fn abi_encode_raw(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
            match self {
                Self::CLOSED(inner) => {
                    <CLOSEDCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::NATIVE_TOKEN_ADDRESS(inner) => {
                    <NATIVE_TOKEN_ADDRESSCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::calculate_root(inner) => {
                    <calculate_rootCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::calculate_root_impl(inner) => {
                    <calculate_root_implCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::cancelResolutions(inner) => {
                    <cancelResolutionsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::chain(inner) => {
                    <chainCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::close_cancel(inner) => {
                    <close_cancelCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::close_deposit_refund(inner) => {
                    <close_deposit_refundCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::close_withdrawal(inner) => {
                    <close_withdrawalCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::counter(inner) => {
                    <counterCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::deposit_0(inner) => {
                    <deposit_0Call as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::deposit_1(inner) => {
                    <deposit_1Call as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::deposit_erc20_0(inner) => {
                    <deposit_erc20_0Call as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::deposit_erc20_1(inner) => {
                    <deposit_erc20_1Call as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::deposit_native_0(inner) => {
                    <deposit_native_0Call as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::deposit_native_1(inner) => {
                    <deposit_native_1Call as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::deposits(inner) => {
                    <depositsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::ferry_withdrawal(inner) => {
                    <ferry_withdrawalCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::find_l2_batch(inner) => {
                    <find_l2_batchCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getMerkleRootsLength(inner) => {
                    <getMerkleRootsLengthCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getPendingRequests(inner) => {
                    <getPendingRequestsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getUpdateForL2(inner) => {
                    <getUpdateForL2Call as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::hashCancel(inner) => {
                    <hashCancelCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::hashFailedDepositResolution(inner) => {
                    <hashFailedDepositResolutionCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::hashWithdrawal(inner) => {
                    <hashWithdrawalCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::lastProcessedUpdate_origin_l1(inner) => {
                    <lastProcessedUpdate_origin_l1Call as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::lastProcessedUpdate_origin_l2(inner) => {
                    <lastProcessedUpdate_origin_l2Call as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::merkleRootRange(inner) => {
                    <merkleRootRangeCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::processedL2Requests(inner) => {
                    <processedL2RequestsCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::roots(inner) => {
                    <rootsCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::setPauserRegistry(inner) => {
                    <setPauserRegistryCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::setUpdater(inner) => {
                    <setUpdaterCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::update_l1_from_l2(inner) => {
                    <update_l1_from_l2Call as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::updaterAccount(inner) => {
                    <updaterAccountCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
            }
        }
    }
    ///Container for all the [`Rolldown`](self) events.
    pub enum RolldownEvents {
        DepositAcceptedIntoQueue(DepositAcceptedIntoQueue),
        DisputeResolutionAcceptedIntoQueue(DisputeResolutionAcceptedIntoQueue),
        ERC20TokensWithdrawn(ERC20TokensWithdrawn),
        FailedDepositResolutionClosed(FailedDepositResolutionClosed),
        FerriedWithdrawalClosed(FerriedWithdrawalClosed),
        Initialized(Initialized),
        L2UpdateAccepted(L2UpdateAccepted),
        NativeTokensWithdrawn(NativeTokensWithdrawn),
        NewUpdaterSet(NewUpdaterSet),
        OwnershipTransferred(OwnershipTransferred),
        Paused(Paused),
        PauserRegistrySet(PauserRegistrySet),
        Unpaused(Unpaused),
        WithdrawalClosed(WithdrawalClosed),
        WithdrawalFerried(WithdrawalFerried),
    }
    #[automatically_derived]
    impl RolldownEvents {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 32usize]] = &[
            [
                0u8,
                231u8,
                99u8,
                247u8,
                119u8,
                139u8,
                140u8,
                238u8,
                247u8,
                39u8,
                12u8,
                137u8,
                183u8,
                209u8,
                223u8,
                16u8,
                8u8,
                176u8,
                228u8,
                130u8,
                218u8,
                57u8,
                196u8,
                56u8,
                49u8,
                65u8,
                119u8,
                51u8,
                175u8,
                150u8,
                251u8,
                13u8,
            ],
            [
                19u8,
                117u8,
                12u8,
                115u8,
                31u8,
                135u8,
                193u8,
                82u8,
                66u8,
                135u8,
                76u8,
                231u8,
                75u8,
                244u8,
                100u8,
                149u8,
                2u8,
                204u8,
                142u8,
                124u8,
                130u8,
                144u8,
                103u8,
                206u8,
                132u8,
                101u8,
                5u8,
                172u8,
                219u8,
                150u8,
                40u8,
                157u8,
            ],
            [
                27u8,
                15u8,
                47u8,
                80u8,
                13u8,
                245u8,
                150u8,
                180u8,
                43u8,
                115u8,
                232u8,
                13u8,
                190u8,
                198u8,
                161u8,
                251u8,
                87u8,
                15u8,
                1u8,
                151u8,
                138u8,
                88u8,
                103u8,
                35u8,
                249u8,
                136u8,
                165u8,
                252u8,
                84u8,
                215u8,
                115u8,
                161u8,
            ],
            [
                34u8,
                83u8,
                5u8,
                236u8,
                182u8,
                111u8,
                169u8,
                185u8,
                178u8,
                159u8,
                141u8,
                234u8,
                217u8,
                186u8,
                234u8,
                54u8,
                90u8,
                108u8,
                34u8,
                93u8,
                99u8,
                157u8,
                253u8,
                134u8,
                110u8,
                120u8,
                44u8,
                203u8,
                99u8,
                226u8,
                240u8,
                91u8,
            ],
            [
                41u8,
                150u8,
                253u8,
                84u8,
                108u8,
                55u8,
                215u8,
                76u8,
                23u8,
                4u8,
                102u8,
                234u8,
                106u8,
                164u8,
                163u8,
                8u8,
                227u8,
                202u8,
                45u8,
                74u8,
                166u8,
                137u8,
                230u8,
                233u8,
                227u8,
                41u8,
                148u8,
                219u8,
                80u8,
                57u8,
                204u8,
                14u8,
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
                73u8,
                193u8,
                88u8,
                212u8,
                144u8,
                219u8,
                158u8,
                6u8,
                111u8,
                1u8,
                181u8,
                212u8,
                241u8,
                160u8,
                148u8,
                72u8,
                90u8,
                101u8,
                152u8,
                203u8,
                108u8,
                82u8,
                150u8,
                180u8,
                192u8,
                126u8,
                70u8,
                193u8,
                42u8,
                29u8,
                193u8,
                28u8,
            ],
            [
                110u8,
                159u8,
                205u8,
                83u8,
                152u8,
                150u8,
                252u8,
                166u8,
                14u8,
                139u8,
                15u8,
                1u8,
                221u8,
                88u8,
                2u8,
                51u8,
                228u8,
                138u8,
                107u8,
                15u8,
                125u8,
                240u8,
                19u8,
                184u8,
                155u8,
                167u8,
                245u8,
                101u8,
                134u8,
                154u8,
                205u8,
                182u8,
            ],
            [
                122u8,
                154u8,
                189u8,
                158u8,
                184u8,
                107u8,
                219u8,
                202u8,
                137u8,
                203u8,
                164u8,
                6u8,
                154u8,
                99u8,
                44u8,
                55u8,
                217u8,
                61u8,
                184u8,
                46u8,
                62u8,
                20u8,
                173u8,
                129u8,
                25u8,
                163u8,
                167u8,
                129u8,
                40u8,
                20u8,
                133u8,
                62u8,
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
                147u8,
                95u8,
                38u8,
                217u8,
                75u8,
                227u8,
                25u8,
                7u8,
                8u8,
                10u8,
                167u8,
                139u8,
                62u8,
                110u8,
                42u8,
                198u8,
                212u8,
                138u8,
                7u8,
                42u8,
                240u8,
                150u8,
                194u8,
                2u8,
                104u8,
                56u8,
                134u8,
                33u8,
                187u8,
                193u8,
                23u8,
                137u8,
            ],
            [
                158u8,
                241u8,
                19u8,
                83u8,
                175u8,
                217u8,
                125u8,
                51u8,
                154u8,
                119u8,
                115u8,
                40u8,
                80u8,
                183u8,
                194u8,
                39u8,
                4u8,
                101u8,
                101u8,
                88u8,
                217u8,
                186u8,
                99u8,
                204u8,
                126u8,
                50u8,
                30u8,
                10u8,
                196u8,
                194u8,
                10u8,
                169u8,
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
                224u8,
                73u8,
                83u8,
                85u8,
                193u8,
                224u8,
                76u8,
                81u8,
                37u8,
                132u8,
                82u8,
                24u8,
                84u8,
                210u8,
                34u8,
                210u8,
                57u8,
                164u8,
                183u8,
                130u8,
                179u8,
                154u8,
                200u8,
                167u8,
                232u8,
                53u8,
                163u8,
                79u8,
                94u8,
                199u8,
                193u8,
                225u8,
            ],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolEventInterface for RolldownEvents {
        const NAME: &'static str = "RolldownEvents";
        const COUNT: usize = 15usize;
        fn decode_raw_log(
            topics: &[alloy_sol_types::Word],
            data: &[u8],
            validate: bool,
        ) -> alloy_sol_types::Result<Self> {
            match topics.first().copied() {
                Some(
                    <DepositAcceptedIntoQueue as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <DepositAcceptedIntoQueue as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::DepositAcceptedIntoQueue)
                }
                Some(
                    <DisputeResolutionAcceptedIntoQueue as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <DisputeResolutionAcceptedIntoQueue as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::DisputeResolutionAcceptedIntoQueue)
                }
                Some(
                    <ERC20TokensWithdrawn as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <ERC20TokensWithdrawn as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::ERC20TokensWithdrawn)
                }
                Some(
                    <FailedDepositResolutionClosed as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <FailedDepositResolutionClosed as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::FailedDepositResolutionClosed)
                }
                Some(
                    <FerriedWithdrawalClosed as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <FerriedWithdrawalClosed as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::FerriedWithdrawalClosed)
                }
                Some(<Initialized as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <Initialized as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::Initialized)
                }
                Some(<L2UpdateAccepted as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <L2UpdateAccepted as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::L2UpdateAccepted)
                }
                Some(
                    <NativeTokensWithdrawn as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <NativeTokensWithdrawn as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::NativeTokensWithdrawn)
                }
                Some(<NewUpdaterSet as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <NewUpdaterSet as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::NewUpdaterSet)
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
                    <PauserRegistrySet as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <PauserRegistrySet as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::PauserRegistrySet)
                }
                Some(<Unpaused as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <Unpaused as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::Unpaused)
                }
                Some(<WithdrawalClosed as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <WithdrawalClosed as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::WithdrawalClosed)
                }
                Some(
                    <WithdrawalFerried as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <WithdrawalFerried as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::WithdrawalFerried)
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
    impl alloy_sol_types::private::IntoLogData for RolldownEvents {
        fn to_log_data(&self) -> alloy_sol_types::private::LogData {
            match self {
                Self::DepositAcceptedIntoQueue(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::DisputeResolutionAcceptedIntoQueue(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::ERC20TokensWithdrawn(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::FailedDepositResolutionClosed(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::FerriedWithdrawalClosed(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::Initialized(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::L2UpdateAccepted(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::NativeTokensWithdrawn(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::NewUpdaterSet(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::OwnershipTransferred(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::Paused(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::PauserRegistrySet(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::Unpaused(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::WithdrawalClosed(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::WithdrawalFerried(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
            }
        }
        fn into_log_data(self) -> alloy_sol_types::private::LogData {
            match self {
                Self::DepositAcceptedIntoQueue(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::DisputeResolutionAcceptedIntoQueue(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::ERC20TokensWithdrawn(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::FailedDepositResolutionClosed(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::FerriedWithdrawalClosed(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::Initialized(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::L2UpdateAccepted(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::NativeTokensWithdrawn(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::NewUpdaterSet(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::OwnershipTransferred(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::Paused(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::PauserRegistrySet(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::Unpaused(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::WithdrawalClosed(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::WithdrawalFerried(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
            }
        }
    }
    use alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`Rolldown`](self) contract instance.

See the [wrapper's documentation](`RolldownInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> RolldownInstance<T, P, N> {
        RolldownInstance::<T, P, N>::new(address, provider)
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
        Output = alloy_contract::Result<RolldownInstance<T, P, N>>,
    > {
        RolldownInstance::<T, P, N>::deploy(provider)
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
        RolldownInstance::<T, P, N>::deploy_builder(provider)
    }
    /**A [`Rolldown`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`Rolldown`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct RolldownInstance<T, P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for RolldownInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("RolldownInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > RolldownInstance<T, P, N> {
        /**Creates a new wrapper around an on-chain [`Rolldown`](self) contract instance.

See the [wrapper's documentation](`RolldownInstance`) for more details.*/
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
        ) -> alloy_contract::Result<RolldownInstance<T, P, N>> {
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
    impl<T, P: ::core::clone::Clone, N> RolldownInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> RolldownInstance<T, P, N> {
            RolldownInstance {
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
    > RolldownInstance<T, P, N> {
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
        ///Creates a new call builder for the [`CLOSED`] function.
        pub fn CLOSED(&self) -> alloy_contract::SolCallBuilder<T, &P, CLOSEDCall, N> {
            self.call_builder(&CLOSEDCall {})
        }
        ///Creates a new call builder for the [`NATIVE_TOKEN_ADDRESS`] function.
        pub fn NATIVE_TOKEN_ADDRESS(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, NATIVE_TOKEN_ADDRESSCall, N> {
            self.call_builder(&NATIVE_TOKEN_ADDRESSCall {})
        }
        ///Creates a new call builder for the [`calculate_root`] function.
        pub fn calculate_root(
            &self,
            leave_hash: alloy::sol_types::private::FixedBytes<32>,
            leave_idx: u32,
            proof: alloy::sol_types::private::Vec<
                alloy::sol_types::private::FixedBytes<32>,
            >,
            leaves_count: u32,
        ) -> alloy_contract::SolCallBuilder<T, &P, calculate_rootCall, N> {
            self.call_builder(
                &calculate_rootCall {
                    leave_hash,
                    leave_idx,
                    proof,
                    leaves_count,
                },
            )
        }
        ///Creates a new call builder for the [`calculate_root_impl`] function.
        pub fn calculate_root_impl(
            &self,
            level: u32,
            pos: u32,
            hash: alloy::sol_types::private::FixedBytes<32>,
            proofs: alloy::sol_types::private::Vec<
                alloy::sol_types::private::FixedBytes<32>,
            >,
            proof_idx: u32,
            max_index: u32,
        ) -> alloy_contract::SolCallBuilder<T, &P, calculate_root_implCall, N> {
            self.call_builder(
                &calculate_root_implCall {
                    level,
                    pos,
                    hash,
                    proofs,
                    proof_idx,
                    max_index,
                },
            )
        }
        ///Creates a new call builder for the [`cancelResolutions`] function.
        pub fn cancelResolutions(
            &self,
            _0: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, cancelResolutionsCall, N> {
            self.call_builder(&cancelResolutionsCall { _0 })
        }
        ///Creates a new call builder for the [`chain`] function.
        pub fn chain(&self) -> alloy_contract::SolCallBuilder<T, &P, chainCall, N> {
            self.call_builder(&chainCall {})
        }
        ///Creates a new call builder for the [`close_cancel`] function.
        pub fn close_cancel(
            &self,
            cancel: <IRolldownPrimitives::Cancel as alloy::sol_types::SolType>::RustType,
            merkle_root: alloy::sol_types::private::FixedBytes<32>,
            proof: alloy::sol_types::private::Vec<
                alloy::sol_types::private::FixedBytes<32>,
            >,
        ) -> alloy_contract::SolCallBuilder<T, &P, close_cancelCall, N> {
            self.call_builder(
                &close_cancelCall {
                    cancel,
                    merkle_root,
                    proof,
                },
            )
        }
        ///Creates a new call builder for the [`close_deposit_refund`] function.
        pub fn close_deposit_refund(
            &self,
            failedDeposit: <IRolldownPrimitives::FailedDepositResolution as alloy::sol_types::SolType>::RustType,
            merkle_root: alloy::sol_types::private::FixedBytes<32>,
            proof: alloy::sol_types::private::Vec<
                alloy::sol_types::private::FixedBytes<32>,
            >,
        ) -> alloy_contract::SolCallBuilder<T, &P, close_deposit_refundCall, N> {
            self.call_builder(
                &close_deposit_refundCall {
                    failedDeposit,
                    merkle_root,
                    proof,
                },
            )
        }
        ///Creates a new call builder for the [`close_withdrawal`] function.
        pub fn close_withdrawal(
            &self,
            withdrawal: <IRolldownPrimitives::Withdrawal as alloy::sol_types::SolType>::RustType,
            merkle_root: alloy::sol_types::private::FixedBytes<32>,
            proof: alloy::sol_types::private::Vec<
                alloy::sol_types::private::FixedBytes<32>,
            >,
        ) -> alloy_contract::SolCallBuilder<T, &P, close_withdrawalCall, N> {
            self.call_builder(
                &close_withdrawalCall {
                    withdrawal,
                    merkle_root,
                    proof,
                },
            )
        }
        ///Creates a new call builder for the [`counter`] function.
        pub fn counter(&self) -> alloy_contract::SolCallBuilder<T, &P, counterCall, N> {
            self.call_builder(&counterCall {})
        }
        ///Creates a new call builder for the [`deposit_0`] function.
        pub fn deposit_0(
            &self,
            tokenAddress: alloy::sol_types::private::Address,
            amount: alloy::sol_types::private::primitives::aliases::U256,
            ferryTip: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, deposit_0Call, N> {
            self.call_builder(
                &deposit_0Call {
                    tokenAddress,
                    amount,
                    ferryTip,
                },
            )
        }
        ///Creates a new call builder for the [`deposit_1`] function.
        pub fn deposit_1(
            &self,
            tokenAddress: alloy::sol_types::private::Address,
            amount: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, deposit_1Call, N> {
            self.call_builder(
                &deposit_1Call {
                    tokenAddress,
                    amount,
                },
            )
        }
        ///Creates a new call builder for the [`deposit_erc20_0`] function.
        pub fn deposit_erc20_0(
            &self,
            tokenAddress: alloy::sol_types::private::Address,
            amount: alloy::sol_types::private::primitives::aliases::U256,
            ferryTip: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, deposit_erc20_0Call, N> {
            self.call_builder(
                &deposit_erc20_0Call {
                    tokenAddress,
                    amount,
                    ferryTip,
                },
            )
        }
        ///Creates a new call builder for the [`deposit_erc20_1`] function.
        pub fn deposit_erc20_1(
            &self,
            tokenAddress: alloy::sol_types::private::Address,
            amount: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, deposit_erc20_1Call, N> {
            self.call_builder(
                &deposit_erc20_1Call {
                    tokenAddress,
                    amount,
                },
            )
        }
        ///Creates a new call builder for the [`deposit_native_0`] function.
        pub fn deposit_native_0(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, deposit_native_0Call, N> {
            self.call_builder(&deposit_native_0Call {})
        }
        ///Creates a new call builder for the [`deposit_native_1`] function.
        pub fn deposit_native_1(
            &self,
            ferryTip: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, deposit_native_1Call, N> {
            self.call_builder(&deposit_native_1Call { ferryTip })
        }
        ///Creates a new call builder for the [`deposits`] function.
        pub fn deposits(
            &self,
            _0: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, depositsCall, N> {
            self.call_builder(&depositsCall { _0 })
        }
        ///Creates a new call builder for the [`ferry_withdrawal`] function.
        pub fn ferry_withdrawal(
            &self,
            withdrawal: <IRolldownPrimitives::Withdrawal as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<T, &P, ferry_withdrawalCall, N> {
            self.call_builder(&ferry_withdrawalCall { withdrawal })
        }
        ///Creates a new call builder for the [`find_l2_batch`] function.
        pub fn find_l2_batch(
            &self,
            requestId: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, find_l2_batchCall, N> {
            self.call_builder(&find_l2_batchCall { requestId })
        }
        ///Creates a new call builder for the [`getMerkleRootsLength`] function.
        pub fn getMerkleRootsLength(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, getMerkleRootsLengthCall, N> {
            self.call_builder(&getMerkleRootsLengthCall {})
        }
        ///Creates a new call builder for the [`getPendingRequests`] function.
        pub fn getPendingRequests(
            &self,
            start: alloy::sol_types::private::primitives::aliases::U256,
            end: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, getPendingRequestsCall, N> {
            self.call_builder(
                &getPendingRequestsCall {
                    start,
                    end,
                },
            )
        }
        ///Creates a new call builder for the [`getUpdateForL2`] function.
        pub fn getUpdateForL2(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, getUpdateForL2Call, N> {
            self.call_builder(&getUpdateForL2Call {})
        }
        ///Creates a new call builder for the [`hashCancel`] function.
        pub fn hashCancel(
            &self,
            cancel: <IRolldownPrimitives::Cancel as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<T, &P, hashCancelCall, N> {
            self.call_builder(&hashCancelCall { cancel })
        }
        ///Creates a new call builder for the [`hashFailedDepositResolution`] function.
        pub fn hashFailedDepositResolution(
            &self,
            failedDeposit: <IRolldownPrimitives::FailedDepositResolution as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<T, &P, hashFailedDepositResolutionCall, N> {
            self.call_builder(
                &hashFailedDepositResolutionCall {
                    failedDeposit,
                },
            )
        }
        ///Creates a new call builder for the [`hashWithdrawal`] function.
        pub fn hashWithdrawal(
            &self,
            withdrawal: <IRolldownPrimitives::Withdrawal as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<T, &P, hashWithdrawalCall, N> {
            self.call_builder(&hashWithdrawalCall { withdrawal })
        }
        ///Creates a new call builder for the [`initialize`] function.
        pub fn initialize(
            &self,
            _pauserRegistry: alloy::sol_types::private::Address,
            initialOwner: alloy::sol_types::private::Address,
            chainId: <IRolldownPrimitives::ChainId as alloy::sol_types::SolType>::RustType,
            updater: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, initializeCall, N> {
            self.call_builder(
                &initializeCall {
                    _pauserRegistry,
                    initialOwner,
                    chainId,
                    updater,
                },
            )
        }
        ///Creates a new call builder for the [`lastProcessedUpdate_origin_l1`] function.
        pub fn lastProcessedUpdate_origin_l1(
            &self,
        ) -> alloy_contract::SolCallBuilder<
            T,
            &P,
            lastProcessedUpdate_origin_l1Call,
            N,
        > {
            self.call_builder(
                &lastProcessedUpdate_origin_l1Call {
                },
            )
        }
        ///Creates a new call builder for the [`lastProcessedUpdate_origin_l2`] function.
        pub fn lastProcessedUpdate_origin_l2(
            &self,
        ) -> alloy_contract::SolCallBuilder<
            T,
            &P,
            lastProcessedUpdate_origin_l2Call,
            N,
        > {
            self.call_builder(
                &lastProcessedUpdate_origin_l2Call {
                },
            )
        }
        ///Creates a new call builder for the [`merkleRootRange`] function.
        pub fn merkleRootRange(
            &self,
            _0: alloy::sol_types::private::FixedBytes<32>,
        ) -> alloy_contract::SolCallBuilder<T, &P, merkleRootRangeCall, N> {
            self.call_builder(&merkleRootRangeCall { _0 })
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
        ///Creates a new call builder for the [`processedL2Requests`] function.
        pub fn processedL2Requests(
            &self,
            _0: alloy::sol_types::private::FixedBytes<32>,
        ) -> alloy_contract::SolCallBuilder<T, &P, processedL2RequestsCall, N> {
            self.call_builder(&processedL2RequestsCall { _0 })
        }
        ///Creates a new call builder for the [`renounceOwnership`] function.
        pub fn renounceOwnership(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, renounceOwnershipCall, N> {
            self.call_builder(&renounceOwnershipCall {})
        }
        ///Creates a new call builder for the [`roots`] function.
        pub fn roots(
            &self,
            _0: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, rootsCall, N> {
            self.call_builder(&rootsCall { _0 })
        }
        ///Creates a new call builder for the [`setPauserRegistry`] function.
        pub fn setPauserRegistry(
            &self,
            newPauserRegistry: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, setPauserRegistryCall, N> {
            self.call_builder(
                &setPauserRegistryCall {
                    newPauserRegistry,
                },
            )
        }
        ///Creates a new call builder for the [`setUpdater`] function.
        pub fn setUpdater(
            &self,
            updater: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, setUpdaterCall, N> {
            self.call_builder(&setUpdaterCall { updater })
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
        ///Creates a new call builder for the [`update_l1_from_l2`] function.
        pub fn update_l1_from_l2(
            &self,
            merkle_root: alloy::sol_types::private::FixedBytes<32>,
            range: <IRolldownPrimitives::Range as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<T, &P, update_l1_from_l2Call, N> {
            self.call_builder(
                &update_l1_from_l2Call {
                    merkle_root,
                    range,
                },
            )
        }
        ///Creates a new call builder for the [`updaterAccount`] function.
        pub fn updaterAccount(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, updaterAccountCall, N> {
            self.call_builder(&updaterAccountCall {})
        }
    }
    /// Event filters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > RolldownInstance<T, P, N> {
        /// Creates a new event filter using this contract instance's provider and address.
        ///
        /// Note that the type can be any event, not just those defined in this contract.
        /// Prefer using the other methods for building type-safe event filters.
        pub fn event_filter<E: alloy_sol_types::SolEvent>(
            &self,
        ) -> alloy_contract::Event<T, &P, E, N> {
            alloy_contract::Event::new_sol(&self.provider, &self.address)
        }
        ///Creates a new event filter for the [`DepositAcceptedIntoQueue`] event.
        pub fn DepositAcceptedIntoQueue_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, DepositAcceptedIntoQueue, N> {
            self.event_filter::<DepositAcceptedIntoQueue>()
        }
        ///Creates a new event filter for the [`DisputeResolutionAcceptedIntoQueue`] event.
        pub fn DisputeResolutionAcceptedIntoQueue_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, DisputeResolutionAcceptedIntoQueue, N> {
            self.event_filter::<DisputeResolutionAcceptedIntoQueue>()
        }
        ///Creates a new event filter for the [`ERC20TokensWithdrawn`] event.
        pub fn ERC20TokensWithdrawn_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, ERC20TokensWithdrawn, N> {
            self.event_filter::<ERC20TokensWithdrawn>()
        }
        ///Creates a new event filter for the [`FailedDepositResolutionClosed`] event.
        pub fn FailedDepositResolutionClosed_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, FailedDepositResolutionClosed, N> {
            self.event_filter::<FailedDepositResolutionClosed>()
        }
        ///Creates a new event filter for the [`FerriedWithdrawalClosed`] event.
        pub fn FerriedWithdrawalClosed_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, FerriedWithdrawalClosed, N> {
            self.event_filter::<FerriedWithdrawalClosed>()
        }
        ///Creates a new event filter for the [`Initialized`] event.
        pub fn Initialized_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, Initialized, N> {
            self.event_filter::<Initialized>()
        }
        ///Creates a new event filter for the [`L2UpdateAccepted`] event.
        pub fn L2UpdateAccepted_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, L2UpdateAccepted, N> {
            self.event_filter::<L2UpdateAccepted>()
        }
        ///Creates a new event filter for the [`NativeTokensWithdrawn`] event.
        pub fn NativeTokensWithdrawn_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, NativeTokensWithdrawn, N> {
            self.event_filter::<NativeTokensWithdrawn>()
        }
        ///Creates a new event filter for the [`NewUpdaterSet`] event.
        pub fn NewUpdaterSet_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, NewUpdaterSet, N> {
            self.event_filter::<NewUpdaterSet>()
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
        ///Creates a new event filter for the [`PauserRegistrySet`] event.
        pub fn PauserRegistrySet_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, PauserRegistrySet, N> {
            self.event_filter::<PauserRegistrySet>()
        }
        ///Creates a new event filter for the [`Unpaused`] event.
        pub fn Unpaused_filter(&self) -> alloy_contract::Event<T, &P, Unpaused, N> {
            self.event_filter::<Unpaused>()
        }
        ///Creates a new event filter for the [`WithdrawalClosed`] event.
        pub fn WithdrawalClosed_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, WithdrawalClosed, N> {
            self.event_filter::<WithdrawalClosed>()
        }
        ///Creates a new event filter for the [`WithdrawalFerried`] event.
        pub fn WithdrawalFerried_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, WithdrawalFerried, N> {
            self.event_filter::<WithdrawalFerried>()
        }
    }
}
