library events;

use std::{contract_id::ContractId, identity::Identity};

pub struct ExecutedEvent {
    data: b256, // TODO: change to Bytes when implemented.
    nonce: u64,
    to: Identity,
    value: u64,
}

pub struct TransferEvent {
    asset: ContractId,
    nonce: u64,
    to: Identity,
    value: u64,
}
