use fuels::{contract::call_response::FuelCallResponse, prelude::*};

use crate::utils::setup::{ExecutionRange, Timelock};

pub async fn queued(contract: &Timelock, id: u64) -> FuelCallResponse<Option<ExecutionRange>> {
    contract.methods().queued(id).call().await.unwrap()
}

pub async fn transaction_hash(
    contract: &Timelock,
    recipient: &Identity,
    value: u64,
    data: Bytes,
    timestamp: u64,
) -> FuelCallResponse<[u64; 32]> {
    contract
        .methods()
        .transaction_hash(recipient.clone(), value, data, timestamp)
        .call()
        .await
        .unwrap()
}
