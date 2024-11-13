use alloy::sol_types::SolValue;
use hex_literal::hex;
use tracing::level_filters::LevelFilter;
use tracing::{debug, info, trace, warn};

mod sequencer;

mod l1;
use l2::L2Error;
use l1::{L1Interface, L1Error, RolldownContract};

mod l2;
use l2::{Gasp, L2Interface};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("L1 error")]
    L1Error(#[from] L1Error),
    #[error("L2 error")]
    L2Error(#[from] L2Error),
}

#[tokio::main]
pub async fn main() {

    let filter = tracing_subscriber::EnvFilter::builder()
        .with_default_directive(LevelFilter::INFO.into())
        .from_env_lossy();
        // .add_directive("keepalive=info".parse()?)
        // .add_directive("p2p_playground=info".parse()?);

    tracing_subscriber::fmt().with_env_filter(filter).init();

    if let Err(err) = run().await {
        eprintln!("{err:?}");
    }
}

async fn run() -> Result<(), Error> {

    println!("Connection established.");

    let gasp = Gasp::new("ws://127.0.0.1:9944", hex!("5fb92d6e98884f76de468fa3f6278f8807c48bebc13595d45af5bdc4da702133")).await?;
    let r = RolldownContract::new("http://localhost:8545", hex!("1429859428C0aBc9C2C47C8Ee9FBaf82cFA0F20f")).await?;
    let at = gasp.latest_block().await?;
    let latest_processed_on_l2 = gasp.get_latest_processed_request_id(at).await?;
    let latest_create_on_l1 = r.get_latest_reqeust_id().await?;
    let start = latest_processed_on_l2.saturating_add(1u128);
    let end = latest_create_on_l1;

    let update = r.get_update(start, end).await?;
    let update_hash = r.get_update_hash(start, end).await?;
    let gasp_update = gasp.deserialize_sequencer_update(update.abi_encode()).await?;
    println!("update {:?}", gasp_update);

    let read_rights = gasp.get_read_rights(at).await?;

    if read_rights > 0u128 {
        gasp.update_l1_from_l2(gasp_update, update_hash).await?;
    }



    Ok(())
}
