use fuels::{contract::call_response::FuelCallResponse, prelude::*};

use crate::utils::setup::Timelock;

pub async fn cancel(contract: &Timelock, id: u64) -> FuelCallResponse<()> {
    contract.methods().cancel(id).call().await.unwrap()
}

pub async fn execute(
    contract: &Timelock,
    recipient: &Identity,
    value: u64,
    data: Bytes,
    timestamp: u64,
) -> FuelCallResponse<()> {
    contract
        .methods()
        .execute(recipient.clone(), value, data, timestamp)
        .append_variable_outputs(1)
        .call()
        .await
        .unwrap()
}

pub async fn queue(contract: &Timelock, id: u64) -> FuelCallResponse<()> {
    contract.methods().queue(id).call().await.unwrap()
}
