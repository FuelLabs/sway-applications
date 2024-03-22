use crate::utils::setup::{Arbiter, Asset, User};
use fuels::{
    prelude::{AssetId, CallParameters, TxPolicies},
    programs::{call_response::FuelCallResponse, call_utils::TxDependencyExtension},
    types::Identity,
};

pub(crate) async fn accept_arbiter(caller: &User, identifier: u64) -> FuelCallResponse<()> {
    caller
        .contract
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
    asset: &AssetId,
    assets: Vec<Asset>,
    buyer: &User,
    caller: &User,
    deadline: u64,
) -> FuelCallResponse<()> {
    let tx_params = TxPolicies::new(Some(0), Some(2_000_000), None, None, None);
    let call_params = CallParameters::new(amount, *asset, 1_000_000);

    caller
        .contract
        .methods()
        .create_escrow(
            arbiter.clone(),
            assets,
            Identity::Address(buyer.wallet.address().into()),
            deadline,
        )
        .with_tx_policies(tx_params)
        .call_params(call_params)
        .unwrap()
        .call()
        .await
        .unwrap()
}

pub(crate) async fn deposit(
    amount: u64,
    asset: &AssetId,
    caller: &User,
    identifier: u64,
) -> FuelCallResponse<()> {
    let tx_params = TxPolicies::new(Some(0), Some(2_000_000), None, None, None);
    let call_params = CallParameters::new(amount, *asset, 1_000_000);

    caller
        .contract
        .methods()
        .deposit(identifier)
        .with_tx_policies(tx_params)
        .call_params(call_params)
        .unwrap()
        .call()
        .await
        .unwrap()
}

pub(crate) async fn dispute(caller: &User, identifier: u64) -> FuelCallResponse<()> {
    caller
        .contract
        .methods()
        .dispute(identifier)
        .call()
        .await
        .unwrap()
}

pub(crate) async fn propose_arbiter(
    arbiter: Arbiter,
    caller: &User,
    identifier: u64,
) -> FuelCallResponse<()> {
    let tx_params = TxPolicies::new(Some(0), Some(2_000_000), None, None, None);
    let call_params = CallParameters::new(arbiter.fee_amount, arbiter.asset, 1_000_000);

    caller
        .contract
        .methods()
        .propose_arbiter(arbiter, identifier)
        .with_tx_policies(tx_params)
        .call_params(call_params)
        .unwrap()
        .append_variable_outputs(1)
        .call()
        .await
        .unwrap()
}

pub(crate) async fn resolve_dispute(
    caller: &User,
    identifier: u64,
    payment_amount: u64,
    user: &User,
) -> FuelCallResponse<()> {
    caller
        .contract
        .methods()
        .resolve_dispute(
            identifier,
            payment_amount,
            Identity::Address(user.wallet.address().into()),
        )
        .append_variable_outputs(4)
        .call()
        .await
        .unwrap()
}

pub(crate) async fn return_deposit(caller: &User, identifier: u64) -> FuelCallResponse<()> {
    caller
        .contract
        .methods()
        .return_deposit(identifier)
        .append_variable_outputs(3)
        .call()
        .await
        .unwrap()
}

pub(crate) async fn take_payment(caller: &User, identifier: u64) -> FuelCallResponse<()> {
    caller
        .contract
        .methods()
        .take_payment(identifier)
        .append_variable_outputs(3)
        .call()
        .await
        .unwrap()
}

pub(crate) async fn transfer_to_seller(caller: &User, identifier: u64) -> FuelCallResponse<()> {
    caller
        .contract
        .methods()
        .transfer_to_seller(identifier)
        .append_variable_outputs(3)
        .call()
        .await
        .unwrap()
}

pub(crate) async fn withdraw_collateral(caller: &User, identifier: u64) -> FuelCallResponse<()> {
    caller
        .contract
        .methods()
        .withdraw_collateral(identifier)
        .append_variable_outputs(2)
        .call()
        .await
        .unwrap()
}
