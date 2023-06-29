use fuels::{contract::call_response::FuelCallResponse, prelude::*};

use crate::utils::setup::{Asset, Bytes, Identity, Timelock};

pub async fn cancel(contract: &Timelock, id: u64) -> FuelCallResponse<()> {
    contract.methods().cancel(id).call().await.unwrap()
}

pub async fn execute(
    contract: &Timelock,
    recipient: &Identity,
    asset: Option<Asset>,
    data: Bytes,
    timestamp: u64,
) -> FuelCallResponse<()> {
    contract
        .methods()
        .execute(recipient.clone(), asset, data, timestamp)
        .append_variable_outputs(1)
        .call()
        .await
        .unwrap()
}

pub async fn queue(
    contract: &Timelock,
    recipient: &Identity,
    asset: Option<Asset>,
    data: Bytes,
    timestamp: u64,
) -> FuelCallResponse<()> {
    contract.methods().queue(recipient.clone(), asset, data, timestamp).call().await.unwrap()
}
