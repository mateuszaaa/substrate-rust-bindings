use alloy::network::{Ethereum, EthereumWallet, TxSigner};
use alloy::signers::local::PrivateKeySigner;
use hex_literal::hex;
use alloy::providers::fillers::{
    BlobGasFiller, ChainIdFiller, FillProvider, GasFiller, JoinFill, NonceFiller, WalletFiller,
};
use alloy::sol_types::SolValue;
use alloy::transports::BoxTransport;
use primitive_types::H256;
use sha3::{Digest, Keccak256};

use alloy::providers::{Identity, PendingTransactionError, ProviderBuilder, RootProvider};
// use subxt::ext::subxt_core::storage::address::{StorageHashers};

pub mod types {
    pub use bindings::rolldown::IRolldownPrimitives::Cancel;
    pub use bindings::rolldown::IRolldownPrimitives::L1Update;
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
    async fn get_merkle_root(&self, request_id: u128) -> Result<([u8; 32], (u128, u128)), L1Error>;
    async fn get_latest_finalized_request_id(&self) -> Result<Option<u128>, L1Error>;
    async fn close_cancel(
        &self,
        cancel: types::Cancel,
        merkle_root: H256,
        proof: Vec<H256>,
    ) -> Result<H256, L1Error>;
}

pub type RolldownInstanceType = bindings::rolldown::Rolldown::RolldownInstance<
    BoxTransport,
    FillProvider<
        JoinFill<
            JoinFill<Identity,
                JoinFill<GasFiller, 
                    JoinFill<BlobGasFiller, 
                        JoinFill<NonceFiller, ChainIdFiller>
                    >
                >,
            >,
            WalletFiller<EthereumWallet>,
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
    pub async fn new(uri: &str, address: [u8; 20], private_key: [u8; 32]) -> Result<Self, L1Error> {

        let signer: PrivateKeySigner = hex::encode(private_key).parse().expect("valid private key");
        let wallet = EthereumWallet::new(signer);
        let provider = ProviderBuilder::new()
            // .wallet(hex!("ac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80"))
            .with_recommended_fillers()
            .wallet(wallet)
            .on_builtin(uri)
            .await?;
        Ok(Self {
            contract_handle: bindings::rolldown::Rolldown::RolldownInstance::new(
                address.into(),
                provider.clone(),
            )
        })
    }

    #[cfg(test)]
    #[tracing::instrument(skip(self))]
    pub async fn deposit_erc20(&self, token: [u8;20], amount: u128, ferry_tip: u128) -> Result<(), L1Error> {
        use hex::encode as hex_encode;

        let provider = self.contract_handle.provider().clone();
        let erc20_handle = bindings::ierc20::IERC20::IERC20Instance::new(
            token.into(),
            provider
            );

        let call = erc20_handle.approve(
            self.contract_handle.address().clone(),
            alloy::primitives::U256::from(amount), 
        );

        tracing::trace!("approve send");
        let builder = call.send().await?;
        let hash = builder.watch().await?;
        tracing::debug!("approve hash: {}", hex_encode(hash));


        let call = self.contract_handle.deposit_erc20_0(
                token.into(), 
                alloy::primitives::U256::from(amount), 
                alloy::primitives::U256::from(ferry_tip)
        );

        let hash = call.send().await?.watch().await?;
        tracing::debug!("deposit hash: {}", hex_encode(hash));

        Ok(())
    }
}

impl L1Interface for RolldownContract {
    #[tracing::instrument(skip(self))]
    async fn get_merkle_root(&self, request_id: u128) -> Result<([u8; 32], (u128, u128)), L1Error>{

        let request_id = alloy::primitives::U256::from(request_id);
        let call = self.contract_handle.find_l2_batch(request_id);
        let merkle_root = call.call().await?._0;

        let call = self.contract_handle.merkleRootRange(merkle_root);
        let range = call.call().await?;

        let range_start = range.start.try_into().or_else(|_| Err(L1Error::OverflowError))?;
        let range_end = range.end.try_into().or_else(|_| Err(L1Error::OverflowError))?;
        Ok((merkle_root.0, (range_start, range_end)))
    }

    #[tracing::instrument(skip(self))]
    async fn get_latest_reqeust_id(&self) -> Result<Option<u128>, L1Error> {
        let call = self.contract_handle.counter();
        let result = call.call().await?;
        let next_request_id: u128 = result
            ._0
            .try_into()
            .or_else(|_| Err(L1Error::OverflowError))?;
        if next_request_id == 1 {
            Ok(None)
        }else{
            Ok(next_request_id.checked_sub(1u128))
        }
    }

