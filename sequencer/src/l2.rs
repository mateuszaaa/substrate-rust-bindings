use std::collections::HashMap;

use hex::encode as hex_encode;

use primitive_types::H256;
use subxt::config::ExtrinsicParams;
use subxt::config::Header;
use subxt::ext::subxt_core;
use subxt::ext::subxt_core::storage::address::StorageHashers;
use subxt::storage::StorageKey;
use subxt::OnlineClient;
use subxt::Config;
use futures::StreamExt;

use crate::gasp;
use crate::gasp::GaspConfig;
use crate::gasp::GaspAddress;
use crate::gasp::GaspSignature;
use crate::signer::Keypair;

#[derive(Debug)]
pub struct PendingUpdate{
    update_id: u128,
    range: (u128, u128),
    hash: H256,
}

use gasp::api::runtime_types::frame_system::EventRecord;
use gasp::api::runtime_types::rollup_runtime::RuntimeEvent;
pub type GaspEvent = EventRecord<RuntimeEvent, H256>;

pub type SequencerPendingUpdateKey = (
    gasp::api::rolldown::storage::types::pending_sequencer_updates::Param0,
    gasp::api::rolldown::storage::types::pending_sequencer_updates::Param1
);

pub trait GaspApi<T: Config> {
    async fn get_latest_processed_request_id(&self, at: T::Hash) -> Result<u128, GaspError>;
    async fn get_read_rights(&self, at: T::Hash) -> Result<u128, GaspError>;
    async fn get_cancel_rights(&self, at: T::Hash) -> Result<u128, GaspError>;
    async fn get_pending_updates(&self, at: T::Hash) -> Result<Vec<PendingUpdate>, GaspError>;
    async fn deserialize_sequencer_update(&self, data: Vec<u8>) -> Result<gasp::api::runtime_types::pallet_rolldown::messages::L1Update, GaspError>;
    async fn cancel_pending_request(&self, request_id: u128) -> Result<bool, GaspError>;
    async fn update_l1_from_l2(&self, update: gasp::api::runtime_types::pallet_rolldown::messages::L1Update, hash: H256) -> Result<bool, GaspError>;
}

pub struct Gasp<T: Config>{
    client: OnlineClient<T>,
    keypair: Keypair,
}


#[derive(Debug, thiserror::Error)]
pub enum GaspError {
    #[error("tx inclusion block does not exist")]
    TxInclusionBlockDoesNotExits,
    #[error("tx included but not executed")]
    TxIncludedButNotExecuted,
    #[error("block fetch error")]
    BlockFetchError,
    #[error("unknown error")]
    Subxt(#[from] subxt::Error),
    #[error("cannot fetch sequencer rights")]
    CanNotFetchRights,
    #[error("runtime api call failed")]
    SequencerUpdateConversionError,
    #[error("cannot fetch last processed request id")]
    CanNotFetchLatestProcessedRequestId,
    #[error("unknown tx status")]
    UnknownTxStatus,
}


impl<T: Config> Gasp<T> {

    pub async fn last_finalized(&self) -> Result<T::Hash, GaspError> {
        Ok(self.client.backend().latest_finalized_block_ref().await?.hash())
    }

    pub async fn latest_block(&self) -> Result<T::Hash, GaspError> {
        let mut stream = self.client.backend().stream_all_block_headers().await?;
        Ok(stream.next().await.expect("infinite stream").map(|elem| elem.1.hash().into())?)
    }

    pub async fn new(uri: &str, secret_key: [u8; 32]) -> Result<Self, GaspError> {
        let client = OnlineClient::<T>::from_url(uri).await?;

        Ok(Self {
            client,
            keypair: Keypair::from_secret_key(secret_key)
        })
    }

    async fn get_events(&self, at: T::Hash) -> Result<Vec<GaspEvent>, GaspError>{

        let storage = gasp::api::storage().system().events();
        Ok(self.client.storage()
            .at(at)
            .fetch(&storage)
            .await?
            .unwrap_or_default())
    }

