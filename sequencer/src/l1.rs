use alloy::network::Ethereum;
use alloy::primitives::Uint;
use alloy::providers::fillers::{
    BlobGasFiller, ChainIdFiller, FillProvider, GasFiller, JoinFill, NonceFiller,
};
use alloy::sol_types::SolValue;
use alloy::transports::BoxTransport;
use primitive_types::H256;
use sha3::{Digest, Keccak256};

use alloy::providers::{Identity, PendingTransactionError, ProviderBuilder, RootProvider};
// use subxt::ext::subxt_core::storage::address::{StorageHashers};

pub mod types {
    pub use bindings::rolldown::IRolldownPrimitives::L1Update;
    pub use bindings::rolldown::IRolldownPrimitives::Cancel;
}

#[derive(Debug, thiserror::Error)]
pub enum L1Error {
    #[error("Invalid range")]
    InvalidRange,
    #[error("Overflow error")]
    OverflowError,
    #[error("alloy error")]
    Alloy(#[from] alloy::contract::Error),
    #[error("alloy error")]
    TransportAlloy(#[from] alloy::transports::TransportError),
    #[error("transaction error")]
    TxSendError(#[from] PendingTransactionError),
}

pub trait L1Interface {
    async fn get_update(&self, start: u128, end: u128) -> Result<types::L1Update, L1Error>;
    async fn get_update_hash(&self, start: u128, end: u128) -> Result<H256, L1Error>;
    async fn get_latest_reqeust_id(&self) -> Result<Option<u128>, L1Error>;
    async fn get_latest_finalized_request_id(&self) -> Result<Option<u128>, L1Error>;
    async fn close_cancel(&self, cancel: types::Cancel, merkle_root:H256, proof: Vec<H256>) -> Result<H256, L1Error>;
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

    #[tracing::instrument(skip(self))]
    async fn get_latest_reqeust_id(&self) -> Result<Option<u128>, L1Error> {
        let call = self.contract_handle.counter();
        let result = call.call().await?;
        let next_request_id: u128 = result._0.try_into().or_else(|_| Err(L1Error::OverflowError))?;
        Ok(next_request_id.checked_sub(1u128))
    }

    #[tracing::instrument(skip(self))]
    async fn get_latest_finalized_request_id(&self) -> Result<Option<u128>, L1Error>{
        let call = self.contract_handle.getMerkleRootsLength();
        let length = call.call().await?;
        if let Some(id) = length._0.checked_sub(alloy::primitives::U256::from(1)){
            let latest_root = self.contract_handle.roots(id.clone()).call().await?;
            let range = self.contract_handle.merkleRootRange(latest_root._0).call().await?;
            Ok(Some(range.end.try_into().or_else(|_| Err(L1Error::OverflowError))?))
        }else{
            Ok(None)
        }
    }

    #[tracing::instrument(skip(self))]
    async fn get_update(&self, start: u128, end: u128) -> Result<types::L1Update, L1Error> {
        let latest = self.get_latest_reqeust_id().await?;

        if let Some(latest) = latest {
            if start < 1u128 || start > latest || end > latest || end < start {
                tracing::warn!("latest :{} range.start:{} range.end:{} ", latest, start, end);
                return Err(L1Error::InvalidRange);
            }

            let range_start = Uint::<256, 4>::from(start);
            let range_end = Uint::<256, 4>::from(end);
            let call = self
                .contract_handle
                .getPendingRequests(range_start, range_end);
            let result = call.call().await?;

            tracing::debug!("deposits: {} cancel_resolutions: {}", result._0.pendingDeposits.len(), result._0.pendingCancelResolutions.len());

            Ok(result._0)
        }else{
            tracing::warn!("there are no requests in contrcact yet");
            Err(L1Error::InvalidRange)
        }
    }

    #[tracing::instrument(skip(self, cancel))]
    async fn close_cancel(&self, cancel: types::Cancel, merkle_root:H256, proof: Vec<H256>) -> Result<H256, L1Error>{
        let proof = proof.into_iter().map(|elem| elem.0.into()).collect();
        let call =  self.contract_handle.close_cancel(cancel, merkle_root.0.into(), proof);
        match call.call().await{
            Ok(_) => {
                tracing::trace!("status ok");
                Ok(call.send().await?.watch().await?.0.into())
            },
            Err(err) => {
                tracing::warn!("status nok");
                Err(err.into())
            }
        }
    }

    #[tracing::instrument(skip(self))]
    async fn get_update_hash(&self, start: u128, end: u128) -> Result<H256, L1Error> {
        let pending_update = self.get_update(start, end).await?;
        let x: [u8; 32] = Keccak256::digest(&pending_update.abi_encode()[..]).into();
        Ok(x.into())
    }
}
