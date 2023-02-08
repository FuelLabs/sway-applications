use core::fmt::Debug;
use fuels::{
    core::traits::Tokenizable,
    prelude::*,
    programs::{call_response::FuelCallResponse, contract::ContractCallHandler},
    types::{Identity, SizedAsciiString},
    // tx::UniqueIdentifier,
    // types::transaction_response::TransactionStatus,
};

use crate::utils::{NameRegistry, RegistrationValidityError};

async fn get_timestamp_and_call<T>(handler: ContractCallHandler<T>) -> (FuelCallResponse<T>, u64)
where
    T: Tokenizable + Debug,
{
    let call_response = handler.call().await.unwrap();

    // TODO: this needs to be updated / reverted when the SDK fixes their breaking changes
    // let script = handler.get_executable_call().await.unwrap();
    // let provider = handler.provider.clone();
    // let tx_id = script.tx.id().to_string();
    // let tx_status = provider
    //     .get_transaction_by_id(&tx_id)
    //     .await
    //     .unwrap()
    //     .unwrap();

    // let time = match tx_status.status {
    //     TransactionStatus::Success() => ( /* get time from here like before */ ),
    //     _ => panic!("tx failed"),
    // }

    let time = 5;

    (call_response, time)
}

pub async fn extend(instance: &NameRegistry, name: &String, duration: u64) -> FuelCallResponse<()> {
    instance
        .methods()
        .extend(
            SizedAsciiString::<8>::new(name.to_owned()).unwrap(),
            duration,
        )
        .tx_params(TxParameters::default())
        .call_params(CallParameters::new(Some(100), None, None))
        .call()
        .await
        .unwrap()
}

pub async fn extend_with_time(
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
            .tx_params(TxParameters::default())
            .call_params(CallParameters::new(Some(100), None, None)),
    )
    .await
}

pub async fn expiry(
    instance: &NameRegistry,
    name: &String,
) -> FuelCallResponse<Result<u64, RegistrationValidityError>> {
    instance
        .methods()
        .expiry(SizedAsciiString::<8>::new(name.to_owned()).unwrap())
        .call()
        .await
        .unwrap()
}

pub async fn identity(
    instance: &NameRegistry,
    name: &String,
) -> FuelCallResponse<Result<Identity, RegistrationValidityError>> {
    instance
        .methods()
        .identity(SizedAsciiString::<8>::new(name.to_owned()).unwrap())
        .call()
        .await
        .unwrap()
}

pub async fn owner(
    instance: &NameRegistry,
    name: &String,
) -> FuelCallResponse<Result<Identity, RegistrationValidityError>> {
    instance
        .methods()
        .owner(SizedAsciiString::<8>::new(name.to_owned()).unwrap())
        .call()
        .await
        .unwrap()
}

pub async fn register(
    instance: &NameRegistry,
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
        .tx_params(TxParameters::default())
        .call_params(CallParameters::new(Some(100), None, None))
        .call()
        .await
        .unwrap()
}

pub async fn register_with_time(
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
            .tx_params(TxParameters::default())
            .call_params(CallParameters::new(Some(100), None, None)),
    )
    .await
}

pub async fn set_identity(
    instance: &NameRegistry,
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

pub async fn set_owner(
    instance: &NameRegistry,
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
