use std::fmt::{self, Display};

use parity_scale_codec::{Encode, Decode};

mod gasp_bindings;
pub use gasp_bindings::api as api;

mod header;
pub use header::GaspHeader;


use subxt::{
    config::{signed_extensions, substrate::BlakeTwo256}, Config
};
use primitive_types::H256;

#[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum GaspConfig {}

pub type GaspExtrinsicParams<T> = signed_extensions::AnyOf<
    T,
    (
        signed_extensions::CheckSpecVersion,
        signed_extensions::CheckTxVersion,
        signed_extensions::CheckGenesis<T>,
        signed_extensions::CheckMortality<T>,
        signed_extensions::CheckNonce,
        signed_extensions::ChargeTransactionPayment,
    ),
>;


impl Config for GaspConfig {
    type Hash = H256;
    type AccountId = GaspAddress;
    type Address = GaspAddress;
    type Signature = GaspSignature;
    type Hasher = BlakeTwo256;
    type Header = GaspHeader<u32, BlakeTwo256>;
    type ExtrinsicParams = GaspExtrinsicParams<Self>;
    type AssetId = u32;
}

#[derive(Encode, Decode, Debug, PartialEq, Eq, Clone)]
pub struct GaspSignature([u8; 65]);

impl From<[u8; 65]> for GaspSignature {
    fn from(data: [u8; 65]) -> Self {
        GaspSignature(data)
    }
}

impl Display for GaspSignature {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "0x{}", hex::encode(self.0))
    }
}

#[derive(Encode, Decode, Debug, PartialEq, Eq, Clone)]
pub struct GaspPublicKey([u8; 33]);

impl From<[u8; 33]> for GaspPublicKey {
    fn from(data: [u8; 33]) -> Self {
        GaspPublicKey(data)
    }
}

#[derive(Encode, Decode, Debug, PartialEq, Eq, Clone, Hash)]
pub struct GaspAddress([u8; 20]);

impl From<[u8; 20]> for GaspAddress {
    fn from(data: [u8; 20]) -> Self {
        GaspAddress(data)
    }
}

impl Display for GaspAddress {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "0x{}", hex::encode(self.0))
    }
}
