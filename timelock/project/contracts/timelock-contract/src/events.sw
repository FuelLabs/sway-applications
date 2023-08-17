library;

use ::data_structures::Asset;
use std::bytes::Bytes;

/// Event for when a transaction is cancelled.
pub struct CancelEvent {
    /// The id of the transaction that was cancelled.
    id: b256,
}

/// Event for when a transaction is executed.
pub struct ExecuteEvent {
    /// The asset that was transferred.
    asset: Option<Asset>,
    /// Assosciated payload of the transaction.
    data: Option<Bytes>,
    /// The id of the transaction that was executed.
    id: b256,
    /// The recipient of the transaction.
    recipient: Identity,
    /// The timestamp of the transaction.
    timestamp: u64,
}

/// Event for when a transaction is queued.
pub struct QueueEvent {
    /// The asset to be transferred.
    asset: Option<Asset>,
    /// Assosciated payload of the transaction.
    data: Option<Bytes>,
    /// The id of the transaction that was queued.
    id: b256,
    /// The recipient of the transaction.
    recipient: Identity,
    /// The timestamp of the transaction.
    timestamp: u64,
}
