use crate::utils::setup::{NameRegistry, RegistrationValidityError};
use fuels::{
    prelude::{ContractId, WalletUnlocked},
    programs::call_response::FuelCallResponse,
    types::{Identity, SizedAsciiString},
};

pub(crate) async fn rate(instance: &NameRegistry<WalletUnlocked>, id: ContractId) -> Option<u64> {
    instance.methods().rate(id).call().await.unwrap().value
}

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
