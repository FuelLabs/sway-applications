use crate::utils::setup::{get_timestamp_and_call, NameRegistry};
use fuels::{
    prelude::CallParameters,
    programs::call_response::FuelCallResponse,
    types::{Identity, SizedAsciiString},
};

pub(crate) async fn extend(
    instance: &NameRegistry,
    name: &String,
    duration: u64,
) -> FuelCallResponse<()> {
    instance
        .methods()
        .extend(
            SizedAsciiString::<8>::new(name.to_owned()).unwrap(),
            duration,
        )
        .call_params(CallParameters::new(Some(100), None, None))
        .call()
        .await
        .unwrap()
}

pub(crate) async fn extend_with_time(
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
            .call_params(CallParameters::new(Some(100), None, None)),
    )
    .await
}

pub(crate) async fn register(
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
        .call_params(CallParameters::new(Some(100), None, None))
        .call()
        .await
        .unwrap()
}

pub(crate) async fn register_with_time(
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
            .call_params(CallParameters::new(Some(100), None, None)),
    )
    .await
}

pub(crate) async fn set_identity(
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

pub(crate) async fn set_owner(
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
