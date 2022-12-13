use fuels::{
    contract::call_response::FuelCallResponse,
    prelude::*,
    tx::{AssetId, ContractId},
};

use crate::utils::setup::{Coin, Fundraiser};

pub async fn cancel_campaign(contract: &Fundraiser, id: u64) -> FuelCallResponse<()> {
    contract.methods().cancel_campaign(id).call().await.unwrap()
}

pub async fn claim_pledges(contract: &Fundraiser, id: u64) -> FuelCallResponse<()> {
    contract
        .methods()
        .claim_pledges(id)
        .append_variable_outputs(1)
        .call()
        .await
        .unwrap()
}

pub async fn create_campaign(
    contract: &Fundraiser,
    asset: &ContractId,
    beneficiary: &Identity,
    deadline: u64,
    target_amount: u64,
) -> FuelCallResponse<()> {
    contract
        .methods()
        .create_campaign(asset.clone(), beneficiary.clone(), deadline, target_amount)
        .call()
        .await
        .unwrap()
}

pub async fn pledge(
    contract: &Fundraiser,
    id: u64,
    asset: &Coin,
    amount: u64,
) -> FuelCallResponse<()> {
    let tx_params = TxParameters::new(None, Some(1_000_000), None);
    let call_params = CallParameters::new(Some(amount), Some(AssetId::from(*asset.id)), None);

    contract
        .methods()
        .pledge(id)
        .tx_params(tx_params)
        .call_params(call_params)
        .call()
        .await
        .unwrap()
}

pub async fn unpledge(contract: &Fundraiser, id: u64, amount: u64) -> FuelCallResponse<()> {
    contract
        .methods()
        .unpledge(id, amount)
        .append_variable_outputs(1)
        .call()
        .await
        .unwrap()
}
