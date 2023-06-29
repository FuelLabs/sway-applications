use crate::utils::setup::{AirdropDistributor, ClaimState};
use fuels::{
    prelude::WalletUnlocked,
    types::{Bits256, Identity},
};

pub(crate) async fn admin(contract: &AirdropDistributor<WalletUnlocked>) -> Option<Identity> {
    contract.methods().admin().call().await.unwrap().value
}

pub(crate) async fn claim_data(
    contract: &AirdropDistributor<WalletUnlocked>,
    identity: Identity,
) -> ClaimState {
    contract
        .methods()
        .claim_data(identity)
        .call()
        .await
        .unwrap()
        .value
}

pub(crate) async fn end_block(contract: &AirdropDistributor<WalletUnlocked>) -> u64 {
    contract.methods().end_block().call().await.unwrap().value
}

pub(crate) async fn is_active(contract: &AirdropDistributor<WalletUnlocked>) -> bool {
    contract.methods().is_active().call().await.unwrap().value
}

pub(crate) async fn merkle_root(contract: &AirdropDistributor<WalletUnlocked>) -> Option<Bits256> {
    contract.methods().merkle_root().call().await.unwrap().value
}

pub(crate) async fn number_of_leaves(contract: &AirdropDistributor<WalletUnlocked>) -> u64 {
    contract
        .methods()
        .number_of_leaves()
        .call()
        .await
        .unwrap()
        .value
}
