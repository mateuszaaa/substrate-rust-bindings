use crate::l1::{types as l1types, L1Error, L1Interface};
use crate::l2::{types as l2types, L2Error, L2Interface, PendingUpdate};

pub struct Sequencer<L1, L2> {
    l1: L1,
    l2: L2,
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("L1 error")]
    L1Error(#[from] L1Error),
    #[error("L2 error")]
    L2Error(#[from] L2Error),
}

impl<L1, L2> Sequencer<L1, L2>
where
    L1: L1Interface,
    L2: L2Interface,
{
    pub async fn find_malicious_update(&self) -> Result<u128, Error> {
        Ok(0u128)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use mockall;
    use primitive_types::H256;

    mockall::mock! {
        pub L1Mock {}

        impl L1Interface for L1Mock {
            async fn get_latest_reqeust_id(&self) -> Result<u128, L1Error>;
            async fn get_update(&self, start: u128, end: u128) ->  Result<l1types::L1Update, L1Error>;
            async fn get_update_hash(&self, start: u128, end: u128) ->  Result<H256, L1Error>;
        }
    }

    mockall::mock! {
        pub L2Mock {}

        impl L2Interface for L2Mock {
            async fn get_latest_processed_request_id(&self, at: H256) -> Result<u128, L2Error>;
            async fn get_read_rights(&self, at: H256) -> Result<u128, L2Error>;
            async fn get_cancel_rights(&self, at: H256) -> Result<u128, L2Error>;
            async fn get_pending_updates(&self, at: H256) -> Result<Vec<PendingUpdate>, L2Error>;
            async fn deserialize_sequencer_update(&self, data: Vec<u8>) -> Result<l2types::L1Update, L2Error>;
            async fn cancel_pending_request(&self, request_id: u128) -> Result<bool, L2Error>;
            async fn update_l1_from_l2(&self, update: l2types::L1Update, hash: H256) -> Result<bool, L2Error>;
        }
    }
}
