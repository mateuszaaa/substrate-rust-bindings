use alloy::primitives::Uint;
use primitive_types::H256;
use subxt::config::{ExtrinsicParams, Header};
use subxt::dynamic::Value;
use subxt::ext::subxt_core;
use subxt::tx::TxInBlock;
use subxt::OnlineClient;
use hex::encode as hex_encode;
use hex_literal::hex;
use sha3::{Keccak256, Digest};
use alloy::sol_types::SolValue;

use alloy::providers::ProviderBuilder;
mod signer;
use signer::Keypair;
use futures::StreamExt;
use subxt::storage::StorageKey;
use alloy::contract;

mod gasp;
use gasp::{GaspAddress, GaspConfig, GaspSignature};
use subxt::Config;
// use bindings::rolldownstorage::RolldownStorage::getUpdateForL2Call

use subxt::ext::subxt_core::storage::address::{StorageHashers};

pub type SequencerPendingUpdateKey = (
    gasp::api::rolldown::storage::types::pending_sequencer_updates::Param0,
    gasp::api::rolldown::storage::types::pending_sequencer_updates::Param1
);

pub trait GaspApi<T: Config> {
    async fn get_pending_updates(&self, at: T::Hash) -> Result<Vec<PendingUpdate>, GaspError>;
    async fn cancel_pending_request(&self, request_id: u128) -> Result<(), GaspError>;
    async fn get_pending_update(&self) -> Result<(), GaspError>;
}

