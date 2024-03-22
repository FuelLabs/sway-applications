use crate::utils::setup::{NameRegistry, RegistrationValidityError};
use fuels::{
    prelude::{AssetId, WalletUnlocked},
    programs::call_response::FuelCallResponse,
    types::Identity,
};

pub(crate) async fn rate(instance: &NameRegistry<WalletUnlocked>, id: AssetId) -> Option<u64> {
    instance.methods().rate(id).call().await.unwrap().value
}

pub(crate) async fn expiry(
    instance: &NameRegistry<WalletUnlocked>,
    name: String,
) -> FuelCallResponse<Result<u64, RegistrationValidityError>> {
    instance.methods().expiry(name).call().await.unwrap()
}

pub(crate) async fn identity(
    instance: &NameRegistry<WalletUnlocked>,
    name: String,
) -> FuelCallResponse<Result<Identity, RegistrationValidityError>> {
    instance.methods().resolver(name).call().await.unwrap()
}

pub(crate) async fn owner(
    instance: &NameRegistry<WalletUnlocked>,
    name: String,
) -> FuelCallResponse<Result<Identity, RegistrationValidityError>> {
    instance.methods().name_owner(name).call().await.unwrap()
}
