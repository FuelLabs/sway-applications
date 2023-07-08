use crate::utils::setup::{get_timestamp_and_call, NameRegistry};
use fuels::{
    prelude::{CallParameters, ContractId, TxParameters, WalletUnlocked},
    programs::call_response::FuelCallResponse,
    types::{AssetId, Identity, SizedAsciiString},
};

pub(crate) async fn extend(
    instance: &NameRegistry<WalletUnlocked>,
    name: &String,
    duration: u64,
) -> FuelCallResponse<()> {
    instance
        .methods()
        .extend(
            SizedAsciiString::<8>::new(name.to_owned()).unwrap(),
            duration,
        )
        .tx_params(TxParameters::new(0, 2_000_000, 0))
        .call_params(CallParameters::new(100, AssetId::new([0u8; 32]), 1_000_000))
        .unwrap()
        .call()
        .await
        .unwrap()
}

pub(crate) async fn extend_with_time(
    instance: &NameRegistry<WalletUnlocked>,
    name: &String,
    duration: u64,
) -> (FuelCallResponse<()>, u64) {
    get_timestamp_and_call(
        instance
            .methods()
            .extend(
                SizedAsciiString::<8>::new(name.to_owned()).unwrap(),
                duration,
            )
            .call_params(CallParameters::new(100, AssetId::new([0u8; 32]), 0))
            .unwrap(),
    )
    .await
}

pub(crate) async fn register(
    instance: &NameRegistry<WalletUnlocked>,
    name: &String,
    duration: u64,
    owner: &Identity,
    identity: &Identity,
) -> FuelCallResponse<()> {
    instance
        .methods()
        .register(
            SizedAsciiString::<8>::new(name.to_owned()).unwrap(),
            duration,
            owner.to_owned(),
            identity.to_owned(),
        )
        .tx_params(TxParameters::new(0, 2_000_000, 0))
        .call_params(CallParameters::new(100, AssetId::new([0u8; 32]), 1_000_000))
        .unwrap()
        .call()
        .await
        .unwrap()
}

pub(crate) async fn register_with_time(
    instance: &NameRegistry<WalletUnlocked>,
    name: &String,
    duration: u64,
    owner: &Identity,
    identity: &Identity,
) -> (FuelCallResponse<()>, u64) {
    get_timestamp_and_call(
        instance
            .methods()
            .register(
                SizedAsciiString::<8>::new(name.to_owned()).unwrap(),
                duration,
                owner.to_owned(),
                identity.to_owned(),
            )
            .call_params(CallParameters::new(100, AssetId::new([0u8; 32]), 0))
            .unwrap(),
    )
    .await
}

pub(crate) async fn set_asset(
    instance: &NameRegistry<WalletUnlocked>,
    id: ContractId,
    rate: Option<u64>,
) -> FuelCallResponse<()> {
    instance.methods().set_asset(id, rate).call().await.unwrap()
}

pub(crate) async fn set_identity(
    instance: &NameRegistry<WalletUnlocked>,
    name: &String,
    identity: Identity,
) -> FuelCallResponse<()> {
    instance
        .methods()
        .set_identity(
            SizedAsciiString::<8>::new(name.to_owned()).unwrap(),
            identity,
        )
        .call()
        .await
        .unwrap()
}

pub(crate) async fn set_owner(
    instance: &NameRegistry<WalletUnlocked>,
    name: &String,
    new_owner: Identity,
) -> FuelCallResponse<()> {
    instance
        .methods()
        .set_owner(
            SizedAsciiString::<8>::new(name.to_owned()).unwrap(),
            new_owner,
        )
        .call()
        .await
        .unwrap()
}
