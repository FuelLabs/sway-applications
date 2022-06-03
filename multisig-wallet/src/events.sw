library events;

use std::{
    chain::auth::Sender,
    contract_id::ContractId,
};

pub struct ExecutedEvent {
    to: Sender, 
    value: u64, 
    data: b256, // TODO: change to vector when implemented
    nonce: u64
}

pub struct TransferEvent {
    to: Sender, 
    asset: ContractId, 
    value: u64,
    nonce: u64
}
