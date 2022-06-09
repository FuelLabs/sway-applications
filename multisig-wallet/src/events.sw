library events;

use std::{contract_id::ContractId, identity::Identity};

pub struct ExecutedEvent {
    to: Identity,
    value: u64,
    data: b256, // TODO: change to vector when implemented
    nonce: u64,
}

pub struct TransferEvent {
    to: Identity,
    asset: ContractId,
    value: u64,
    nonce: u64,
}
