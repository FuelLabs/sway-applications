use crate::utils::setup::{AirdropDistributor, ClaimState};
use fuels::types::{Bits256, Identity};

pub(crate) async fn admin(contract: &AirdropDistributor) -> Option<Identity> {
    contract.methods().admin().call().await.unwrap().value
}

pub(crate) async fn claim_data(contract: &AirdropDistributor, identity: Identity) -> ClaimState {
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

pub(crate) async fn is_active(contract: &AirdropDistributor) -> bool {
    contract.methods().is_active().call().await.unwrap().value
}

pub(crate) async fn merkle_root(contract: &AirdropDistributor) -> Option<Bits256> {
    contract.methods().merkle_root().call().await.unwrap().value
}

pub(crate) async fn number_of_leaves(contract: &AirdropDistributor) -> u64 {
    contract
        .methods()
        .number_of_leaves()
        .call()
        .await
        .unwrap()
        .value
}
