use core::fmt::Debug;
use fuels::{
    client::types::TransactionStatus,
    contract::contract::{CallResponse, ContractCallHandler},
};

use crate::utils::*;

async fn get_timestamp_and_call<T>(handler: ContractCallHandler<T>) -> (CallResponse<T>, u64)
where
    T: Tokenizable + Debug,
{
    let script = handler.get_call_execution_script().await.unwrap();
    let tx_id = script.tx.id().to_string();
    let provider = handler.provider.clone();
    let call_response = handler.call().await.unwrap();
    let tx_status = provider.get_transaction_by_id(&tx_id).await.unwrap().status;

    let time = match tx_status {
        TransactionStatus::Success { time, .. } => time,
        _ => panic!("tx failed"),
    };
    let time = time.timestamp() as u64;

    (call_response, time)
}

pub async fn extend(
    instance: &NameRegistry,
    name: &String,
    duration: u64,
) -> (CallResponse<()>, u64) {
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

pub async fn expiry(instance: &NameRegistry, name: &String) -> (CallResponse<Result<u64, RegistrationValidityError>>, u64) {
    get_timestamp_and_call(
        instance
            .methods()
            .expiry(SizedAsciiString::<8>::new(name.to_owned()).unwrap()),
    )
    .await
}

pub async fn identity(instance: &NameRegistry, name: &String) -> (CallResponse<Result<Identity, RegistrationValidityError>>, u64) {
    get_timestamp_and_call(
        instance
            .methods()
            .identity(SizedAsciiString::<8>::new(name.to_owned()).unwrap()),
    )
    .await
}

pub async fn owner(instance: &NameRegistry, name: &String) -> (CallResponse<Result<Identity, RegistrationValidityError>>, u64) {
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
) -> (CallResponse<()>, u64) {
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
) -> (CallResponse<()>, u64) {
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
) -> (CallResponse<()>, u64) {
    get_timestamp_and_call(instance.methods().set_owner(
        SizedAsciiString::<8>::new(name.to_owned()).unwrap(),
        new_owner,
    ))
    .await
}
