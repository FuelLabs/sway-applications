use crate::utils::setup::{NameRegistry, RegistrationValidityError, String};
use fuels::{
    accounts::wallet::WalletUnlocked, programs::call_response::FuelCallResponse, types::Identity,
};

pub(crate) async fn expiry(
    instance: &NameRegistry<WalletUnlocked>,
    name: &String,
) -> FuelCallResponse<Result<u64, RegistrationValidityError>> {
    instance
        .methods()
        .expiry(name.to_owned())
        .call()
        .await
        .unwrap()
}

pub(crate) async fn identity(
    instance: &NameRegistry<WalletUnlocked>,
    name: &String,
) -> FuelCallResponse<Result<Identity, RegistrationValidityError>> {
    instance
        .methods()
        .identity(name.to_owned())
        .call()
        .await
        .unwrap()
}

pub(crate) async fn owner(
    instance: &NameRegistry<WalletUnlocked>,
    name: &String,
) -> FuelCallResponse<Result<Identity, RegistrationValidityError>> {
    instance
        .methods()
        .owner(name.to_owned())
        .call()
        .await
        .unwrap()
}
