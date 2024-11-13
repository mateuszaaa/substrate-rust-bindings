use alloy::network::Ethereum;
use alloy::primitives::Uint;
use alloy::providers::fillers::{
    BlobGasFiller, ChainIdFiller, FillProvider, GasFiller, JoinFill, NonceFiller,
};
use alloy::sol_types::SolValue;
use alloy::transports::BoxTransport;
use primitive_types::H256;
use sha3::{Digest, Keccak256};

use alloy::providers::{Identity, ProviderBuilder, RootProvider};
// use subxt::ext::subxt_core::storage::address::{StorageHashers};

pub mod types {
    pub use bindings::rolldown::IRolldownPrimitives::L1Update;
}

#[derive(Debug, thiserror::Error)]
pub enum L1Error {
    #[error("Invalid range")]
    InvalidRange,
    #[error("alloy error")]
    Alloy(#[from] alloy::contract::Error),
    #[error("alloy error")]
    TransportAlloy(#[from] alloy::transports::TransportError),
}

pub trait L1Interface {
    async fn get_update(&self, start: u128, end: u128) -> Result<types::L1Update, L1Error>;
    async fn get_update_hash(&self, start: u128, end: u128) -> Result<H256, L1Error>;
    async fn get_latest_reqeust_id(&self) -> Result<u128, L1Error>;
}

pub type RolldownInstanceType = bindings::rolldown::Rolldown::RolldownInstance<
    BoxTransport,
    FillProvider<
        JoinFill<
            Identity,
            JoinFill<GasFiller, JoinFill<BlobGasFiller, JoinFill<NonceFiller, ChainIdFiller>>>,
        >,
        RootProvider<BoxTransport>,
        BoxTransport,
        Ethereum,
    >,
>;

pub struct RolldownContract {
    contract_handle: RolldownInstanceType,
}

impl RolldownContract {
    pub async fn new(uri: &str, address: [u8; 20]) -> Result<Self, L1Error> {
        let provider = ProviderBuilder::new()
            .with_recommended_fillers()
            .on_builtin(uri)
            .await?;
        Ok(Self {
            contract_handle: bindings::rolldown::Rolldown::RolldownInstance::new(
                address.into(),
                provider,
            ),
        })
    }
}

impl L1Interface for RolldownContract {
    async fn get_latest_reqeust_id(&self) -> Result<u128, L1Error> {
        let call = self.contract_handle.counter();
        let result = call.call().await?;
        let next_request_id: u128 = result._0.try_into().unwrap();

        Ok(next_request_id.saturating_sub(1u128))
    }

    async fn get_update(&self, start: u128, end: u128) -> Result<types::L1Update, L1Error> {
        let latest = self.get_latest_reqeust_id().await?;

        println!("latest : {} start:{} end:{} ", latest, start, end);

        if start < 1u128 || start > latest || end > latest || end < start {
            println!("invalid range !!!!");
            return Err(L1Error::InvalidRange);
        }

        let range_start = Uint::<256, 4>::from(start);
        let range_end = Uint::<256, 4>::from(end);
        let call = self
            .contract_handle
            .getPendingRequests(range_start, range_end);
        Ok(call.call().await?._0)
    }

    async fn get_update_hash(&self, start: u128, end: u128) -> Result<H256, L1Error> {
        let pending_update = self.get_update(start, end).await?;
        let x: [u8; 32] = Keccak256::digest(&pending_update.abi_encode()[..]).into();
        Ok(x.into())
    }
}
