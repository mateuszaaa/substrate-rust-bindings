use alloy::sol_types::SolValue;
use hex_literal::hex;
use tracing::level_filters::LevelFilter;

mod sequencer;

mod l1;
use l1::{L1Error, L1Interface, RolldownContract};
use l2::L2Error;

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
        .with_default_directive(LevelFilter::TRACE.into())
        .from_env_lossy();
    // .add_directive("keepalive=info".parse()?)
    // .add_directive("p2p_playground=info".parse()?);

    tracing_subscriber::fmt().with_env_filter(filter).init();

    if let Err(err) = run().await {
        tracing::error!("{err:?}");
    }
}

async fn run() -> Result<(), Error> {
    println!("Connection established.");

    // let gasp = Gasp::new(
    //     "ws://127.0.0.1:9944",
    //     hex!("5fb92d6e98884f76de468fa3f6278f8807c48bebc13595d45af5bdc4da702133"),
    // )
    // .await?;
    let r = RolldownContract::new(
        "http://localhost:8545",
        hex!("1429859428C0aBc9C2C47C8Ee9FBaf82cFA0F20f"),
    )
    .await?;

    let result =  r.deposit_erc20(hex!("FD471836031dc5108809D173A067e8486B9047A3"), 1000, 10).await?;

    // let at = gasp.latest_block().await?.1;
    // let latest_processed_on_l2 = gasp.get_latest_processed_request_id(at).await?;
    // let latest_create_on_l1 = r.get_latest_reqeust_id().await?;
    // let start = latest_processed_on_l2.saturating_add(1u128);
    // let end = latest_create_on_l1.unwrap();
    //
    // let update = r.get_update(start, end).await?;
    // let update_hash = r.get_update_hash(start, end).await?;
    // let gasp_update = gasp
    //     .deserialize_sequencer_update(update.abi_encode())
    //     .await?;
    // println!("update {:?}", gasp_update);
    //
    Ok(())
}