    async fn wait_for_tx_execution(&self, tx_hash: T::Hash) -> Result<bool, GaspError>{
        let mut stream = self.client
            .backend()
            .stream_best_block_headers()
            .await?;


        while let Some(header) = stream.next().await{
            let (header, hash) = header?;

            println!("looking for tx hash:{} in block {}", hex_encode(tx_hash), header.number().into());

            let block = self.client.blocks().at(hash.clone()).await?;
            let extrinsics = block.extrinsics().await?;

            if let Some((id, _)) = extrinsics.iter().enumerate().find(|(id, extrinsic)| extrinsic.hash() == tx_hash) {

                let events= self.get_events(hash.hash()).await?;
                let events = events.iter()
                    .filter(|elem| matches!(elem.phase, gasp::api::runtime_types::frame_system::Phase::ApplyExtrinsic(id)))
                    .collect::<Vec<_>>();

                let status = events.iter().find(|elem | {
                        matches!(elem.event, RuntimeEvent::System(gasp::api::runtime_types::frame_system::pallet::Event::ExtrinsicSuccess{..})) ||
                        matches!(elem.event, RuntimeEvent::System(gasp::api::runtime_types::frame_system::pallet::Event::ExtrinsicFailed{..}))
                });

                let elem = status.ok_or(GaspError::UnknownTxStatus)?;

                return match elem.event {
                    RuntimeEvent::System(gasp::api::runtime_types::frame_system::pallet::Event::ExtrinsicSuccess{..}) => {
                        Ok(true)
                    },
                    RuntimeEvent::System(gasp::api::runtime_types::frame_system::pallet::Event::ExtrinsicFailed{..}) => {
                        Ok(false)
                    },
                    _ => {
                        Err(GaspError::UnknownTxStatus)
                    }
                };

            }
        }



        Err(GaspError::UnknownTxStatus)

    }

}


impl<T: Config> GaspApi<T> for Gasp<T> where
    T::AccountId: From<GaspAddress>,
    T::Address: From<GaspAddress>,
    T::Signature: From<GaspSignature>,
    <<T as Config>::ExtrinsicParams as ExtrinsicParams<T>>::Params: Default
{

    async fn get_latest_processed_request_id(&self, at: T::Hash) -> Result<u128, GaspError>{

        //NOTE: use through parameter
        let chain = gasp::api::rolldown::calls::types::cancel_requests_from_l1::Chain::Ethereum;
        let storage = gasp::api::storage().rolldown().last_processed_request_on_l2(chain);
        Ok(self.client.storage()
            .at(at)
            .fetch(&storage)
            .await?
            .unwrap_or_default())
    }

    async fn get_read_rights(&self, at: T::Hash) -> Result<u128, GaspError>{
        use gasp::api::runtime_types::pallet_rolldown::pallet::SequencerRights;

        let chain = gasp::api::rolldown::calls::types::cancel_requests_from_l1::Chain::Ethereum;
        let storage = gasp::api::storage().rolldown().sequencers_rights(chain);
        let rights : HashMap<GaspAddress, SequencerRights>= self.client.storage()
            .at(at)
            .fetch(&storage)
            .await?
            .unwrap_or_default()
            .into_iter()
            .map(|(k, v)| (k.0.into(), v))
            .collect();

        rights.get(&self.keypair.address())
            .map(|elem| elem.read_rights)
            .ok_or(GaspError::CanNotFetchRights)
    }

    async fn get_cancel_rights(&self, at: T::Hash) -> Result<u128, GaspError>{
        use gasp::api::runtime_types::pallet_rolldown::pallet::SequencerRights;

        //NOTE: create parameter
        let chain = gasp::api::rolldown::calls::types::cancel_requests_from_l1::Chain::Ethereum;
        let storage = gasp::api::storage().rolldown().sequencers_rights(chain);
        let rights : HashMap<GaspAddress, SequencerRights>= self.client.storage()
            .at(at)
            .fetch(&storage)
            .await?
            .unwrap_or_default()
            .into_iter()
            .map(|(k, v)| (k.0.into(), v))
            .collect();

        rights.get(&self.keypair.address())
            .map(|elem| elem.cancel_rights)
            .ok_or(GaspError::CanNotFetchRights)
    }

    async fn deserialize_sequencer_update(&self, payload: Vec<u8>) -> Result<gasp::api::runtime_types::pallet_rolldown::messages::L1Update, GaspError>{

        let call = gasp::api::runtime_apis::rolldown_runtime_api::RolldownRuntimeApi.get_native_sequencer_update(payload);

        let update = self.client
            .runtime_api()
            .at_latest()
            .await?
            .call(call)
        .await?;

        update.ok_or(GaspError::SequencerUpdateConversionError)
    }

    async fn update_l1_from_l2(&self, update: gasp::api::runtime_types::pallet_rolldown::messages::L1Update, hash: H256) -> Result<bool, GaspError>{

        let call = gasp::api::tx().rolldown().update_l2_from_l1(
            update,
            hash
        );

        let tx = self.client.tx();

        let partial_signed = tx.create_partial_signed(&call, &self.keypair.address().into(), Default::default()).await.expect("correct");
        println!("transaction payload : {}", hex_encode(partial_signed.signer_payload()));

        let signed = partial_signed.sign(&self.keypair);
        println!("signed tx payload   : {}", hex_encode(signed.encoded()));

        let tx_hash = signed.hash();
        println!("TX HASH: {}", hex_encode(tx_hash));

        let status = signed.submit_and_watch()
            .await
            .inspect(|elem| {
                println!("Tx submitted successfully {:?}", call);
            })?;

        Ok(self.wait_for_tx_execution(tx_hash).await?)
    }

    async fn cancel_pending_request(&self, request_id: u128) -> Result<bool, GaspError>{

    let call = gasp::api::tx().rolldown().cancel_requests_from_l1(
        gasp::api::rolldown::calls::types::cancel_requests_from_l1::Chain::Ethereum,
        request_id
    );

    let tx = self.client.tx();

    let partial_signed = tx.create_partial_signed(&call, &self.keypair.address().into(), Default::default()).await.expect("correct");
     println!("transaction payload : {}", hex_encode(partial_signed.signer_payload()));

     let signed = partial_signed.sign(&self.keypair);
     println!("signed tx payload   : {}", hex_encode(signed.encoded()));

     let tx_hash = signed.hash();
     println!("TX HASH: {}", hex_encode(tx_hash));

     let status = signed.submit_and_watch()
     .await
     .inspect(|elem| {
         println!("Tx submitted successfully {:?}", call);
      })?;

    Ok(self.wait_for_tx_execution(tx_hash).await?)
    }

    async fn get_pending_updates(&self, at: T::Hash) -> Result<Vec<PendingUpdate>, GaspError> {
        use ::subxt::ext::subxt_core::storage::address::StaticStorageKey;
        use gasp::api::rolldown::storage::types as gasp_types;

        let metadata = self.client.metadata();
        let (_pallet, entry) = subxt_core::storage::lookup_storage_entry_details(
            "Rolldown",
            "PendingSequencerUpdates",
            &metadata,
        ).expect("should work");
        let hashers = StorageHashers::new(entry.entry_type(), metadata.types()).expect("is fine");

        let iter = gasp::api::storage().rolldown().pending_sequencer_updates_iter();
        let result: Vec<Result<PendingUpdate, GaspError>> = self.client.storage()
            .at(at)
            .iter(iter)
            .await?
            .map(|result| -> Result<PendingUpdate, GaspError> {
                let storage_kv = result?;
                // println!("storage_kv: {:?}", storage_kv);
                let (acc, update, hash) = storage_kv.value;

                let min_deposit_id = update.pendingDeposits
                    .iter()
                    .map(|elem| elem.requestId.id)
                    .min()
                    .unwrap_or(u128::MAX);
                let max_deposit_id = update.pendingDeposits.iter()
                    .map(|elem| elem.requestId.id)
                    .max()
                    .unwrap_or(0u128);

                let min_cancel_id = update.pendingCancelResolutions.iter()
                    .map(|elem| elem.requestId.id)
                    .min()
                    .unwrap_or(u128::MAX);
                let max_cancel_id = update.pendingCancelResolutions.iter()
                    .map(|elem| elem.requestId.id)
                    .max()
                    .unwrap_or(0u128);

                //TODO: remove unwrap
                let keys = <( StaticStorageKey<gasp_types::pending_sequencer_updates::Param0>, StaticStorageKey<gasp_types::pending_sequencer_updates::Param1>)>::decode_storage_key(
                    &mut &storage_kv.key_bytes[32..],
                    &mut hashers.iter(),
                    metadata.types(),
                ).unwrap();



                Ok(PendingUpdate {
                    update_id: keys.0.decoded().unwrap(),
                    range: (
                        std::cmp::min(min_deposit_id, min_cancel_id),
                        std::cmp::max(max_deposit_id, max_cancel_id)
                    ),
                    hash,
                }) 
            })
            .collect()
            .await;
            result.into_iter().collect()
    }

}
