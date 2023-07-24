use fuels::{
    prelude::{AssetId, CallParameters, ContractId, TxParameters, WalletUnlocked},
    programs::call_response::FuelCallResponse,
    types::Identity,
};

use crate::utils::setup::{Coin, Fundraiser};

pub(crate) async fn cancel_campaign(
    contract: &Fundraiser<WalletUnlocked>,
    id: u64,
) -> FuelCallResponse<()> {
    contract.methods().cancel_campaign(id).call().await.unwrap()
}

pub(crate) async fn claim_pledges(
    contract: &Fundraiser<WalletUnlocked>,
    id: u64,
) -> FuelCallResponse<()> {
    contract
        .methods()
        .claim_pledges(id)
        .append_variable_outputs(1)
        .call()
        .await
        .unwrap()
}

pub(crate) async fn create_campaign(
    contract: &Fundraiser<WalletUnlocked>,
    asset: &ContractId,
    beneficiary: &Identity,
    deadline: u64,
    target_amount: u64,
) -> FuelCallResponse<()> {
    contract
        .methods()
        .create_campaign(*asset, beneficiary.clone(), deadline, target_amount)
        .call()
        .await
        .unwrap()
}

pub(crate) async fn pledge(
    contract: &Fundraiser<WalletUnlocked>,
    id: u64,
    asset: &Coin,
    amount: u64,
) -> FuelCallResponse<()> {
    let tx_params = TxParameters::new(0, 2_000_000, 0);
    let call_params = CallParameters::new(amount, AssetId::from(*asset.id), 1_000_000);

    contract
        .methods()
        .pledge(id)
        .tx_params(tx_params)
        .call_params(call_params)
        .unwrap()
        .call()
        .await
        .unwrap()
}

pub(crate) async fn unpledge(
    contract: &Fundraiser<WalletUnlocked>,
    id: u64,
    amount: u64,
) -> FuelCallResponse<()> {
    contract
        .methods()
        .unpledge(id, amount)
        .append_variable_outputs(1)
        .call()
        .await
        .unwrap()
}
