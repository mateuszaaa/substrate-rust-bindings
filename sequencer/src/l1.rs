use std::collections::HashMap;

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
use futures::StreamExt;
use subxt::storage::StorageKey;
use alloy::contract;

use subxt::Config;
use bindings::irolldown::IRolldownPrimitives::L1Update;

use subxt::ext::subxt_core::storage::address::{StorageHashers};


#[derive(Debug, thiserror::Error)]
pub enum RolldownError{
    #[error("Invalid range")]
    InvalidRange,
    #[error("alloy error")]
    Alloy(#[from] alloy::contract::Error),
    #[error("alloy error")]
    TransportAlloy(#[from] alloy::transports::TransportError),
}

pub trait RolldownApi {
   async fn get_update(&self, start: u128, end: u128) -> Result<bindings::rolldown::IRolldownPrimitives::L1Update, RolldownError>; 
   async fn get_update_hash(&self, start: u128, end: u128) -> Result<H256, RolldownError>; 
   async fn get_latest_reqeust_id(&self) -> Result<u128, RolldownError>;
}

pub struct Rolldown;

impl RolldownApi for Rolldown {
    async fn get_latest_reqeust_id(&self) -> Result<u128, RolldownError> {
        let provider = ProviderBuilder::new().with_recommended_fillers().on_builtin("http://localhost:8545").await?;
        let rolldown = bindings::rolldown::Rolldown::RolldownInstance::new(hex!("1429859428C0aBc9C2C47C8Ee9FBaf82cFA0F20f").into(), provider);
        let call = rolldown.counter();
        let result = call.call().await?;
        let next_request_id: u128 = result._0.try_into().unwrap();

        Ok(next_request_id.saturating_sub(1u128))
    }


    async fn get_update(&self, start: u128, end: u128) ->  Result<bindings::rolldown::IRolldownPrimitives::L1Update, RolldownError> {

        let provider = ProviderBuilder::new().with_recommended_fillers().on_builtin("http://localhost:8545").await?;
        let rolldown = bindings::rolldown::Rolldown::RolldownInstance::new(hex!("1429859428C0aBc9C2C47C8Ee9FBaf82cFA0F20f").into(), provider);
        let latest = self.get_latest_reqeust_id().await?;

        println!("latest : {} start:{} end:{} ", latest, start, end);


        if start < 1u128 || start > latest || end > latest || end < start {
            println!("invalid range !!!!");
            return Err(RolldownError::InvalidRange);
        }

        let range_start = Uint::<256, 4>::from(start);
        let range_end = Uint::<256, 4>::from(end);
        let call = rolldown.getPendingRequests(range_start, range_end);
        Ok(call.call().await?._0)
    }

    async fn get_update_hash(&self, start: u128, end: u128) ->  Result<H256, RolldownError> {
        let pending_update = self.get_update(start,end).await?;
        let x: [u8; 32] = Keccak256::digest(&pending_update.abi_encode()[..]).into();
        Ok(x.into())
    }
}
