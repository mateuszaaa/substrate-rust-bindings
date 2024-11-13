use alloy::sol_types::SolValue;
use futures::StreamExt;
use parity_scale_codec::Encode;
use primitive_types::H256;

use crate::l1::{L1Error, L1Interface};
use crate::l2::{L2Error, L2Interface, PendingUpdate, types as l2types};


pub struct Sequencer<L1, L2> {
    l1: L1,
    l2: L2,
    chain: l2types::Chain,
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
    fn new(l1: L1, l2: L2, chain: l2types::Chain) -> Self {
        Self { l1, l2 , chain}
    }

    pub async fn find_malicious_update(&self, at: H256) -> Result<Option<u128>, Error> {


        let updates = self.l2.get_pending_updates(at).await?;
        let mut updates = updates.into_iter().map(|(update_id, update, update_hash)| -> PendingUpdate {
            let min_deposit_id = update
                .pendingDeposits
                .iter()
                .map(|elem| elem.requestId.id)
                .min()
                .unwrap_or(u128::MAX);

            let max_deposit_id = update
                .pendingDeposits
                .iter()
                .map(|elem| elem.requestId.id)
                .max()
                .unwrap_or(0u128);

            let min_cancel_id = update
                .pendingCancelResolutions
                .iter()
                .map(|elem| elem.requestId.id)
                .min()
                .unwrap_or(u128::MAX);

            let max_cancel_id = update
                .pendingCancelResolutions
                .iter()
                .map(|elem| elem.requestId.id)
                .max()
                .unwrap_or(0u128);

            PendingUpdate {
                update_id,
                chain: update.chain,
                range: (
                    std::cmp::min(min_deposit_id, min_cancel_id),
                    std::cmp::max(max_deposit_id, max_cancel_id),
                ),
                hash: update_hash,
            }
         })
        .filter(|update| update.chain == self.chain)
        .collect::<Vec<_>>();
        updates.sort_by_key(|update| update.update_id);

        let l1handle = &self.l1;

        let mut verified = futures::stream::iter(updates)
            .map(|update| async {
                let correct_hash = l1handle.get_update_hash(update.range.0, update.range.1).await?;
                Ok::<_, Error>((correct_hash, update))
            });

        while let Some(result) = verified.next().await {
            let (correct_hash, update) = result.await?;
            if correct_hash != update.hash {
                return Ok(Some(update.update_id));
            }
        }

        Ok(None)
    }


    pub async fn cancel_update(&self, update_id: u128) -> Result<bool, Error> {
        Ok(self.l2.cancel_pending_request(update_id, self.chain.clone()).await?)
    }

    pub async fn has_read_rights_available(&self, at: H256) -> Result<bool, Error>{
        Ok(self.l2.get_read_rights(at).await? > 0)
    }

    pub async fn has_cancel_rights_available(&self, at: H256) -> Result<bool, Error>{
        Ok(self.l2.get_cancel_rights(at).await? > 0)
    }

    pub async fn get_pending_update(&self, at: H256) -> Result<Option<(H256, l2types::L1Update)>, Error> {
        let latest_processed_on_l2 = self.l2.get_latest_processed_request_id(at).await?;
        let latest_create_on_l1 = self.l1.get_latest_reqeust_id().await?;
        let start = latest_processed_on_l2.saturating_add(1u128);
        let end = latest_create_on_l1;
        let update = self.l1.get_update(start, end).await?;
        let update_hash = self.l1.get_update_hash(start, end).await?;
        let native_update = self.l2.deserialize_sequencer_update(update.abi_encode()).await?;
        Ok(Some((update_hash, native_update)))
    }
}

#[cfg(test)]
mod test {
    use std::sync::Arc;

