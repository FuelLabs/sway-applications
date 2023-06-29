library;

use ::data_structures::Asset;
use std::bytes::Bytes;

pub struct CancelEvent {
    id: b256,
}

pub struct ExecuteEvent {
    asset: Option<Asset>,
    data: Option<Bytes>,
    id: b256,
    recipient: Identity,
    timestamp: u64,
}

pub struct QueueEvent {
    asset: Option<Asset>,
    data: Option<Bytes>,
    id: b256,
    recipient: Identity,
    timestamp: u64,
}
