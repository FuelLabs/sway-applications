use crate::utils::setup::{DaoVoting, ProposalInfo, Votes};
use fuels::{
    prelude::{Bech32Address, WalletUnlocked},
    types::{AssetId, Identity},
};

pub(crate) async fn balance(contract: &DaoVoting<WalletUnlocked>) -> u64 {
    contract.methods().balance().call().await.unwrap().value
}

pub(crate) async fn user_balance(
    contract: &DaoVoting<WalletUnlocked>,
    user_identity: &Bech32Address,
) -> u64 {
    contract
        .methods()
        .user_balance(Identity::Address(user_identity.into()))
        .call()
        .await
        .unwrap()
        .value
}

pub(crate) async fn user_votes(
    contract: &DaoVoting<WalletUnlocked>,
    user_identity: &Bech32Address,
    id: u64,
) -> Votes {
    contract
        .methods()
        .user_votes(id, Identity::Address(user_identity.into()))
        .call()
        .await
        .unwrap()
        .value
}

pub(crate) async fn proposal(contract: &DaoVoting<WalletUnlocked>, id: u64) -> ProposalInfo {
    contract.methods().proposal(id).call().await.unwrap().value
}

pub(crate) async fn governance_asset_id(contract: &DaoVoting<WalletUnlocked>) -> AssetId {
    contract
        .methods()
        .governance_asset_id()
        .call()
        .await
        .unwrap()
        .value
}

pub(crate) async fn proposal_count(contract: &DaoVoting<WalletUnlocked>) -> u64 {
    contract
        .methods()
        .proposal_count()
        .call()
        .await
        .unwrap()
        .value
}
