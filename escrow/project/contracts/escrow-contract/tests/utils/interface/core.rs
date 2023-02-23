use crate::utils::setup::{Arbiter, Asset, Escrow};
use fuels::{
    prelude::{AssetId, Bech32Address, CallParameters, ContractId, TxParameters},
    programs::call_response::FuelCallResponse,
    types::Identity,
};

pub(crate) async fn accept_arbiter(contract: &Escrow, identifier: u64) -> FuelCallResponse<()> {
    contract
        .methods()
        .accept_arbiter(identifier)
        .append_variable_outputs(1)
        .call()
        .await
        .unwrap()
}

pub(crate) async fn create_escrow(
    amount: u64,
    arbiter: &Arbiter,
    asset: &ContractId,
    assets: Vec<Asset>,
    buyer: &Bech32Address,
    contract: &Escrow,
    deadline: u64,
) -> FuelCallResponse<()> {
    let tx_params = TxParameters::new(None, Some(1_000_000), None);
    let call_params =
        CallParameters::new(Some(amount), Some(AssetId::from(**asset)), Some(1_000_000));

    contract
        .methods()
        .create_escrow(
            arbiter.clone(),
            assets,
            Identity::Address(buyer.into()),
            deadline,
        )
        .tx_params(tx_params)
        .call_params(call_params)
        .unwrap()
        .call()
        .await
        .unwrap()
}

pub(crate) async fn deposit(
    amount: u64,
    asset: &ContractId,
    contract: &Escrow,
    identifier: u64,
) -> FuelCallResponse<()> {
    let tx_params = TxParameters::new(None, Some(1_000_000), None);
    let call_params =
        CallParameters::new(Some(amount), Some(AssetId::from(**asset)), Some(1_000_000));

    contract
        .methods()
        .deposit(identifier)
        .tx_params(tx_params)
        .call_params(call_params)
        .unwrap()
        .call()
        .await
        .unwrap()
}

pub(crate) async fn dispute(contract: &Escrow, identifier: u64) -> FuelCallResponse<()> {
    contract.methods().dispute(identifier).call().await.unwrap()
}

pub(crate) async fn propose_arbiter(
    arbiter: Arbiter,
    contract: &Escrow,
    identifier: u64,
) -> FuelCallResponse<()> {
    let tx_params = TxParameters::new(None, Some(1_000_000), None);
    let call_params = CallParameters::new(
        Some(arbiter.fee_amount),
        Some(AssetId::from(*arbiter.asset)),
        Some(1_000_000),
    );

    contract
        .methods()
        .propose_arbiter(arbiter, identifier)
        .tx_params(tx_params)
        .call_params(call_params)
        .unwrap()
        .append_variable_outputs(1)
        .call()
        .await
        .unwrap()
}

pub(crate) async fn resolve_dispute(
    contract: &Escrow,
    identifier: u64,
    payment_amount: u64,
    user: &Bech32Address,
) -> FuelCallResponse<()> {
    contract
        .methods()
        .resolve_dispute(identifier, payment_amount, Identity::Address(user.into()))
        .append_variable_outputs(4)
        .call()
        .await
        .unwrap()
}

pub(crate) async fn return_deposit(contract: &Escrow, identifier: u64) -> FuelCallResponse<()> {
    contract
        .methods()
        .return_deposit(identifier)
        .append_variable_outputs(3)
        .call()
        .await
        .unwrap()
}

pub(crate) async fn take_payment(contract: &Escrow, identifier: u64) -> FuelCallResponse<()> {
    contract
        .methods()
        .take_payment(identifier)
        .append_variable_outputs(3)
        .call()
        .await
        .unwrap()
}

pub(crate) async fn transfer_to_seller(contract: &Escrow, identifier: u64) -> FuelCallResponse<()> {
    contract
        .methods()
        .transfer_to_seller(identifier)
        .append_variable_outputs(3)
        .call()
        .await
        .unwrap()
}

pub(crate) async fn withdraw_collateral(
    contract: &Escrow,
    identifier: u64,
) -> FuelCallResponse<()> {
    contract
        .methods()
        .withdraw_collateral(identifier)
        .append_variable_outputs(2)
        .call()
        .await
        .unwrap()
}
