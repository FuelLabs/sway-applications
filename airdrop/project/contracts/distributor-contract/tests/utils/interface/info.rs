use crate::utils::setup::{AirdropDistributor, ClaimData};
use fuels::types::{Bits256, Identity};

pub(crate) async fn claim_data(contract: &AirdropDistributor, identity: Identity) -> ClaimData {
    contract
        .methods()
        .claim_data(identity)
        .call()
        .await
        .unwrap()
        .value
}

pub(crate) async fn end_block(contract: &AirdropDistributor) -> u64 {
    contract.methods().end_block().call().await.unwrap().value
}

pub(crate) async fn merkle_root(contract: &AirdropDistributor) -> Bits256 {
    contract.methods().merkle_root().call().await.unwrap().value
}
