use subxt::{
    client::OnlineClientT, config::{signed_extensions, substrate::{BlakeTwo256, Digest, NumberOrHex}, Hasher, Header, PolkadotExtrinsicParams}, utils::{AccountId32, MultiAddress}, Config, OnlineClient, PolkadotConfig, SubstrateConfig
};
use hex::encode as hex_encode;
use sha3;
use sha3::Digest as Keccak256Digest;
use hex_literal::hex;
use std::fmt;
// mod gasp;
// use subxt_signer::sr25519::dev::{self};
use primitive_types::{U256, H256, H512};

use parity_scale_codec::{Encode, Decode};
use serde::{Serialize, Deserialize};
use secp256k1::{
	ecdsa::{RecoverableSignature, RecoveryId}, rand, Message, PublicKey, Secp256k1, SecretKey
    // SECP256K1,
};
use subxt_signer::ecdsa::dev;
use subxt::tx::signer::Signer as SignerT;

use secp256k1::{
	SECP256K1,
};

mod signer;
use signer::Keypair;
use gasp::{GaspSignature, GaspPublicKey, GaspAddress, GaspConfig};

mod gasp;




#[tokio::main]
pub async fn main() {
    if let Err(err) = run().await {
        eprintln!("{err}");
    }
}

async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let api = OnlineClient::<GaspConfig>::from_url("ws://127.0.0.1:9944").await?;
    println!("Connection with parachain established.");

    let keypair = Keypair::from_secret_key(hex!("5fb92d6e98884f76de468fa3f6278f8807c48bebc13595d45af5bdc4da702133"));

    println!("Address: {}", keypair.address());



    let storage_query = gasp::api::storage().system().block_hash(0);

    let result = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
    .await?;

    let call = gasp::api::tx().tokens().transfer(gasp::api::runtime_types::sp_runtime::account::AccountId20([0u8; 20]), 0, 100);

    let tx = api
        .tx();

    let latest = api.backend().latest_finalized_block_ref().await?;
    let header = api.backend().block_header(latest.hash()).await?.unwrap();
    println!("Latest block: {:?}", latest);
    println!("Latest block: {:#?}", header);

    let partial_signed = tx.create_partial_signed(&call, &keypair.address(), Default::default()).await.expect("correct");
    // let signed = tx.create_signed(&call, &keypair, Default::default()).await.expect("correct");

    println!("payload 0x{}", hex_encode(partial_signed.signer_payload()));;
    let signed = partial_signed.sign(&keypair);
    println!("TX 0x{}", hex_encode(signed.encoded()));;

    let result = signed.submit_and_watch()
    .await
    .map(|e| {
        println!("NFT creation submitted, waiting for transaction to be finalized...");
        e
     })?
    .wait_for_finalized_success()
    .await?;
    //
    // println!("{:?}", partial_signed);
    // println!("{:?}", signed);
    // println!("{:?}", signed2);

    // signed.sign(keypair);
    // println!("Encoded call {}", hex_encode(signed.encoded()));

    // let result = tx.sign_and_submit_then_watch_default(&call, &keypair)
    // .await
    // .map(|e| {
    //     println!("NFT creation submitted, waiting for transaction to be finalized...");
    //     e
    //  })?
    // .wait_for_finalized_success()
    // .await?;
    //
    // let nft_creation_tx = gasp::tx()
    //     .tokens()
    //     .transfer(COLLECTION_ID, NTF_ID, alice.clone());

    // let _nft_creation_events = api
    //     .tx()
    //     .sign_and_submit_then_watch_default(&nft_creation_tx, &alice_pair_signer)
    //     .await
    //     .map(|e| {
    //         println!("NFT creation submitted, waiting for transaction to be finalized...");
    //         e
    //     })?
    //     .wait_for_finalized_success()
    //     .await?;
    println!("NFT created.");



    Ok(())
}

#[cfg(test)]
mod test{
    use super::*;
    use crate::Keypair;

    #[test]
    fn test_signer() {
        let keypair = Keypair::from_secret_key(hex!("5fb92d6e98884f76de468fa3f6278f8807c48bebc13595d45af5bdc4da702133"));
        let payload = hex!("0a000000000000000000000000000000000000000000000000009101c4000c006400000001000000fa3e64b8f64cdcb1541f6bcf09476c516bf20545befbabc9899f167b537174348bc02ea9888bcb3c61dd6ce6d5a29dfa0fb63b967d3e5bb23185c141a2a3409e");
        let hashed_payload: [u8; 32] = sha3::Keccak256::digest(&payload).into();

        println!("payload: {}", hex_encode(&payload));
        println!("hash(payload): {}", hex_encode(&hashed_payload));

        let signature = keypair.sign_prehashed(&hashed_payload);
        println!("signature: {}", signature);
        assert!(false);

    }
}

