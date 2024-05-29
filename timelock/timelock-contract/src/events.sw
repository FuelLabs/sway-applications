library;

use ::data_structures::Asset;
use std::bytes::Bytes;

/// Event for when a transaction is cancelled.
pub struct CancelEvent {
    /// The id of the transaction that was cancelled.
    pub id: b256,
}

/// Event for when a transaction is executed.
pub struct ExecuteEvent {
    /// The asset that was transferred.
    pub asset: Option<Asset>,
    /// Associated payload of the transaction.
    pub data: Option<Bytes>,
    /// The id of the transaction that was executed.
    pub id: b256,
    /// The recipient of the transaction.
    pub recipient: Identity,
    /// The timestamp of the transaction.
    pub timestamp: u64,
}

/// Event for when a transaction is queued.
pub struct QueueEvent {
    /// The asset to be transferred.
    pub asset: Option<Asset>,
    /// Associated payload of the transaction.
    pub data: Option<Bytes>,
    /// The id of the transaction that was queued.
    pub id: b256,
    /// The recipient of the transaction.
    pub recipient: Identity,
    /// The timestamp of the transaction.
    pub timestamp: u64,
}
