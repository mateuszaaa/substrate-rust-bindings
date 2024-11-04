use primitive_types::H256;
use subxt::dynamic::Value;
use subxt::ext::subxt_core;
use subxt::OnlineClient;
use hex::encode as hex_encode;
use hex_literal::hex;

mod signer;
use signer::Keypair;
use futures::StreamExt;
use subxt::storage::StorageKey;

mod gasp;
use gasp::GaspConfig;
use subxt::Config;

use subxt::ext::subxt_core::storage::address::{StorageHashers};

pub type SequencerPendingUpdateKey = (
    gasp::api::rolldown::storage::types::pending_sequencer_updates::Param0,
    gasp::api::rolldown::storage::types::pending_sequencer_updates::Param1
);

pub trait GaspApi<T: Config> {
    async fn get_pending_updates(&self, at: T::Hash) -> Result<Vec<PendingUpdate>, Error>;
}

pub struct Gasp<T: Config>(pub OnlineClient<T>);

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("unknown error")]
    Unknown,
    #[error("unknown error")]
    Subxt(#[from] subxt::Error),
}

#[derive(Debug)]
pub struct PendingUpdate{
    update_id: u128,
    range: (u128, u128),
    hash: H256,
}

impl<T: Config> GaspApi<T> for Gasp<T> {
    async fn get_pending_updates(&self, at: T::Hash) -> Result<Vec<PendingUpdate>, Error> {


        use ::subxt::ext::subxt_core::storage::address::StaticStorageKey;
        use gasp::api::rolldown::storage::types as gasp_types;

        let metadata = self.0.metadata();
        let (_pallet, entry) = subxt_core::storage::lookup_storage_entry_details(
            "Rolldown",
            "PendingSequencerUpdates",
            &metadata,
        ).expect("should work");
        let hashers = StorageHashers::new(entry.entry_type(), metadata.types()).expect("is fine");

        let iter = gasp::api::storage().rolldown().pending_sequencer_updates_iter();
        let result: Vec<Result<PendingUpdate, Error>> = self.0.storage()
            .at(at)
            .iter(iter)
            .await?
            .map(|result| -> Result<PendingUpdate, Error> {
                let storage_kv = result?;
                println!("storage_kv: {:?}", storage_kv);
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
    let api = OnlineClient::<GaspConfig>::from_url("ws://127.0.0.1:9944").await?;

    println!("Connection established.");
    let at = api.backend().latest_finalized_block_ref().await?;

    println!("# 0x{} received", hex_encode(at.hash()));

    let keypair = Keypair::from_secret_key(hex!("5fb92d6e98884f76de468fa3f6278f8807c48bebc13595d45af5bdc4da702133"));
    println!("Address             : {}", keypair.address());

    let gasp = Gasp(api);
    //
    let result = gasp.get_pending_updates(at.hash())
        .await
        .expect("should work");

    println!("result: {:?}", result);

    // let storage_query = gasp::api::storage().rolldown().pending_sequencer_updates();



    // NOTE: dynamic example
    // // Build a dynamic storage query to iterate account information.
    // // With a dynamic query, we can just provide an empty vector as the keys to iterate over all entries.
    // let keys: Vec<Value> = vec![];
    // let storage_query = subxt::dynamic::storage("Rolldown", "PendingSequencerUpdates", keys);
    //
    // // Use that query to return an iterator over the results.
    // let mut results = api.storage().at_latest().await?.iter(storage_query).await?;
    //
    // while let Some(Ok(kv)) = results.next().await {
    //     println!("Keys decoded: {:?}", kv.keys);
    //     // println!("Key: 0x{}", hex::encode(&kv.key_bytes));
    //     // println!("Value: {:?}", kv.value.to_value()?);
    // }


    // let genesis = api
    //     .storage()
    //     .at_latest()
    //     .await?
    //     .fetch(&storage_query)
    // .await?
    // .expect("should fetch genesis block");

    // let latest = api.backend().latest_finalized_block_ref().await?;
    // let header = api.backend().block_header(latest.hash()).await?.unwrap();
    //
    // println!("Genesis hash        : {:?}", genesis);
    // println!("Latest block hash   : {:?}", latest);
    // println!("Latest block seed   : {:?}", hex_encode(header.extrinsics_root));
    //
    // // gasp::api::
    //
    // let call = gasp::api::tx().tokens().transfer(gasp::api::runtime_types::sp_runtime::account::AccountId20([0u8; 20]), 0, 100);
    // let tx = api
    //     .tx();
    //
    //
    // let partial_signed = tx.create_partial_signed(&call, &keypair.address(), Default::default()).await.expect("correct");
    // println!("transaction payload : {}", hex_encode(partial_signed.signer_payload()));
    // let signed = partial_signed.sign(&keypair);
    // println!("signed tx payload   : {}", hex_encode(signed.encoded()));
    //
    // signed.submit_and_watch()
    // .await
    // .inspect(|_| {
    //     println!("Tx submitted successfully {:?}", call);
    //  })?;
    // // .wait_for_finalized()
    // // .await?;

    Ok(())
}
