library events;

use std::{contract_id::ContractId, identity::Identity};

pub struct ExecutedEvent {
    data: [u64; 3],
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