#[derive(Debug, thiserror::Error)]
enum RolldownError{
    #[error("Invalid range")]
    InvalidRange,
    #[error("alloy error")]
    Alloy(#[from] alloy::contract::Error),
    #[error("alloy error")]
    TransportAlloy(#[from] alloy::transports::TransportError),
}

pub trait RolldownApi {
   async fn get_update_hash(&self, start: u128, end: u128) -> Result<H256, RolldownError>; 
   async fn get_latest_reqeust_id(&self) -> Result<u128, RolldownError>;
}

pub struct Rolldown;
    // secret_key: Keypair,
// }


impl RolldownApi for Rolldown {
    async fn get_latest_reqeust_id(&self) -> Result<u128, RolldownError> {
        let provider = ProviderBuilder::new().with_recommended_fillers().on_builtin("http://localhost:8545").await?;
        let rolldown = bindings::rolldown::Rolldown::RolldownInstance::new(hex!("1429859428C0aBc9C2C47C8Ee9FBaf82cFA0F20f").into(), provider);
        let call = rolldown.counter();
        let result = call.call().await?;
        Ok(result._0.try_into().unwrap())
    }

    async fn get_update_hash(&self, start: u128, end: u128) ->  Result<H256, RolldownError> {

        let provider = ProviderBuilder::new().with_recommended_fillers().on_builtin("http://localhost:8545").await?;
        let rolldown = bindings::rolldown::Rolldown::RolldownInstance::new(hex!("1429859428C0aBc9C2C47C8Ee9FBaf82cFA0F20f").into(), provider);
        let latest = self.get_latest_reqeust_id().await?;

        println!("latest : {} start:{} end:{} ", latest, start, end);


        if (start >= latest || end >= latest) {
            println!("invalid range !!!!");
            return Err(RolldownError::InvalidRange);
        }

        let range_start = Uint::<256, 4>::from(start);
        let range_end = Uint::<256, 4>::from(end);
        let call = rolldown.getPendingRequests(range_start, range_end);

        let pending_update = call.call().await?;
        // let xxx = pending_update._0.abi_encode();
        let x: [u8; 32] = Keccak256::digest(&pending_update._0.abi_encode()[..]).into();
        Ok(x.into())
    }
}


pub struct Gasp<T: Config>{
    client: OnlineClient<T>,
    keypair: Keypair,
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("unknown error")]
    Unknown,
    #[error("unknown error")]
    Subxt(#[from] subxt::Error),
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
}


#[derive(Debug)]
pub struct PendingUpdate{
    update_id: u128,
    range: (u128, u128),
    hash: H256,
}

impl<T: Config> Gasp<T> {

    async fn last_finalized(&self) -> Result<T::Hash, Error> {
        Ok(self.client.backend().latest_finalized_block_ref().await?.hash())
    }

    async fn latest_block(&self) -> Result<T::Hash, Error> {
        let mut stream = self.client.backend().stream_all_block_headers().await?;
        Ok(stream.next().await.expect("infinite stream").map(|elem| elem.1.hash().into())?)
    }

    async fn new(uri: &str, secret_key: [u8; 32]) -> Result<Self, Error> {
        let client = OnlineClient::<T>::from_url(uri).await?;

        Ok(Self {
            client,
            keypair: Keypair::from_secret_key(secret_key)
        })
    }

    async fn wait_for_tx_execution(&self, tx_hash: T::Hash) -> Result<(), GaspError>{
        // let block_hash =  status.block_hash();
        // let tx_hash =  status.extrinsic_hash();

        // println!("tx hash: {} finalized at block {}", hex_encode(tx_hash), hex_encode(block_hash));
        // let block_number = self.client.backend()
        //     .block_header(block_hash)
        //     .await?
        //     .ok_or(GaspError::TxInclusionBlockDoesNotExits)?
        //     .number();
        //
        // // let max_block_number = block_number.add(10.into());
        // let max_block_number: u64 = block_number.into() + 10u64;
        //
        let mut stream = self.client
            .backend()
            .stream_best_block_headers()
            .await?;


        while let Some(header) = stream.next().await{
            let (header, hash) = header?;

            println!("looking for tx hash:{} in block {}", hex_encode(tx_hash), header.number().into());
            // if header.number().into() > max_block_number {
            //     return Err(GaspError::TxIncludedButNotExecuted);
            // }

            let block = self.client.blocks().at(hash).await?;
            let extrinsics = block.extrinsics().await?;

            if let Some(_) = extrinsics.iter().find(|extrinsic| extrinsic.hash() == tx_hash) {
                break;
            }
        }



        Ok(())

    }

}


impl<T: Config> GaspApi<T> for Gasp<T> where
    T::AccountId: From<GaspAddress>,
    T::Address: From<GaspAddress>,
    T::Signature: From<GaspSignature>,
    <<T as Config>::ExtrinsicParams as ExtrinsicParams<T>>::Params: Default
{
    async fn get_pending_update(&self) -> Result<(), GaspError>{
        Ok(())
    }

    async fn cancel_pending_request(&self, request_id: u128) -> Result<(), GaspError>{

    // let call = gasp::api::tx().rolldown().cancel_requests_from_l1(
    //     gasp::api::rolldown::calls::types::cancel_requests_from_l1::Chain::Ethereum,
    //     request_id
    // );

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
     // .wait_for_finalized()
     // .await?;

    self.wait_for_tx_execution(tx_hash).await?;


    Ok(())
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


#[tokio::main]
pub async fn main() {
    if let Err(err) = run().await {
        eprintln!("{err}");
    }
}

async fn run() -> Result<(), Box<dyn std::error::Error>> {

    println!("Connection established.");
    // let at = api.backend().latest_finalized_block_ref().await?;
    // println!("# 0x{} received", hex_encode(at.hash()));

    // let keypair = Keypair::from_secret_key();

    let gasp = Gasp::<GaspConfig>::new("ws://127.0.0.1:9944", hex!("5fb92d6e98884f76de468fa3f6278f8807c48bebc13595d45af5bdc4da702133")).await?;



    loop {
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        let at = gasp.latest_block().await?;
        println!("#{} is latest block", at);

        let result = gasp.get_pending_updates(at)
            .await
            .expect("should work");

        println!("pending updates: {}", result.len());
        // let provider = ProviderBuilder::new().with_recommended_fillers().on_builtin("http://localhost:8545").await?;
        // let rolldown = bindings::rolldown::Rolldown::RolldownInstance::new(hex!("1429859428C0aBc9C2C47C8Ee9FBaf82cFA0F20f").into(), provider);

        for update in result {
            // println!("pending update {:#?}", update);
            // let range_end = Uint::<256, 4>::from(update.range.1);
            // let range_start = Uint::<256, 4>::from(update.range.0);
            // let call = rolldown.getPendingRequests(range_start, range_end);

            let r = Rolldown;

            println!("checking update {:#?}", update);

            match r.get_update_hash(update.range.0, update.range.1).await {
                Ok(hash) => {
                    println!("hash: {} vs {}", hash, update.hash);
                    gasp.cancel_pending_request(update.update_id).await;
                },
                Err(RolldownError::InvalidRange) => {
                    println!("invalid range");
                    gasp.cancel_pending_request(update.update_id).await.unwrap();
                }
                Err(err) => {
                    println!("error {:?}", err);
                }
            }

            // if let Err(RolldownError::InvalidRange) =  {
            //     println!("invalid");
            // }else{
            //     println!("valid");
            // }
        }
    }

    Ok(())
}
