library events;

use std::bytes::Bytes;

pub struct CancelEvent {
    id: b256,
}

pub struct ExecuteEvent {
    data: Bytes,
    id: b256,
    recipient: Identity,
    timestamp: u64,
    value: u64,
}

pub struct QueueEvent {
    data: Bytes,
    id: b256,
    recipient: Identity,
    timestamp: u64,
    value: u64,
}
