use crate::utils::setup::{DaoVoting, ProposalInfo, Votes};
use fuels::{
    prelude::{Bech32Address, ContractId},
    types::Identity,
};

pub(crate) async fn balance(contract: &DaoVoting) -> u64 {
    contract.methods().balance().call().await.unwrap().value
}

pub(crate) async fn user_balance(contract: &DaoVoting, user_identity: &Bech32Address) -> u64 {
    contract
        .methods()
        .user_balance(Identity::Address(user_identity.into()))
        .call()
        .await
        .unwrap()
        .value
}

pub(crate) async fn user_votes(
    contract: &DaoVoting,
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

pub(crate) async fn proposal(contract: &DaoVoting, id: u64) -> ProposalInfo {
    contract.methods().proposal(id).call().await.unwrap().value
}

pub(crate) async fn governance_token_id(contract: &DaoVoting) -> ContractId {
    contract
        .methods()
        .governance_token_id()
        .call()
        .await
        .unwrap()
        .value
}

pub(crate) async fn proposal_count(contract: &DaoVoting) -> u64 {
    contract
        .methods()
        .proposal_count()
        .call()
        .await
        .unwrap()
        .value
}
