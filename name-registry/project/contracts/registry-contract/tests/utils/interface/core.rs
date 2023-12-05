use crate::utils::setup::{get_timestamp_and_call, NameRegistry};
use fuels::{
    prelude::{AssetId, CallParameters, TxPolicies, WalletUnlocked},
    programs::call_response::FuelCallResponse,
    types::Identity,
};

pub(crate) async fn extend(
    instance: &NameRegistry<WalletUnlocked>,
    name: String,
    duration: u64,
    payment_asset: AssetId,
) -> FuelCallResponse<()> {
    let call_params = CallParameters::new(100, payment_asset, 1_000_000);

    instance
        .methods()
        .extend(name, duration)
        .with_tx_policies(TxPolicies::default().with_script_gas_limit(2_000_000))
        .call_params(call_params)
        .unwrap()
        .call()
        .await
        .unwrap()
}

pub(crate) async fn extend_with_time(
    instance: &NameRegistry<WalletUnlocked>,
    name: String,
    duration: u64,
    payment_asset: AssetId,
) -> (FuelCallResponse<()>, u64) {
    let call_params = CallParameters::new(100, payment_asset, 0);

    get_timestamp_and_call(
        instance
            .methods()
            .extend(name, duration)
            .call_params(call_params)
            .unwrap(),
    )
    .await
}

pub(crate) async fn register(
    instance: &NameRegistry<WalletUnlocked>,
    name: String,
    duration: u64,
    owner: &Identity,
    identity: &Identity,
    payment_asset: AssetId,
) -> FuelCallResponse<()> {
    let call_params = CallParameters::new(100, payment_asset, 1_000_000);

    instance
        .methods()
        .register(name, duration, owner.to_owned(), identity.to_owned())
        .with_tx_policies(TxPolicies::default().with_script_gas_limit(2_000_000))
        .call_params(call_params)
        .unwrap()
        .call()
        .await
        .unwrap()
}

pub(crate) async fn register_with_time(
    instance: &NameRegistry<WalletUnlocked>,
    name: String,
    duration: u64,
    owner: &Identity,
    identity: &Identity,
    payment_asset: AssetId,
) -> (FuelCallResponse<()>, u64) {
    let call_params = CallParameters::new(100, payment_asset, 0);

    get_timestamp_and_call(
        instance
            .methods()
            .register(name, duration, owner.to_owned(), identity.to_owned())
            .call_params(call_params)
            .unwrap(),
    )
    .await
}

pub(crate) async fn set_asset(
    instance: &NameRegistry<WalletUnlocked>,
    id: AssetId,
    rate: Option<u64>,
) -> FuelCallResponse<()> {
    instance.methods().set_asset(id, rate).call().await.unwrap()
}

pub(crate) async fn set_identity(
    instance: &NameRegistry<WalletUnlocked>,
    name: String,
    identity: Identity,
) -> FuelCallResponse<()> {
    instance
        .methods()
        .set_resolver(name, identity)
        .call()
        .await
        .unwrap()
}

pub(crate) async fn set_owner(
    instance: &NameRegistry<WalletUnlocked>,
    name: String,
    new_owner: Identity,
) -> FuelCallResponse<()> {
    instance
        .methods()
        .transfer_name_ownership(name, new_owner)
        .call()
        .await
        .unwrap()
}
