use crate::utils::setup::{DaoVoting, Proposal};
use fuels::{
    prelude::{CallParameters, ContractId, TxParameters},
    programs::call_response::FuelCallResponse,
};

pub(crate) async fn constructor(contract: &DaoVoting, token: ContractId) -> FuelCallResponse<()> {
    contract.methods().constructor(token).call().await.unwrap()
}

pub(crate) async fn create_proposal(
    contract: &DaoVoting,
    acceptance_percentage: u64,
    deadline: u64,
    proposal: Proposal,
) -> FuelCallResponse<()> {
    contract
        .methods()
        .create_proposal(acceptance_percentage, deadline, proposal)
        .call()
        .await
        .unwrap()
}

pub(crate) async fn deposit(
    contract: &DaoVoting,
    call_params: CallParameters,
) -> FuelCallResponse<()> {
    let tx_params = TxParameters::new(None, Some(1_000_000), None);
    contract
        .methods()
        .deposit()
        .tx_params(tx_params)
        .call_params(call_params)
        .call()
        .await
        .unwrap()
}

pub(crate) async fn withdraw(contract: &DaoVoting, amount: u64) -> FuelCallResponse<()> {
    contract
        .methods()
        .withdraw(amount)
        .append_variable_outputs(1)
        .call()
        .await
        .unwrap()
}

pub(crate) async fn vote(
    contract: &DaoVoting,
    approve: bool,
    proposal_id: u64,
    vote_amount: u64,
) -> FuelCallResponse<()> {
    contract
        .methods()
        .vote(approve, proposal_id, vote_amount)
        .call()
        .await
        .unwrap()
}

pub(crate) async fn execute(contract: &DaoVoting, id: u64) -> FuelCallResponse<()> {
    contract.methods().execute(id).call().await.unwrap()
}

pub(crate) async fn unlock_votes(contract: &DaoVoting, id: u64) -> FuelCallResponse<()> {
    contract.methods().unlock_votes(id).call().await.unwrap()
}
