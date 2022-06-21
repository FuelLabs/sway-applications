library events;

use std::{identity::Identity, vec::Vec};

pub struct ApprovalEvent {
    approved: Identity,
    owner: Identity,
    token_id: u64,
}

pub struct BurnEvent {
    owner: Identity,
    token_id: u64,
}

pub struct MintEvent {
    owner: Identity,
    token_ids: Vec<u64>,
}

pub struct OperatorEvent {
    operator: Identity,
    owner: Identity,
}

pub struct TransferEvent {
    from: Identity,
    to: Identity,
    token_id: u64,
}
