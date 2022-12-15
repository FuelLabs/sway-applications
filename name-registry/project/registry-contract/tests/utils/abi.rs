use core::fmt::Debug;
use fuels::{
    contract::{call_response::FuelCallResponse, contract::ContractCallHandler},
    prelude::*,
    tx::UniqueIdentifier,
    // TODO: remove?
    // client::types::TransactionStatus,
    types::transaction_response::TransactionStatus,
};

use crate::utils::{NameRegistry, RegistrationValidityError};

async fn get_timestamp_and_call<T>(handler: ContractCallHandler<T>) -> (FuelCallResponse<T>, u64)
where
    T: Tokenizable + Debug,
{
    let script = handler.get_executable_call().await.unwrap();
    let tx_id = script.tx.id().to_string();
    let provider = handler.provider.clone();
    let call_response = handler.call().await.unwrap();
    let tx_status = provider
        .get_transaction_by_id(&tx_id)
        .await
        .unwrap()
        .unwrap();

    match tx_status.status {
        TransactionStatus::Success() => (),
        _ => panic!("tx failed"),
    }

    // TODO: this needs to be updated / reverted when the SDK fixes their breaking changes
    let time = 5;

    (call_response, time)
}

pub async fn extend(
    instance: &NameRegistry,
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
            .call_params(CallParameters {
                amount: duration / 100,
                asset_id: AssetId::BASE,
                gas_forwarded: Some(1000000),
            }),
    )
    .await
}

pub async fn expiry(
    instance: &NameRegistry,
    name: &String,
) -> (
    FuelCallResponse<Result<u64, RegistrationValidityError>>,
    u64,
) {
    get_timestamp_and_call(
        instance
            .methods()
            .expiry(SizedAsciiString::<8>::new(name.to_owned()).unwrap()),
    )
    .await
}

pub async fn identity(
    instance: &NameRegistry,
    name: &String,
) -> (
    FuelCallResponse<Result<Identity, RegistrationValidityError>>,
    u64,
) {
    get_timestamp_and_call(
        instance
            .methods()
            .identity(SizedAsciiString::<8>::new(name.to_owned()).unwrap()),
    )
    .await
}

pub async fn owner(
    instance: &NameRegistry,
    name: &String,
) -> (
    FuelCallResponse<Result<Identity, RegistrationValidityError>>,
    u64,
) {
    get_timestamp_and_call(
        instance
            .methods()
            .owner(SizedAsciiString::<8>::new(name.to_owned()).unwrap()),
    )
    .await
}

pub async fn register(
    instance: &NameRegistry,
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
            .call_params(CallParameters {
                amount: duration / 100,
                asset_id: AssetId::BASE,
                gas_forwarded: Some(1000000),
            }),
    )
    .await
}

pub async fn set_identity(
    instance: &NameRegistry,
    name: &String,
    identity: Identity,
) -> (FuelCallResponse<()>, u64) {
    get_timestamp_and_call(instance.methods().set_identity(
        SizedAsciiString::<8>::new(name.to_owned()).unwrap(),
        identity,
    ))
    .await
}

pub async fn set_owner(
    instance: &NameRegistry,
    name: &String,
    new_owner: Identity,
) -> (FuelCallResponse<()>, u64) {
    get_timestamp_and_call(instance.methods().set_owner(
        SizedAsciiString::<8>::new(name.to_owned()).unwrap(),
        new_owner,
    ))
    .await
}