    use super::*;
    use crate::l1::types as l1types;
    use crate::l2::{types as l2types, PendingUpdate, PendingUpdateWithKeys};
    use hex_literal::hex;
    use mockall;
    use mockall::predicate::eq;
    use parity_scale_codec::Decode;
    use primitive_types::H256;
    use tracing_test::traced_test;

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
            async fn get_pending_updates(&self, at: H256) -> Result<Vec<PendingUpdateWithKeys>, L2Error>;
            async fn deserialize_sequencer_update(&self, data: Vec<u8>) -> Result<l2types::L1Update, L2Error>;
            async fn cancel_pending_request(&self, request_id: u128, chain: l2types::Chain) -> Result<bool, L2Error>;
            async fn update_l1_from_l2(&self, update: l2types::L1Update, hash: H256) -> Result<bool, L2Error>;
        }
    }

    const DUMMY_ADDRESS: [u8; 20] = hex!("0000000000000000000000000000000000000000");
    const ETHEREUM: l2types::Chain = l2types::Chain::Ethereum;
    const ARBITRUM: l2types::Chain = l2types::Chain::Arbitrum;

    enum Request {
        Deposit(l2types::Deposit),
        Cancel(l2types::CancelResolution),
    }

    impl From<l2types::Deposit> for Request {
        fn from(d: l2types::Deposit) -> Self {
            Self::Deposit(d)
        }
    }

    struct UpdateBuilder(Vec<Request>);

    pub fn to_u256(value: u128) -> l2types::bindings::runtime_types::primitive_types::U256 {
        let x = primitive_types::U256::from(7u128);
        let data = x.to_big_endian();
        l2types::bindings::runtime_types::primitive_types::U256::decode(& mut &data[..]).unwrap()
    }

    impl UpdateBuilder {
        fn new() -> Self {
            Self(vec![])
        }

        fn with_dummy_deposit(mut self) -> Self {
            self.with_request(l2types::Deposit {
                requestId: l2types::RequestId {
                    origin: l2types::Origin::L1,
                    id: 1u128,
                },
                depositRecipient: DUMMY_ADDRESS,
                tokenAddress: DUMMY_ADDRESS,
                amount: to_u256(100u128),
                timeStamp: to_u256(0u128),
                ferryTip: to_u256(0u128),
            }.into())
        }

        fn with_request(mut self, r: Request) -> Self {
            self.0.push(r);
            self
        }

        fn build(mut self, chain: l2types::Chain) -> l2types::L1Update {
            self.0.iter_mut().enumerate().for_each(|(idx, r)| match r {
                Request::Deposit(d) => {
                    d.requestId.id = (idx + 1) as u128;
                }
                Request::Cancel(c) => {
                    c.requestId.id = (idx + 1) as u128;
                }
            });

            let mut result = l2types::L1Update {
                chain,
                pendingDeposits: vec![],
                pendingCancelResolutions: vec![],
            };

            for elem in self.0.into_iter() {
                match elem {
                    Request::Deposit(d) => {
                        result.pendingDeposits.push(d);
                    }
                    Request::Cancel(c) => {
                        result.pendingCancelResolutions.push(c);
                    }
                }
            }

            result
        }
    }


    #[tokio::test]
    async fn test_find_malicious_update_ignores_valid_updates() {
        let update_hash =  H256::zero();
        let correct_hash =  update_hash.clone();

        let update = UpdateBuilder::new()
            .with_dummy_deposit()
            .build(ETHEREUM);
        let pending : PendingUpdateWithKeys = (1u128, update, update_hash);

        let mut l1mock = MockL1::new();
        l1mock
            .expect_get_update_hash()
            .with(eq(1u128), eq(1u128))
            .times(1)
            .returning(move |_, _| Ok(correct_hash));

        let mut l2mock = MockL2::new();
        l2mock
            .expect_get_pending_updates()
            .times(1)
            .return_once(move |_| Ok(vec![pending]));

        let sequencer = Sequencer::new(l1mock, l2mock, ETHEREUM);

        assert_eq!(
            sequencer.find_malicious_update(H256::zero()).await.unwrap(),
            None
        );

    }

    #[tokio::test]
    async fn test_find_malicious_update_ignores_updates_from_other_chains() {
        let update_hash =  H256::zero();
        let correct_hash =  update_hash.clone();

        let update = UpdateBuilder::new()
            .with_dummy_deposit()
            .build(ARBITRUM);
        let pending : PendingUpdateWithKeys = (1u128, update, update_hash);

        let mut l1mock = MockL1::new();
        l1mock
            .expect_get_update_hash()
            .times(0);

        let mut l2mock = MockL2::new();
        l2mock
            .expect_get_pending_updates()
            .times(1)
            .return_once(move |_| Ok(vec![pending]));

        let sequencer = Sequencer::new(l1mock, l2mock, ETHEREUM);

        assert_eq!(
            sequencer.find_malicious_update(H256::zero()).await.unwrap(),
            None
        );

    }

    #[tokio::test]
    async fn test_find_malicious_update_works() {
        let update_hash =  H256::from(hex!("1111111111111111111111111111111111111111111111111111111111111111"));
        let correct_hash =  H256::zero();

        let update = UpdateBuilder::new()
            .with_dummy_deposit()
            .build(ETHEREUM);
        let pending : PendingUpdateWithKeys = (1u128, update, update_hash);

        let mut l1mock = MockL1::new();
        l1mock
            .expect_get_update_hash()
            .with(eq(1u128), eq(1u128))
            .times(1)
            .returning(move |_, _| Ok(correct_hash));

        let mut l2mock = MockL2::new();
        l2mock
            .expect_get_pending_updates()
            .times(1)
            .return_once(move |_| Ok(vec![pending]));

        let sequencer = Sequencer::new(l1mock, l2mock, ETHEREUM);

        assert_eq!(
            sequencer.find_malicious_update(H256::zero()).await.unwrap(),
            Some(1u128)
        );
    }
    //
    // #[tokio::test]
    // async fn test_find_malicious_update_works() {
    //     let update_hash =  H256::from(hex!("1111111111111111111111111111111111111111111111111111111111111111"));
    //     let correct_hash =  H256::zero();
    //
    //     let update = UpdateBuilder::new()
    //         .with_dummy_deposit()
    //         .build(ETHEREUM);
    //     let pending : PendingUpdateWithKeys = (1u128, update, update_hash);
    //
    //     let mut l1mock = MockL1::new();
    //     l1mock
    //         .expect_get_update_hash()
    //         .with(eq(1u128), eq(1u128))
    //         .times(1)
    //         .returning(move |_, _| Ok(correct_hash));
    //
    //     let mut l2mock = MockL2::new();
    //     l2mock
    //         .expect_get_pending_updates()
    //         .times(1)
    //         .return_once(move |_| Ok(vec![pending]));
    //
    //     let sequencer = Sequencer::new(l1mock, l2mock, ETHEREUM);
    //
    //     assert_eq!(
    //         sequencer.find_malicious_update(H256::zero()).await.unwrap(),
    //         Some(1u128)
    //     );
    // }
}
