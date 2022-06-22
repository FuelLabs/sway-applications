library events;

use std::{contract_id::ContractId, identity::Identity, vec::Vec};

pub struct ExecutedEvent {
    data: Vec<u64>,
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
