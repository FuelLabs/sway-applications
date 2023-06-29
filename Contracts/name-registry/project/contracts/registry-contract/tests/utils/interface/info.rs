use crate::utils::setup::{NameRegistry, RegistrationValidityError};
use fuels::{
    prelude::WalletUnlocked,
    programs::call_response::FuelCallResponse,
    types::{Identity, SizedAsciiString},
};

pub(crate) async fn expiry(
    instance: &NameRegistry<WalletUnlocked>,
    name: &String,
) -> FuelCallResponse<Result<u64, RegistrationValidityError>> {
    instance
        .methods()
        .expiry(SizedAsciiString::<8>::new(name.to_owned()).unwrap())
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
        .identity(SizedAsciiString::<8>::new(name.to_owned()).unwrap())
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
        .owner(SizedAsciiString::<8>::new(name.to_owned()).unwrap())
        .call()
        .await
        .unwrap()
}
