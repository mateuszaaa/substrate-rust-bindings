use subxt::OnlineClient;
use hex::encode as hex_encode;
use hex_literal::hex;

mod signer;
use signer::Keypair;
use gasp::GaspConfig;

mod gasp;




#[tokio::main]
pub async fn main() {
    if let Err(err) = run().await {
        eprintln!("{err}");
    }
}

async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let api = OnlineClient::<GaspConfig>::from_url("ws://127.0.0.1:9944").await?;
    println!("Connection established.");

    let keypair = Keypair::from_secret_key(hex!("5fb92d6e98884f76de468fa3f6278f8807c48bebc13595d45af5bdc4da702133"));
    println!("Address: {}", keypair.address());


    let storage_query = gasp::api::storage().system().block_hash(0);
    let genesis = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
    .await?
    .expect("should fetch genesis block");

    let latest = api.backend().latest_finalized_block_ref().await?;
    let header = api.backend().block_header(latest.hash()).await?.unwrap();

    println!("Genesis hash        : {:?}", genesis);
    println!("Latest block hash   : {:?}", latest);
    println!("Latest block seed   : {:?}", hex_encode(header.extrinsics_root));


    let call = gasp::api::tx().tokens().transfer(gasp::api::runtime_types::sp_runtime::account::AccountId20([0u8; 20]), 0, 100);
    let tx = api
        .tx();


    let partial_signed = tx.create_partial_signed(&call, &keypair.address(), Default::default()).await.expect("correct");
    println!("transaction payload 0x{}", hex_encode(partial_signed.signer_payload()));
    let signed = partial_signed.sign(&keypair);
    println!("signed transaction payload 0x{}", hex_encode(signed.encoded()));

    signed.submit_and_watch()
    .await
    .inspect(|_| {
        println!("Tx submitted successfully {:?}", call);
     })?;
    // .wait_for_finalized()
    // .await?;

    Ok(())
}