    #[tracing::instrument(skip(self))]
    async fn get_latest_finalized_request_id(&self) -> Result<Option<u128>, L1Error> {
        let call = self.contract_handle.getMerkleRootsLength();
        let length = call.call().await?;
        if let Some(id) = length._0.checked_sub(alloy::primitives::U256::from(1)) {
            let latest_root = self.contract_handle.roots(id.clone()).call().await?;
            let range = self
                .contract_handle
                .merkleRootRange(latest_root._0)
                .call()
                .await?;
            Ok(Some(
                range
                    .end
                    .try_into()
                    .or_else(|_| Err(L1Error::OverflowError))?,
            ))
        } else {
            Ok(None)
        }
    }

    #[tracing::instrument(skip(self))]
    async fn get_update(&self, start: u128, end: u128) -> Result<types::L1Update, L1Error> {
        let latest = self.get_latest_reqeust_id().await?;

        if let Some(latest) = latest {
            if start < 1u128 || start > latest || end > latest || end < start {
                tracing::warn!(
                    "latest :{} range.start:{} range.end:{} ",
                    latest,
                    start,
                    end
                );
                return Err(L1Error::InvalidRange);
            }

            let range_start = alloy::primitives::U256::from(start);
            let range_end = alloy::primitives::U256::from(end);
            let call = self
                .contract_handle
                .getPendingRequests(range_start, range_end);
            let result = call.call().await?;

            tracing::debug!(
                "deposits: {} cancel_resolutions: {}",
                result._0.pendingDeposits.len(),
                result._0.pendingCancelResolutions.len()
            );

            Ok(result._0)
        } else {
            tracing::warn!("there are no requests in contrcact yet");
            Err(L1Error::InvalidRange)
        }
    }

    #[tracing::instrument(skip(self, cancel))]
    async fn close_cancel(
        &self,
        cancel: types::Cancel,
        merkle_root: H256,
        proof: Vec<H256>,
    ) -> Result<H256, L1Error> {
        let proof = proof.into_iter().map(|elem| elem.0.into()).collect();
        let call = self
            .contract_handle
            .close_cancel(cancel, merkle_root.0.into(), proof);
        match call.call().await {
            Ok(_) => {
                tracing::trace!("status ok");
                Ok(call.send().await?.watch().await?.0.into())
            }
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

#[cfg(test)]
mod test{
    use super::*;
    use hex_literal::hex;
    use serial_test::serial;

    const URI: &'static str = "http://localhost:8545";
    const DUMMY_ADDRESS: [u8; 20] = hex!("0000000000000000000000000000000000000000");
    const ROLLDOWN_ADDRESS: [u8; 20] = hex!("1429859428C0aBc9C2C47C8Ee9FBaf82cFA0F20f");
    const TOKEN_ADDRESS: [u8; 20] = hex!("FD471836031dc5108809D173A067e8486B9047A3");
    const ALICE_PKEY: [u8; 32] = hex!("dbda1821b80551c9d65939329250298aa3472ba22feea921c0cf5d620ea67b97");
    // const ETHEREUM: l2types::Chain = l2types::Chain::Ethereum;
    // const ARBITRUM: l2types::Chain = l2types::Chain::Arbitrum;

    // async fn get_update(&self, start: u128, end: u128) -> Result<types::L1Update, L1Error>;
    // async fn get_update_hash(&self, start: u128, end: u128) -> Result<H256, L1Error>;
    // async fn get_latest_reqeust_id(&self) -> Result<Option<u128>, L1Error>;
    // async fn get_latest_finalized_request_id(&self) -> Result<Option<u128>, L1Error>;
    // async fn close_cancel( &self, cancel: types::Cancel, merkle_root: H256, proof: Vec<H256>) -> Result<H256, L1Error>;

    #[serial]
    #[tokio::test]
    async fn test_can_connect() {
        RolldownContract::new(URI, ROLLDOWN_ADDRESS, ALICE_PKEY).await.unwrap();
    }

    #[serial]
    #[tokio::test]
    async fn test_can_latest_request_id() {
        let rolldown = RolldownContract::new(URI, ROLLDOWN_ADDRESS, ALICE_PKEY).await.unwrap();
        rolldown.deposit_erc20(TOKEN_ADDRESS, 1000, 10).await.unwrap();
    }


}
