library events;

use std::bytes::Bytes;

pub struct CancelEvent {
    id: b256,
}

pub struct ExecuteEvent {}

pub struct QueueEvent {
    data: Bytes,
    recipient: Identity,
    timestamp: u64,
    value: u64,
}
