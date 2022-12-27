library events;

use std::bytes::Bytes;

pub struct CancelEvent {
    id: b256,
}

pub struct ExecuteEvent {
    asset_id: Option<ContractId>,
    data: Bytes,
    id: b256,
    recipient: Identity,
    timestamp: u64,
    value: Option<u64>,
}

pub struct QueueEvent {
    asset_id: Option<ContractId>,
    data: Bytes,
    id: b256,
    recipient: Identity,
    timestamp: u64,
    value: Option<u64>,
}
