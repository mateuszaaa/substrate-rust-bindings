use crate::l1::{L1Error, L1Interface};
use crate::l2::{L2Error, L2Interface};

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
    fn new(l1: L1, l2: L2) -> Self {
        Self { l1, l2 }
    }

    pub async fn find_malicious_update(&self) -> Result<u128, Error> {
        Ok(0u128)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::l1::types as l1types;
    use crate::l2::{types as l2types, PendingUpdate};
    use mockall;
    use mockall::predicate::eq;
    use primitive_types::H256;
    use hex_literal::hex;

    mockall::mock! {
        pub L1 {}

        impl L1Interface for L1{
            async fn get_latest_reqeust_id(&self) -> Result<u128, L1Error>;
            async fn get_update(&self, start: u128, end: u128) ->  Result<l1types::L1Update, L1Error>;
            async fn get_update_hash(&self, start: u128, end: u128) ->  Result<H256, L1Error>;
        }
    }

    mockall::mock! {
        pub L2 {}

        impl L2Interface for L2{
            async fn get_latest_processed_request_id(&self, at: H256) -> Result<u128, L2Error>;
            async fn get_read_rights(&self, at: H256) -> Result<u128, L2Error>;
            async fn get_cancel_rights(&self, at: H256) -> Result<u128, L2Error>;
            async fn get_pending_updates(&self, at: H256) -> Result<Vec<PendingUpdate>, L2Error>;
            async fn deserialize_sequencer_update(&self, data: Vec<u8>) -> Result<l2types::L1Update, L2Error>;
            async fn cancel_pending_request(&self, request_id: u128) -> Result<bool, L2Error>;
            async fn update_l1_from_l2(&self, update: l2types::L1Update, hash: H256) -> Result<bool, L2Error>;
        }
    }

    const DUMMY_ADDRESS: [u8; 20] = hex!("0000000000000000000000000000000000000000");

    #[tokio::test]
    async fn test_find_malicious_update() {
        // let dummy_update = l2types::L1Update {
        //     chain: l2types::Chain::Ethereum,
        //     pendingDeposits: vec![l2types::Deposit {
        //         requestId: l2types::RequestId {
        //             origin: l2types::Origin,
        //             id: 1u128,
        //         },
        //         depositRecipient: DUMMY_ADDRESS,
        //         tokenAddress: DUMMY_ADDRESS,
        //         amount: 100u128,
        //         timeStamp: 0u128,
        //         ferryTip: 0u128,
        //     }],
        //     pendingCancelResolutions: vec![],
        // };
        //
        // let l1mock = MockL1::new();
        // l1mock
        //     .expect_get_update_hash()
        //     .with(eq(0u128), eq(1u128))
        //     .times(1)
        //     .returning(|_, _| Ok(H256::zero()));
        //
        // let l2mock = MockL2::new();
        // l2mock
        //     .expect_get_pending_updates()
        //     .times(1)
        //     .returning(|_| Ok(vec![dummy_update]));
        //
        // let sequencer = Sequencer::new(l1mock, l2mock);
        //
        // // sequencer.find_malicious_update();
    }
}
